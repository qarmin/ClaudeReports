# Verify SIMD auto-vectorization claim for candidate 002 (YBR->RGB fixed-point)

Candidate `002.YBR_FIXED_POINT`'s `RESULT.md` left one thing unconfirmed: whether the
measured wall-clock win came (also) from the fixed-point rewrite letting LLVM auto-vectorize the
per-pixel loop into packed SIMD instructions, or purely from removing the per-pixel `floorf()`
libm calls and the float arithmetic itself. This candidate applies the exact same patch (copied
from `002/patch.diff`, no source change of its own) and inspects the actual generated x86-64
assembly (`objdump -d -C` on release builds with `debug = true`) of
`convert_colorspace_u8`/`convert_colorspace_u16` to answer that directly, plus re-runs the timing
and correctness checks as a sanity re-check of 002's numbers on this same box.

## What was tried

No new source variant - this is a measurement/investigation candidate re-applying `002`'s exact
patch. Two build configurations were disassembled for both baseline and patched source, using the
existing `harness/bench_pipeline` (see `README.md`):

- **rayon-enabled (default features)**: `convert_colorspace_u8`/`u16`'s body is NOT inlined into
  the caller; it survives as a standalone closure, `<&dicom_pixeldata::convert_colorspace_u8::{closure#0}
  as core::ops::function::FnMut<(&mut [u8],)>>::call_mut` (`nm -C -S`), called once per 3-byte
  pixel chunk from `rayon::iter::plumbing::bridge_producer_consumer::helper` via a real `call`
  instruction inside a scalar loop (`asm/baseline_rayon_closure_u8.s`,
  `asm/patched_rayon_closure_u8.s` - the calling loop itself is in the `helper` symbol, also
  disassembled and checked, see below).
- **no-rayon (`--no-default-features --features image,ndarray,native`)**: both baseline and
  patched bodies are fully inlined into `<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options`
  as a single scalar per-pixel loop (`asm/baseline_norayon_inlined_loop_excerpt_u16.s`,
  `asm/patched_norayon_inlined_loop_excerpt_u8.s`).

Timing/correctness re-check (same harness, same synthetic `testdata/ybr_big.dcm` /
`ybr_random.dcm` files as candidate 002, `RUNS=9` paired, host load average ~2-4 on 24 cores):

| Variant | single-core (no `rayon` feature, `taskset -c 3`) p50, 200 iters | multi-core (default `rayon` feature, unpinned) p50, 200 iters |
|---|---|---|
| baseline | 17.937 ms | 6.917 ms |
| patched (002's patch, unmodified) | 9.298 ms (-48.2%, 9/9 pairs faster) | 5.902 ms (-14.7%, 9/9 pairs faster) |

## Result

**The generated assembly contains no packed SIMD instructions operating across multiple pixels,
in either the baseline (float) or the patched (fixed-point integer) version, in either build
configuration.** Specifically:

- **Baseline** (`f32` multiply + `.floor()` + `.clamp()`): scalar SSE instructions only
  (`cvtsi2ss`, `mulss`, `addss`, `subss`, `maxss`, `minss`, `cvttss2si` - all single-scalar `ss`
  forms, not the packed `ps` forms used across multiple lanes of independent pixels), plus **two
  real function-pointer calls per pixel to `floorf`** (`call *%r14` / `call *%r13` against the
  `floorf$got` entry - confirmed via `objdump -d` symbol resolution, `asm/baseline_rayon_closure_u8.s`
  line with `# 421ba8 <floorf$got>` and the no-rayon inlined loop resolving to the same `floorf`
  GOT slot). A couple of `addps`/`mulps` (packed) instructions do appear, but only to compute two
  of the three output channels' shared partial term in one 4-lane register before immediately
  extracting single lanes back out via `shufps`+`addss`/`subss` (still one pixel's 3 channels, not
  multiple pixels) - this is packing-for-instruction-count, not vectorizing-across-pixels.
- **Patched** (fixed-point `i32`/`i64` multiply-shift-clamp): plain scalar 32-bit general-purpose-
  register integer instructions only - `movzbl`, `shl`, `imul` (immediate-coefficient forms, e.g.
  `imul $0x166e98,%r14d,%r15d`), `sar`, `cmovle`/`cmovge`, `mov` byte stores. **Zero `xmm`/`ymm`
  register uses anywhere in the hot per-pixel body** (`asm/patched_rayon_closure_u8.s`,
  `asm/patched_norayon_inlined_loop_excerpt_u8.s`) - not even the incidental packed-then-extract
  pattern the baseline has.
- In the rayon build, the calling loop (`bridge_producer_consumer::helper`) invokes the
  per-pixel closure via one real `call` instruction per 3-byte chunk (`call 39a740
  <...convert_colorspace_u8::{closure#0}...call_mut>` inside a loop with a decrementing counter) -
  confirming rayon's chunking itself also blocks any possibility of the optimizer fusing multiple
  pixels' work into one vector operation across that call boundary, independent of what's inside
  the callee.
- In the no-rayon build, both baseline and patched bodies are fully inlined into one large
  function (`to_dynamic_image_with_options`) as a straight-line loop with exactly one iteration
  per pixel - LLVM's loop vectorizer did not turn either version's per-pixel body into a
  multi-pixel-per-iteration SIMD loop, despite full inlining giving it the opportunity to.

**Conclusion: the measured wall-clock win is not from SIMD auto-vectorization - there isn't
any, in either version.** It comes entirely from (a) eliminating 2-3 real `floorf()` libm calls
per pixel (each a real out-of-line function call, not something LLVM can fold into the
surrounding scalar code) and (b) replacing an `f32` multiply/add chain + `.clamp()` with a
noticeably shorter integer `imul`+`shift`+`cmov` sequence with no function calls at all. This
directly corrects candidate 002's `RESULT.md` "Why the result looks the way it does" section,
which hedged that the integer rewrite gives "a much better chance at SIMD auto-vectorization" -
that specific mechanism is now checked and ruled out; only the "avoids the libm call" mechanism
holds up under inspection.

The timing/correctness re-check reproduces 002's original numbers closely (-48.2% single-core here
vs. -46.9%/-50.8% originally; -14.7% multi-core here vs. -15.9%/-16.9% originally; same bit-exact
result on the tiled-real-content file and the same characterized 1-LSB difference on the
random-content file) - expected, since this is the exact same patch re-measured on the same box,
not a new change.

## Does it change behavior/output?

Same as candidate 002 (same patch, unmodified) - not repeated in full here, see
`002.YBR_FIXED_POINT/RESULT.md` for the full exhaustive-check numbers. Confirmed again this
session: bit-exact on `testdata/ybr_big.dcm` (sha256 identical baseline vs. patched), differs by
exactly the same characterized pattern on `testdata/ybr_random.dcm` (sha256 differs, 1-LSB-only
difference per candidate 002's exhaustive check - not re-verified byte-by-byte again this session,
only the checksum-differs fact was re-confirmed).

- **Is this a breaking change?** Same as 002: not to a public API or file format, but converted
  pixel values can differ by up to 1 LSB from the previous release's output for some inputs.
- **Project test suite:** 537 passed / 0 failed / 6 ignored, both before (baseline, clean
  checkout) and after (002's patch applied) - `cargo test --workspace --release`. Identical counts
  to candidate 002's own run.

## Why the result looks the way it does

`convert_colorspace_u8`/`u16` are called through `.chunks_mut(3)`/`.par_chunks_mut(3)` with a
closure that receives exactly one pixel (3 elements) per call - there is structurally no
multi-pixel batch for LLVM to vectorize across inside that closure/loop-body, regardless of
whether the arithmetic inside is float or integer. The rayon path additionally puts a real `call`
instruction between iterations (the parallel-iterator bridge calls the consumer closure once per
leaf chunk), which independently rules out cross-pixel fusion there. The no-rayon path inlines
fully and iterates in a tight scalar loop, but LLVM's vectorizer still did not lift it to
process, e.g., 4 pixels (12 bytes for `u8`, or use of `pmulld`-based lanes) per iteration - likely
because 3-byte/RGB-interleaved strides and the per-channel clamp-to-`[0, 255]` branching
(implemented via `cmov`, itself vectorizer-unfriendly without a `pmaxsd`/`pminsd`-based rewrite)
don't line up with a stride LLVM's SLP/loop vectorizer chose to exploit here. Candidate 002's win
is real and reproducible but is a "fewer/cheaper scalar instructions, no function calls" win, not
a SIMD win.

## What's not done / open questions

- Did not attempt to force vectorization (e.g. restructuring the loop to process 4 or 8 pixels at
  a time with explicit `u8x16`/manual SIMD, or `#[inline(always)]`+shape changes to hint LLVM's
  vectorizer) to see whether a genuinely vectorized version would be faster still than candidate
  002's scalar-integer rewrite - that would be new, separate work, not this investigation's scope
  (added to `IDEAS.md`).
- Did not disassemble the `u16` rayon-closure path specifically (only the no-rayon inlined `u16`
  loop and the rayon `u8` closure) - the `u16` rayon closure symbol
  (`<&dicom_pixeldata::convert_colorspace_u16::{closure#0} as core::ops::function::FnMut<(&mut
  [u16],)>>::call_mut`) was located via `nm` but not separately disassembled; given the `u8`
  rayon closure and both no-rayon loops (which cover both `u8` and `u16` bodies) all show the same
  scalar, no-SIMD pattern, this is treated as sufficient coverage rather than a real gap, but
  noting it for completeness.
- `objdump`/`nm` were used directly (no `cargo-asm`, not installed on this box: `cargo asm`
  reports "no such command"); a future session wanting nicer per-function assembly output should
  either install `cargo-show-asm` or keep using this `nm -C -S` + `objdump -d -C` +
  address-range-grep approach, which worked fine here.
