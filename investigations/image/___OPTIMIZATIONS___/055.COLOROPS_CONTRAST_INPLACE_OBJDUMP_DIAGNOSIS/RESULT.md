# colorops.rs: root-causing `contrast_in_place`'s fast-path regression via objdump/perf, not just nm

Follow-up to candidate 054, which confirmed `#[inline(always)]` fixes `brighten_in_place`'s
fast-path regression (a lost-inlining artifact) but does **not** fix `contrast_in_place`'s
~7-8% regression, and left the real cause unexplained (candidate 054's own "prologue cost vs.
float-heavy body" hypothesis was untested). This candidate runs the `objdump`/`perf` diagnosis
candidate 054's `RESULT.md` proposed as the next step, using the same fast-path patch (candidate
003's `to_pixel_view_mut` conversion) plus `#[inline(always)]` on `contrast_in_place` only.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidates 003/054). Benchmark:
`benches/imageops.rs`, `contrast_in_place` case added (same `iter_batched(.., BatchSize::
LargeInput)` shape as candidates 003/054, bundled into `patch.diff`). `perf` turned out to be
**usable in this session's container** (`perf_event_paranoid = -1`), unlike prior sessions in this
workspace that assumed it wasn't installed - see `projects/image.md` update.

| Variant | single-core p50 (8 pairs) | multi-core p50 (6 pairs) | notes |
|---|---|---|---|
| baseline (unpatched get_pixel/put_pixel loop) | 8.4096 ms | 8.2798 ms | |
| fast path + `#[inline(always)]` (this candidate's `patch.diff`) | 8.9857 ms (ratio 1.069, 0/8 pairs favor patch) | 8.7962 ms (ratio 1.062, 0/6 pairs favor patch) | matches candidate 054's own +7-8%, 0/12 finding on an earlier session |

Diagnosis (not a variant to measure, but the point of this candidate) used four builds of a new,
temporary, criterion-free standalone harness (`perfcheck.rs` in this folder, built in turn as
`examples/_perfcheck_055.rs` against each source variant, binary copied out, source reverted
before the next build - never coexisted with any other real-source change) to get clean `perf
stat`/`perf record`/`perf annotate` numbers without criterion's own bootstrap-resampling overhead
in the way:

- `perfcheck_baseline` (unpatched)
- `perfcheck_patched` (fast path + `#[inline(always)]`, this candidate's actual patch)
- `perfcheck_noinline` (fast path, no annotation - candidate 003's exact original patch)
- `perfcheck_chunksexact` (fast path with the `Pixel::pixels_from_channels_mut` raw-pointer-cast
  iteration replaced by safe `chunks_exact_mut`, `#[inline(always)]` kept) - a control to test an
  alias-analysis hypothesis (see below)

Full counter dump for all of these: `perf_stat_evidence.txt`. Full command transcript and every
intermediate disassembly file this session produced: available in this session's own working
notes; the two files kept in this folder (`disasm_baseline_standalone_fn.asm`,
`perf_annotate_patched_hotloop_excerpt.txt`) are the ones the finding below is built on.

## Result

**Confirmed and reproduced**: fast path + `#[inline(always)]` is ~6-7% slower than baseline,
single- and multi-core, 0/14 total paired runs favor the patch (this session) - consistent with
candidate 054's 0/12 finding. This candidate adds the root-cause diagnosis candidate 054 left
open, covering time only (no memory-relevant mechanism here).

**Root cause identified**: the regression is **not** explained by inlining context (already ruled
out by candidate 054) and **not** explained by per-call prologue cost (the `to_pixel_view_mut`/
`SampleLayout::{row_major_packed,is_normal,fits,min_length}` checks are O(1) and, confirmed via
`perf annotate`, receive ~0% of samples - candidate 054's "prologue cost" hypothesis is ruled out
here). The real mechanism, confirmed via `perf stat`/`perf annotate`/`objdump` cross-checked
against LLVM IR:

1. `perf record`+`annotate` shows the criterion binary's *actual executed* hot loop for the
   patched build is inlined directly into `<criterion::bencher::Bencher>::iter_batched<...>`
   itself (43.27% of all samples) - **not** the separate `call_mut`/`fold` symbol pair visible via
   plain `nm`/`objdump`, which is dead code for this bench's batch-size-1 configuration (0
   samples). Comparing the wrong (unexecuted) copy of the loop was a real trap here - `nm`
   presence/absence alone is not sufficient once a function is duplicated by the optimizer;
   `perf record` was needed to find the copy that actually runs.
2. `perf stat` on that real hot loop shows patched has *fewer* total instructions and *fewer*
   branches than baseline (leaner code, as static disassembly reading alone suggested), yet *more*
   cycles (lower IPC) and **~1.77x-4.6x more `L1-dcache-loads`** (1.77x in the criterion binary,
   4.6x in the criterion-free standalone harness - the standalone harness's ratio is the cleaner
   number since it has none of criterion's own fixed per-run overhead diluting the comparison).
   `L1-dcache-load-misses` is essentially unchanged between variants - the extra loads are cheap
   L1 *hits*, not cache misses.
3. `objdump`/`perf annotate` on the patched loop's disassembly shows why: baseline hoists all 5-6
   `f32` constants (`max`, `0.5`, `percent`, the two clamp bounds) into fixed `xmm0`-`xmm5`
   registers **once**, before the loop, and never touches memory for them again across the whole
   2M-pixel image (confirmed: zero `movss <mem>,%xmm` instructions inside the loop body,
   `disasm_baseline_standalone_fn.asm`). The patched fast-path loop instead re-issues a
   RIP-relative `movss`/direct-memory-operand load of these *same* constants from the `.rodata`
   constant pool on **every channel of every pixel** (`perf_annotate_patched_hotloop_excerpt.txt`
   shows this concretely, with real sample percentages landing on these very instructions,
   e.g. 6.85% of all cycles on a single `ucomiss %xmm2, %xmm1` line whose `xmm2` was *just*
   reloaded from memory two instructions earlier). This is a genuine, disassembly-and-perf
   -confirmed backend codegen-shape difference between the two loop bodies, not a guess.
4. Two controls narrow down *why* LLVM's backend makes this choice differently for the two loops:
   - `perfcheck_noinline` (fast path, no `#[inline(always)]` at all) shows the *identical*
     magnitude of extra `L1-dcache-loads` (~51.6M/iter vs. patched's ~52.4M/iter, both ~4.5x
     baseline's ~11.4M/iter) - so this is **not** an inlining-context effect (matches, and extends,
     candidate 054's own finding that inlining context doesn't explain this function).
   - `perfcheck_chunksexact` (fast path rewritten to use safe `chunks_exact_mut` instead of
     `Pixel::pixels_from_channels_mut`'s `unsafe { slice::from_raw_parts_mut(ptr as *mut P, len) }`
     raw-pointer cast) shows the *same* magnitude again (~51.7M/iter) - ruling out a plausible
     alias-analysis hypothesis (that the raw-pointer-derived mutable pixel slice gives LLVM a
     weaker `noalias` guarantee than `get_pixel`/`put_pixel`'s checked API, forcing conservative
     reloads of provably-immutable `.rodata` constants). This hypothesis looked plausible going in
     and was worth the extra control to actually check rather than assume.
   - Checked the LLVM IR (`cargo rustc --emit=llvm-ir`) for the standalone harness: the float
     constants are plain SSA immediate operands (`fdiv float %x, 2.550000e+02`) at the IR level in
     *both* variants - there is no "reload" concept visible in IR at all. This means the
     divergence is introduced entirely during **machine-code instruction selection / register
     allocation** (the backend choosing to materialize a float immediate via a constant-pool load
     inside vs. outside the loop), not by anything at the Rust source or LLVM-IR level - so there
     is no source-level rewrite this session found that avoids it (both the closure-based
     `Pixel::apply` version and the manual `chunks_exact_mut` version hit the same backend
     behavior).

The residual, unresolved layer: *why* LLVM's backend picks "rematerialize the constant every
iteration" for this specific loop shape (a single flat loop over a `chunks_exact_mut`/raw-pointer
slice with a divisad-computed trip count) but "hoist once, keep in a register" for baseline's
loop shape (a 2D row/col nested loop with per-pixel bounds checks) was not identified - that would
require reading `MachineLICM`/register-allocator remat-heuristic internals or comparing
`-mllvm -print-after-all` passes, which is beyond what this session's bounded-time budget covered.

## Does it change behavior/output?

**No - bit-exact.** `correctness_check.rs` in this folder (run as
`examples/_correctness_check_055.rs`, not part of `patch.diff`, removed before reverting) hashes
(FNV-1a) `contrast_in_place`'s output for: `Rgb<u8>` and `Rgba<u8>` plain `ImageBuffer`s, extreme
negative and extreme positive `contrast` values (exercising both clamp branches), and a `SubImage`
view (exercising the fallback per-pixel loop, since a `SubImage`'s stride is never "fully packed").
Every hash matched byte-for-byte between an unpatched-source build and this candidate's
patched-source build.

**Test suite**: `cargo test --release --lib`: 331/331 passed, identically before and after.
`cargo test --release` (full suite, every test binary): **550 passed / 1 failed, identically
before and after** - the one failure is `tests/regression.rs::resizing_with_catmul`, independently
re-confirmed this session (ran on the unpatched baseline in isolation, matching output) to already
fail on this base commit, unrelated to this candidate - the same pre-existing failure candidates
003/054 already documented (`pic-scale-safe` version-resolution drift).

**Not a breaking change** - no public signature, panic behavior, or documented numeric result
changed.

## Why the result looks the way it does

See "Result" above - this candidate's entire point was the "why". Summary: `contrast_in_place`'s
fast-path regression is a real, reproducible, LLVM-backend register-allocation/rematerialization
difference between two functionally-identical loop shapes (both compute the exact same per-channel
float arithmetic via the exact same `Pixel::apply`/`Pixel::map` mechanism under the hood), not an
inlining-context artifact (candidate 054 already ruled that out; this candidate's `noinline`
control confirms it independently), not a per-call prologue cost (confirmed O(1) and ~0% of
samples via `perf annotate`), and not a raw-pointer/alias-analysis effect (the `chunksexact`
control rules this out). The measured 6-8% slowdown is consistent in magnitude with "a few extra
cycles of load-use latency added to the critical path of a divss/addss/mulss chain, a few times per
channel, three channels per pixel, over ~2M pixels" rather than anything cache-miss-driven (L1
misses are unchanged) or instruction-count-driven (patched has *fewer* instructions).

## What's not done / open questions

- The precise LLVM backend heuristic responsible for choosing "rematerialize from constant pool
  inside the loop" for the fast-path loop shape vs. "hoist once, keep in a register" for baseline's
  loop shape was not identified - would need `-mllvm -print-after-all`/`-mllvm -debug-only=machine-licm`
  (or an equivalent recent-LLVM diagnostic) on both variants, which wasn't attempted given the
  session's bounded-time guidance once the mechanism itself (constant rematerialization, not
  inlining/prologue/aliasing) was confirmed via three independent controls.
- No fix was found or attempted beyond the diagnosis itself - this candidate's `patch.diff` is the
  same measured-regressing patch as before (fast path + `#[inline(always)]`), kept only because
  `patch.diff` is mandatory for a real-source change regardless of Impact; a reader should not take
  this patch as-is for `contrast_in_place`. A plausible, untested next step: check whether an
  explicit local `let` binding pattern, a different closure shape, or an upstream LLVM version
  changes the backend's remat decision - none of these were tried this session.
- Whether the same rematerialization pattern affects `huerotate`/`huerotate_in_place` (candidate
  003's other float-heavy function, which measured as a *win* rather than a regression) was not
  checked - if it also loses cycles to the same effect, its win might be understating what a fixed
  codegen could achieve. Added to `IDEAS.md`.
- This diagnosis technique (finding the *actually executed* copy of a duplicated function via
  `perf record`+`annotate` before trusting `nm`/`objdump` symbol presence, then reading real
  per-instruction sample percentages rather than only counting static instructions) is worth
  reusing for any future "why is this codegen slower" investigation in this workspace - recorded in
  `projects/image.md`.
