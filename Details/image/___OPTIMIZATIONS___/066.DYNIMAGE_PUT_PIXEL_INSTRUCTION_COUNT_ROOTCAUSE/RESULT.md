# Root-causing candidate 065's unexplained +16.2% dynamic-instruction increase from the `DynamicImage::put_pixel` `#[inline]` patch

Follow-up to candidate 065, which confirmed (via `perf stat`) that candidate 060/064's
`#[inline(always)]`/`#[inline]` patch on `DynamicImage::get_pixel`/`put_pixel` increases total
dynamic `instructions` retired by ~16.2% for both `vertical_gradient` and `horizontal_gradient`
equally, while simultaneously decreasing `L1-dcache-loads` ~10% and leaving static `.text` size
essentially flat - a real, reproducible, but unexplained side effect. This candidate root-causes
it via a side-by-side `objdump`/`perf stat`/LLVM-IR disassembly comparison of the patch's actual
call site and function body, the technique candidate 055 used for its own rematerialization
finding. **No new source patch was written** - this candidate reuses candidate 060/064's exact,
unmodified fix purely as the before/after toggle for this investigation, same convention
candidate 065 itself used.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidates 060/064/065). A
temporary standalone example, `examples/_perf_check_066.rs` (not part of `patch.diff` -
measurement tooling, same convention as candidates 025/037/055/065's own temporary drivers),
recreates candidate 065's own harness: takes a mode (`vertical`/`horizontal`) and an iteration
count, builds one 1920x1080 `DynamicImage::ImageRgba8` once, and calls
`imageops::vertical_gradient`/`horizontal_gradient` on it in a tight loop (200 iterations),
wrapping the mutable reference in `black_box` each call.

Built two `--release` binaries: `baseline` (unpatched source) and `patched` (candidate
060/064's `#[inline(always)]` on `get_pixel` + `#[inline]` on `put_pixel`, applied unmodified).
`perf stat -e instructions` reproduced candidate 065's own finding exactly on this harness (5
pinned repeats `taskset -c 3`, 3 unpinned repeats, <0.02% run-to-run variance, `uptime` load
average 0.37-2.6 throughout, well under the 24-core count):

| Variant | instructions (median of 5, `taskset -c 3`) | delta vs. baseline |
|---|---|---|
| baseline `vertical_gradient` | 17,866,143,672 | |
| baseline `horizontal_gradient` | 17,877,338,527 | |
| patched `vertical_gradient` | 20,768,357,023 | +16.24% |
| patched `horizontal_gradient` | 20,777,555,578 | +16.23% |

Unpinned spot-check (3 repeats each) landed within the same range (baseline 17.866-17.879B,
patched 20.770-20.782B) - not a `taskset`-pinning artifact, matching candidate 065's own
unpinned control.

**Step 1 - find the call site.** `nm --demangle` on both binaries shows `vertical_gradient`/
`horizontal_gradient` themselves fully inlined into `_perf_check_066::main` in both variants (no
standalone symbol for either), but `<DynamicImage as GenericImage>::put_pixel` **does** survive as
its own standalone symbol in both binaries, called via a real `call` instruction from inside the
main loop in both baseline and patched - **`put_pixel` itself is not inlined into the hot loop in
either variant**, ruling out "the 8-arm `match` gets inlined directly into the loop" (065's own
leading hypothesis) as the mechanism outright. The only visible difference at the call site itself
is *how* the call is made: baseline calls through a GOT-indirect load (`mov r15, [rip+...
put_pixel$got]` then `call r15`, consistent with `put_pixel` having ordinary external linkage in
the library rlib), while patched calls directly to a local symbol (`call 14e70 <...put_pixel>`,
consistent with `put_pixel` being compiled as a private copy inside the *consuming* crate/example
itself once `#[inline]` makes it eligible for cross-crate MIR-based codegen instead of always
being compiled with normal external linkage in the defining crate). This by itself costs nothing
extra per call (one `call` instruction either way).

**Step 2 - isolate which attribute causes it.** A control build with **only** `#[inline]` on
`put_pixel` (not touching `get_pixel` at all, since `get_pixel` is never called by
`vertical_gradient`/`horizontal_gradient`) reproduces the *exact* same instruction count as the
full patch (20,767,007,720 vertical / 20,777,383,457 horizontal - matches within noise), and its
`objdump` disassembly of `put_pixel` is **byte-for-byte identical** to the full patch's. This
isolates the effect entirely to `#[inline]` on `put_pixel`; `get_pixel`'s `#[inline(always)]` has
zero effect on this workload (never called by either gradient function), consistent with 065's own
report that only `put_pixel`'s attribute could plausibly be relevant here.

**Step 3 - read `put_pixel`'s own disassembly.** Since the call site itself is unchanged in shape,
the difference must be inside `put_pixel`'s own function body. `objdump` on the `ImageRgba8` match
arm (the one arm actually exercised by this test - `img` is always `DynamicImage::ImageRgba8`,
selected purely as a direct byte-copy path with no color conversion) shows:

- **Baseline** (`disasm_baseline_rgba8_arm.asm`): after the two bounds checks and index
  computation (15 identical instructions in both variants), stores the pixel with a single
  instruction: `mov DWORD PTR [rdx+rax*1],ecx` - directly reusing the still-live, already-packed
  32-bit pixel argument register.
- **Patched** (`disasm_patched_rgba8_arm.asm`): same 15 setup instructions, but before the store,
  **rebuilds** the 32-bit pixel value from four separately-unpacked byte fields the function had
  earlier split out of the same argument (used by the other 11 match arms' color-conversion math):
  `shl r8d,0x8` / `movzx esi,r9b` / `movzx edi,r8w` / `or edi,esi` / `shl edi,0x8` /
  `and ecx,0xff0000ff` / `or ecx,edi` - **7 extra instructions** - before the identical
  `mov DWORD PTR [rdx+rax*1],ecx` store.

**Step 4 - confirm the LLVM IR is identical; the divergence is backend-only.** Extracted
`put_pixel`'s full LLVM IR (`cargo rustc --lib -- --emit=llvm-ir` for baseline; the equivalent
per-crate copy from the example's own `--emit=llvm-ir` output for patched, since `#[inline]`
routes its actual codegen into the *consuming* crate rather than the library). The IR-level
"reconstruct from decomposed bytes" instruction sequence (`pixel.sroa.13.sroa.13.0.insert.ext`,
etc.) for the `ImageRgba8` arm is **already present in baseline's own IR too** - both variants'
LLVM IR compute the store value the same "reconstruct from parts" way at the IR level; baseline's
*backend* folds this chain back down to a no-op (recognizing it's mathematically identical to the
original packed argument) while patched's backend does not. A normalized `diff` of the two
variants' full 908-line `put_pixel` IR bodies (stripping crate-hash-derived symbol suffixes and
`!metadata` node numbers, both artifacts of the two variants being compiled in different LLVM
modules) shows **zero opcode differences** anywhere in the function - the only real differences are
the function's own linkage/calling-convention/attributes (`llvm_ir_function_header_diff.txt`):

```
baseline: define void @...put_pixel(...) unnamed_addr #0
          attributes #0 = { nonlazybind uwtable ... }              (external linkage, ordinary ccc)
patched:  define internal fastcc void @...put_pixel(...) unnamed_addr #3
          attributes #3 = { inlinehint nonlazybind uwtable ... }   (internal linkage, fastcc, +inlinehint)
```

**Step 5 - quantitative cross-check.** The measured total instruction delta (200 iterations x
1,920x1,080 pixels = 414,720,000 `put_pixel` calls per run) is 20,768,357,023 - 17,866,143,672 =
2,902,213,351 instructions, i.e. **6.997 instructions per call** - matching the 7-instruction
reconstruction sequence found by reading the disassembly almost exactly, and explaining why the
percentage is identical for `vertical_gradient` and `horizontal_gradient`: both loops call the
exact same `put_pixel` function the exact same number of times per pixel, so an effect that lives
entirely inside `put_pixel`'s own body (not in either loop's own structure) necessarily produces
the same per-call, and therefore the same overall percentage, delta regardless of which loop calls
it.

## Result

**Root cause identified, to the same depth candidate 055 reached for its own similar finding**:
the +16.2% dynamic-instruction increase is **not** a different unroll factor and **not** a
vectorization-shape change (both hypotheses 065 raised are ruled out - `put_pixel` is not inlined
into either loop in either variant, and neither variant's hot loop or `put_pixel`'s own body uses
any SIMD instructions at all for this pixel format). It is **"something else"**, the third option
065's own open question named: a **backend register-allocation/instruction-selection difference
inside `put_pixel`'s own function body**, triggered by `#[inline]` changing the function from
"always compiled with ordinary external linkage in the defining library crate" (baseline) to
"compiled as a private, `internal fastcc`-linked copy inside each consuming crate, from exported
MIR" (patched) - a linkage/calling-convention/`inlinehint`-attribute change that leaves the
LLVM IR's actual instructions byte-for-byte identical, but causes the x86-64 backend to lose track
of the fact that the `ImageRgba8` arm's rebuild-from-parts store value is mathematically identical
to the original argument register, adding 7 extra instructions to the one match arm actually
exercised by this workload - reproduced consistently across all 8 pinned + 6 unpinned runs
(<0.02% variance) and confirmed quantitatively (7.00 measured vs. 7 instructions read directly off
the disassembly).

This does not question candidate 060/064's own wall-clock win for `vertical_gradient` (the extra
7 instructions/call are cheap, register-only ALU ops with no memory access - consistent with
065's own finding that `L1-dcache-loads` *decreased* from the same patch) - it only explains the
previously-unexplained instruction-count side effect, which candidate 065 had already noted does
not itself change the patch's wall-clock conclusion.

## Does it change behavior/output?

**N/A - measurement/investigation only, no new patch applied to real source.** This candidate
reuses candidate 060/064's already bit-exact-verified `#[inline(always)]`/`#[inline]` fix
unchanged, purely as the toggle for this investigation; `patch.diff` in this folder is a copy of
candidate 064/065's, kept for reproducibility (same convention candidates 037/065 used), not a new
change. `examples/_perf_check_066.rs` was temporary measurement tooling, removed before the
working tree was reverted, and was never part of `patch.diff`.

- **Bit-exact?** N/A (no new patch); candidates 060/064 already confirmed bit-exact output for
  this exact fix.
- **Is this a breaking change?** No - same as candidates 060/064/065, only `#[inline]` attributes,
  no signature/behavior change.
- Project test suite (`cargo test --release`), re-run directly in this candidate's own session:
  **550 passed / 1 failed, identically before and after** applying/reverting the same patch. The
  one failure is the same pre-existing, unrelated `tests/regression.rs::resizing_with_catmul`
  every prior candidate in this workspace touching this area has already documented.

## Why the result looks the way it does

See "Result" above - this candidate's entire point was the "why" 065 left open. In short: adding
`#[inline]` to a function doesn't just change *whether* callers inline it - for a function that
ends up NOT actually inlined at its call site either way (as here), it can still change *where and
how* that function's own body gets compiled (moved from the defining crate to each consuming
crate, with different linkage/calling convention), and that alone is enough to change the backend's
final instruction selection for functionally/IR-identical code. This is the same general shape as
candidate 055's finding (an LLVM backend codegen difference for textually-identical-at-the-IR-level
code, driven by compilation context rather than by any front-end/source-level difference) but a
different concrete mechanism: 055's was loop-shape-driven register rematerialization inside one
compilation unit, this one is calling-convention/linkage-driven register allocation triggered by
which compilation unit the function ends up compiled in at all.

## What's not done / open questions

- Which of the three simultaneous IR-level changes (`external` -> `internal` linkage, `ccc` ->
  `fastcc` calling convention, or the added `inlinehint` function attribute) is actually
  responsible for the backend's different register-allocation choice was not isolated further -
  all three are simultaneous, inseparable consequences of adding `#[inline]` to a Rust function via
  the normal compilation pipeline used here, and disentangling them would need hand-editing the
  extracted LLVM IR (forcing just one attribute at a time) and feeding it through `llc` directly,
  bypassing rustc entirely - not attempted in this candidate's bounded scope, same kind of residual
  "which exact backend heuristic" gap candidate 055 left open for its own finding.
- Only the `ImageRgba8` arm (the one this harness exercises) was checked byte-for-byte; the other
  11 match arms in `put_pixel` were not individually compared, so it isn't confirmed whether the
  same "reconstruct instead of reuse" pattern also costs extra instructions for `Luma8`/`Rgb8`/
  16-bit/32-bit-float variants, or whether it's specific to the direct-copy `ImageRgba8` arm having
  the least own arithmetic to hide the reconstruction cost behind.
- This candidate did not check whether the same linkage/calling-convention effect (and its
  instruction-count cost) reproduces for `SubImageInner::get_pixel`/`put_pixel` (candidate 061) or
  any other `#[inline]`-annotated function in this crate that ends up not actually inlined at its
  call site - worth a quick spot-check if a future candidate revisits any of those.
