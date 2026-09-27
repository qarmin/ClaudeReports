<!-- rustc/cargo used throughout: rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0 (797e8a9bc 2026-08-05) -->

# Confirming what `f32::round()`'s libcall actually resolves to (candidate 015's hypothesis, checked)

Candidate `015`'s disassembly-grounded explanation for why the `sse4.1`-gated manual fallback
(candidate `010`'s patch) *regresses* `hayro-syntax::filter::jpx::scale()`'s call site was: "glibc's
`roundf` itself very plausibly dispatches to a hardware `ROUNDSS`-based implementation via its own
runtime IFUNC CPU-feature resolution, independent of what the calling binary was compiled with" -
explicitly flagged there as "a plausible, disassembly-grounded hypothesis, not a fully proven one."
This candidate steps into the actual call with `gdb` (and cross-checks with `objdump`/`nm`/`ldd`/
`LD_DEBUG=bindings`) to find out what symbol it really is, on the same machine (AMD Ryzen 9 7900,
`x86_64-unknown-linux-gnu`, default target features - no `sse4.1` at compile time) both `010` and
`015` used. **The hypothesis was wrong** - not for the reason expected. It then re-runs candidate
`010`'s own bench end-to-end (apply/build/measure/revert, both single- and multi-core, with the
project's test suite before and after) to check whether this correction changes `010`'s
already-measured win.

## What was tried

**Part 1 - symbol identity.** Built a minimal standalone Rust probe (`fn round_it(x: f32) -> f32 {
x.round() }`, `#[inline(never)]`, release profile, default `x86_64-unknown-linux-gnu` target - same
as this workspace's own default build) and separately used the real `hayro-bench` `jpx_decode_bench`
binary (unmodified `hayro-jpeg2000::math::round_f32`, `std` feature on). For both:

- `objdump -d --demangle` on the compiled binary: the call site is `jmp *GOT_SLOT # roundf$got`,
  landing on a **locally-defined** (`t`, not `T`) symbol named `roundf` inside the binary itself,
  not a PLT stub into any shared library.
- `readelf -sW` / `nm`: that local `roundf` symbol is `FUNC LOCAL HIDDEN`, i.e. statically linked
  into the binary, not imported.
- `ldd`: the binary links only `libgcc_s.so.1`, `libc.so.6`, `ld-linux-x86-64.so.2` - **no
  `libm.so.6`** is linked at all.
- `nm -D` / `objdump -T` on this system's actual `/usr/lib/x86_64-linux-gnu/libc.so.6` (glibc
  2.43-2ubuntu2.3, merged-libm build): it does **not** export `roundf` (or `floor`, `sin`, `cos`,
  ...) as a public dynamic symbol at all on this system - those names only exist as dynamic exports
  in the separate compatibility shim `/usr/lib/x86_64-linux-gnu/libm.so.6` (`roundf@@GLIBC_2.2.5`,
  `WEAK`), which the Rust binary never links against in the first place.
- `LD_DEBUG=bindings ./round_probe` (dynamic-linker binding trace): no `roundf`/libm binding event
  ever appears - consistent with the call never leaving the binary.
- `gdb -batch -ex "break roundf" -ex run -ex bt -ex disassemble` on both the probe and the real
  `jpx_decode_bench` binary: the breakpoint resolves with full DWARF source info to
  `compiler_builtins::math::full_availability::roundf::roundf`
  (`library/compiler-builtins/compiler-builtins/src/math/generic/round.rs`, via
  `compiler_builtins::math::libm_math::generic::copysign::copysign::<f32>`), i.e. **Rust's own
  `compiler_builtins` crate's software (bit-manipulation) `libm` implementation, statically linked
  into every Rust binary** - not glibc, and not any IFUNC-resolved hardware path. Disassembly of
  this function: `andps`/`orps`/`addss` to compute `x + copysign(0.5, x)`, then exponent-based
  bit-masking to truncate - a `cmp`/`cmov`/`and` sequence, no `roundss` instruction anywhere.
  `objdump -d | grep -c roundss` over the whole binary: **0** matches, confirming the hardware
  `ROUNDSS` instruction is never emitted or reached, by either variant, anywhere in this binary.
- The two disassemblies (standalone probe vs. real `jpx_decode_bench`) are instruction-for-instruction
  identical (same `movaps`/`andps`/`orps`/`addss`/`shr`/`cmp`/`cmovae`/`and` sequence, only the
  absolute addresses differ) - the finding transfers directly to the code candidates `010`/`015`
  actually measured, not just the synthetic probe.

**Part 2 - re-verify candidate `010`'s win with the corrected mechanism in mind.** Same bench/inputs/
methodology as `010`: `hayro-bench`'s `jpx_decode_bench`, `hayro-tests/pdfs/load/issue391.pdf`,
`taskset -c 2` single-core / unpinned multi-core, 200 iterations/round, in a fresh git worktree
(`git worktree add -d /tmp/hayro-worktree-039 5a5f0e24`), every heavy command through
`___OPTIMIZATIONS___/.locks/build.lock`. `uptime` load 11-12 throughout (below the 24-core count,
per MEASUREMENT.md's re-baseline threshold, but visibly noisier than `010`'s own quiet-host run -
see "Why the result looks the way it does"). 3 rounds per variant per mode (vs. `010`'s original
2x5/2x3):

| Variant | single-core p50 (3 rounds, ms/iter) | multi-core p50 (3 rounds, ms/iter) | notes |
|---|---|---|---|
| baseline (`x.round()` unconditional) | 16.937 (16.937, 15.574, 17.241) | 15.978 (17.194, 15.683, 15.978) | |
| patched (candidate 010's `sse4.1`/`neon` gate) | 16.116 (16.029, 16.578, 16.116) | 15.448 (15.268, 15.448, 15.556) | |

## Result

**Part 1 (dispatch identity): confirmed, and it overturns candidate `015`'s specific mechanism.**
The baseline `x.round()` call - in both the isolated probe and the real `hayro-jpeg2000`/
`hayro-syntax` code candidates `010`/`015` measured - resolves at link time to a statically-linked
copy of `compiler_builtins`'s own software `roundf`, not to glibc's `roundf`, and never to a
hardware `ROUNDSS` instruction via any IFUNC resolution (this glibc build's `libc.so.6` doesn't
even export `roundf` as a dynamic symbol, and the binary doesn't link `libm.so.6` where it does
live). Every variant of "the baseline is fast because it reaches glibc's IFUNC-optimized hardware
path" in `015`'s writeup is not what happens on this machine/toolchain.

**Part 2 (re-check of `010`'s win): direction unchanged, magnitude noisier this session, not
invalidated.** Multi-core: every one of the 3 patched rounds (15.268/15.448/15.556) beats every
one of the 3 baseline rounds (15.683/15.978/17.194) - a clean separation, median delta -3.3%,
consistent in both direction and rough magnitude with `010`'s original -2.2%. Single-core: median
delta -4.8% (16.116 vs. 16.937), same direction as `010`'s original -3.2%, but this session's
3-round spread partially overlaps (baseline's low outlier of 15.574 undercuts 2 of 3 patched
rounds) - noisier than `010`'s clean 5-vs-5/3-vs-3 separation on a quieter host. Nothing here
points the other way (no round of either mode showed patched slower than baseline's own median).
This is a wall-clock-only re-check; no memory/allocation dimension was measured (none is expected
to differ - the patch replaces one scalar computation with another, no allocation shape change).

## Does it change behavior/output?

- **Bit-exact?** yes, for the re-verification in Part 2: `cargo test -p hayro-jpeg2000 -p
  hayro-tests --release --test tests -- stream_jpx` produced **identical** results before and
  after the patch - `1 passed; 3 failed` both times, with identical pixel-diff counts
  (`stream_jpx_3`: 65974, `stream_jpx_5`: 91173, `stream_jpx_6`: 29580) - matching `010`'s and
  `015`'s own prior runs exactly (same pre-existing, unrelated failures; confirmed reproducible
  in a completely fresh worktree by copying `hayro-tests/snapshots/` from the main checkout,
  since those reference images are git-ignored and not present in a bare `git worktree add`).
- **Is this a breaking change?** no - `round_f32` is `pub(crate)`; this candidate applies no new
  patch of its own (Part 1 is measurement-only; Part 2 reapplies `010`'s already-reviewed patch
  verbatim, unmodified).
- Project test suite: see above - `1 passed, 3 failed` before and after, no new/different
  failures caused by this candidate's work.

## Why the result looks the way it does

`f32::round()` lowers via LLVM's `llvm.round.f32` intrinsic. When the target has no hardware
lowering for it (no `sse4.1`/`neon`, this machine's default `x86_64-unknown-linux-gnu` build), LLVM
legalizes it to a **libcall** named `roundf`. What actually satisfies that libcall symbol at link
time depends on what's available: on this glibc build, the public libc no longer exports `roundf`
as a dynamic symbol (only the separate compat `libm.so.6` does, and the Rust binary never links
it), so the weak-linkage `roundf` that Rust's own `compiler_builtins` crate ships (as
`compiler_builtins::math::full_availability::roundf`, a portable, `libm`-derived bit-manipulation
implementation meant to work on any target regardless of system libm availability) is what actually
gets pulled in and statically linked - with nothing else present to out-compete it. This is an
LLVM-lowering-plus-linker-resolution fact independent of the CPU's actual capability (this Ryzen 9
7900 *does* have real `sse4.1` hardware, per `/proc/cpuinfo`; it's simply never used because the
compiled code was never told it could) - `015`'s core observation, "the compiled binary's own
capability and what actually executes at runtime can differ," was directionally right, but the
mechanism is "Rust statically links its own software fallback because the system libm doesn't
export the symbol this build needs," not "glibc's own `roundf` internally IFUNC-dispatches to
hardware." Both explanations predict the same shape of result (a real, non-inlined, non-vectorized
`call` in the baseline) - which is why `010`'s and `015`'s original *measurements* don't need to
change, only the story of *why* the baseline's call has the cost profile it does. `010`'s win
(bulk `interleave_and_convert` loop, no per-element side effects) and `015`'s regression
(`scale()`'s serial `BitWriter`-coupled loop) both still trace to the same real property: whether
the surrounding loop lets LLVM auto-vectorize the manual replacement, not to which library
provides the libcall being replaced.

## What's not done / open questions

- Not checked whether a target actually compiled with `+sse4.1` (or `-C target-cpu=native` on this
  AVX-512-capable machine) changes which lowering LLVM picks for `f32::round()` (a real hardware
  `roundss`/`vroundss` inline instruction instead of any libcall at all) - out of scope here, this
  candidate is specifically about what the *default* `x86_64-unknown-linux-gnu` build (what `010`/
  `015`/this workspace's own bench builds all use) does. Still an open item from `015`, unchanged.
  candidate `010`'s own `sse4.1`/`neon` `cfg` gate would already pick the hardware path there if
  it were ever built that way - just never exercised/measured under that config in this workspace.
- The multi-core/single-core split in Part 2's numbers (clean separation multi-core, noisier
  single-core) is plausibly explained by this session's own host load (`uptime` 11-12, vs. `010`'s
  quiet-host run) rather than anything about the patch itself - not independently isolated further
  (e.g. re-run on a quiet host) since the direction is already consistent with `010`'s original
  finding and re-litigating `010`'s magnitude precisely wasn't this candidate's goal.
- Whether other Rust toolchain versions/distros (a glibc build that *does* export `roundf`
  dynamically, or a `musl` target, or a statically-linked-libc build) would resolve this libcall
  differently (genuinely reaching glibc's own `roundf`, IFUNC or not) was not checked - this
  candidate's finding is specific to this session's environment (Ubuntu's glibc 2.43-2ubuntu2.3,
  rustc 1.98.0), not a claim about every possible Linux/Rust combination.
