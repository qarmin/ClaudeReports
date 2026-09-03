# RESIZE_TO_FILL_INTERMEDIATE_OOM

`DynamicImage::resize_to_fill` (`src/images/dynimage.rs`) computes an intermediate "fill" size via
`resize_dimensions(..., fill=true)` (`src/math/utils.rs`), whose ratio is `max(wratio, hratio)`,
then resizes the *whole source* to that intermediate size before cropping down to the requested
`nwidth x nheight`. For a small, valid source image with an extreme aspect ratio, that intermediate
size can be orders of magnitude larger than the target, even though the crop immediately after
discards almost all of it. Picked from `IDEAS.md`'s backlog (originally sourced from upstream
image-rs/image issue #3083, still open, labeled `kind: slow` / `kind: oom`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + arithmetic check of `resize_dimensions` | Worked out `resize_dimensions(1, 1_000_000, 100, 100, fill=true)` by hand: `wratio=100`, `hratio=0.0001`, `ratio=max=100` -> intermediate size `(100, 100_000_000)` = 10^10 pixels | Confirmed the arithmetic blow-up exists purely from reading the code, before running anything |
| Standalone Rust probe (`___BUGHUNT___/_reference/resize_fill_probe`, path-dependency on the checkout, release build) calling `DynamicImage::resize_to_fill(100, 100, Triangle)` on a real `RgbaImage::new(1, 1_000_000)` (4MB), run under `ulimit -v 4000000` (4GB cap) to make the allocation failure deterministic and safe instead of needing dozens of real GB of RAM | `cargo build --release` + `ulimit -v 4000000; ./target/release/resize_fill_probe`, run 3x | Reproduced 3/3 runs, identical: `memory allocation of 40000000000 bytes failed`, process aborts (SIGABRT, exit 134) |
| Differential correctness probe (same scratch folder, `src/bin/diff.rs`) comparing the old resize-then-crop algorithm against the drafted crop-then-resize fix on 6 ordinary (non-extreme) resize cases with smooth-gradient source images | `cargo run --release --bin diff`, release build | All 6 cases matched dimensions exactly; pixel values differed by at most +/-1/255 per channel (sub-pixel rounding only) - confirms the fix doesn't change behavior for normal inputs |

Only 2 distinct tools/approaches were needed (both under the soft cap of 3) - the second one
(the standalone probe under a memory cap) reproduced deterministically on the first try, and the
third one listed above was used to validate the fix's correctness, not to dig further into the bug
itself.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs against the pristine (unpatched) code, same
build, same input, same exact "memory allocation of 40000000000 bytes failed" message and abort
every time - this is a pure arithmetic consequence of the input's dimensions (no timing/memory-
layout dependence), so it is expected to reproduce 100% of the time on any build/platform where a
40GB allocation exceeds available (virtual) memory.

**Fully reproducible**: Yes. `repro/` calls the real, unmodified public entry point named in the
candidate (`DynamicImage::resize_to_fill`) with a source image built entirely through the crate's
own public API (`RgbaImage::new`) - no internal test scaffolding, no hand-constructed internal
state standing in for what a caller would produce. This is not a "two participants" scenario (no
peer/wire/upstream-caller boundary is involved - it's a single caller calling a single public
function with an ordinary, self-constructed image), so `CONFIRMATION.md` section 11's second-
participant requirement does not apply here; the repro already runs the real, complete, unmodified
trigger path end-to-end.

## Class

`crash` (abrupt process termination via an unhandled allocation failure - not memory-safety UB,
but an availability/OOM crash, matching upstream's own `kind: oom` / `kind: slow` labels).

## Root cause

`resize_to_fill` (pre-fix) does, in order:

1. `resize_dimensions(width, height, nwidth, nheight, fill=true)` picks `ratio = max(wratio,
   hratio)` - i.e., whichever axis needs to grow *more* to cover the target on both axes. For a
   source with an extreme aspect ratio relative to the target (e.g. 1x1,000,000 source, 100x100
   target), one axis's required ratio (100 for width) is ~1,000,000x larger than what the other
   axis would have needed (0.0001 for height) - but `fill`'s definition requires using the larger
   one so that *both* target dimensions end up covered before cropping.
2. `self.resize_exact(width2, height2, filter)` then actually allocates and resizes to that full
   `(100, 100_000_000)` intermediate size - **10 billion pixels**, ~40GB as RGBA8 - even though
   the very next step immediately crops it down to `100x100` (10,000 pixels), discarding
   999,999,999,900 of the 10,000,000,000 computed/allocated pixels.
3. The crop happens only *after* that allocation and full resize already completed (or, in
   practice, after the allocation already failed and aborted the process).

The bug is structural, not a missing bounds check: the algorithm resizes the *entire* source to
fill-cover the target on the "wide" axis, uniformly across the *whole* other axis too, then throws
almost all of it away - there was never a need to compute more than a `nwidth x nheight`-sized
*region* of that fill-resize in the first place, since the final crop only ever keeps a window of
exactly that size.

## Introduced by

46bf6b8 "Add a resize_to_fill method" (2017-12-08)

## The fix

Restructured `resize_to_fill` to crop the *source* image first (in source coordinates, computed by
mapping the existing crop-rectangle-selection logic in the `width2 x height2` intermediate space
back through the same scale factor), and only then call `resize_exact(nwidth, nheight, filter)`
directly to the final target size - instead of resizing to the full `width2 x height2` intermediate
size and cropping afterward. This makes every allocation in the function bounded by
`max(source size, nwidth x nheight)`, never by the disproportionate intermediate `width2 x height2`
fill size.

The crop-rectangle math itself (`select`, using `resize_dimensions`'s `width2`/`height2` and the
existing `ratio`/`nratio` comparison to decide which axis to crop) is unchanged - only *when* the
crop and resize happen relative to each other changed (crop-then-resize instead of
resize-then-crop), which is mathematically equivalent up to sub-pixel rounding (see the
differential probe results above: max +/-1/255 per channel on 6 ordinary cases, 0 dimension
mismatches).

- Before (pristine): repro aborts with `memory allocation of 40000000000 bytes failed` (SIGABRT,
  exit 134) under a 4GB virtual-memory cap, 3/3 runs.
- After (patched): repro completes normally, prints `did not reproduce - result dims: (100, 100)`,
  exit 0, under the same 4GB cap.
- Project test suite (`cargo test --lib --release`): **331 passed; 0 failed** before the fix, and
  **331 passed; 0 failed** after the fix (identical pass count, no regressions, no new failures).

`patch.diff` contains this fix, applied against `src/images/dynimage.rs` at commit
`44ce9226c541dd2b11897b9ef07156a28871cdb7`.

## What's not done / open questions

- `resize_dimensions` itself (`src/math/utils.rs`) is unchanged - it is still used elsewhere
  (`resize`, `thumbnail`) where `fill=false` is used and this specific blow-up doesn't apply (the
  non-fill `min(wratio, hratio)` path never overflows the target this way). Only `resize_to_fill`'s
  *use* of the fill-mode result was restructured; `resize_dimensions`'s own public(crate) contract
  and its existing unit tests (`resize_handles_fill`, etc.) are untouched and still pass.
- The fix was validated with 6 hand-picked (width, height, nwidth, nheight) combinations for
  correctness-preservation (smooth-gradient differential check) and with the exact extreme-aspect
  case from the candidate for the OOM fix itself - not with a broader randomized/fuzz sweep across
  many aspect ratios and rounding edge cases. A future session could run a larger randomized
  differential comparison (old vs. new algorithm, checking both dimension-exactness and pixel
  closeness) if broader confidence is wanted before this fix is proposed upstream.
- Did not check whether any *other* public resize entry point in this crate has the same
  "compute a disproportionately large intermediate buffer that a later step then discards" shape -
  this candidate was scoped narrowly to `resize_to_fill` per the backlog item. Worth a grep sweep
  for other resize-then-crop or resize-then-discard patterns if this bug class is suspected
  elsewhere.
- Not verified against a real, unbounded-memory 40GB allocation (i.e., without `ulimit -v`) - doing
  so on this session's own machine (29GB total RAM) would have risked exhausting real system
  memory. The `ulimit -v` cap is a standard, deterministic way to demonstrate "this code path
  attempts an allocation far larger than the input/output justify" without needing that much real
  RAM; the exact requested byte count (40,000,000,000, printed by Rust's own allocator-failure
  message) is real, actual evidence of the attempted allocation size, not simulated.
