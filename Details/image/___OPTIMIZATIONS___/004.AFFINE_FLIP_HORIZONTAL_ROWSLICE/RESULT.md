# flip_horizontal_in / flip_horizontal_in_place: per-row pixel-slice fast path

`src/imageops/affine.rs`'s `flip_horizontal_in` and `flip_horizontal_in_place` iterate every pixel
via the generic `GenericImageView::get_pixel`/`GenericImage::put_pixel` trait methods, even though
horizontal flip is a per-row pixel-order reversal. This is the direct sibling of candidate 001
(`flip_vertical_in`/`rotate180_in_place`), extended to the row-reversal shape: both new fast paths
are gated on the same existing `to_pixel_view()`/`to_pixel_view_mut()` mechanism, firing only when
the buffer exposes a provably packed row (`width_stride == pix_stride`), and falling back to the
original per-pixel loop otherwise.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: `benches/imageops.rs`'s
existing `flip_horizontal` case (exercises `flip_horizontal_in` via the public
`flip_horizontal()` wrapper, 1920x1080 `RgbImage`) plus a new `flip_horizontal_in_place` case added
to the same file (see `patch.diff`; no prior bench exercised `flip_horizontal_in_place` directly -
same convention candidate 001 used for `rotate180_in_place`, `b.iter_batched(.., LargeInput)` to
keep the per-iteration clone out of the timed portion).

All numbers below are from alternating A/B runs of two prebuilt `--release` criterion binaries
(baseline vs. patched), `--warm-up-time 1 --measurement-time 2`, seconds apart on the same
machine, per MEASUREMENT.md's paired-comparison discipline (this machine's load varied 5.9-11.6
during the session; see `README.md`'s note on why single-binary repeated runs aren't trusted
here).

**`flip_horizontal` (-> `flip_horizontal_in`), single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 1.947 ms | 3.786 ms | 3.786 ms | 3.612 ms | 3.615 ms |
| patched | 1.506 ms | 1.531 ms | 1.605 ms | 1.714 ms | 1.583 ms |

Patched faster in 5/5 pairs. Median baseline 3.612 ms, median patched 1.583 ms (~2.3x faster).

**`flip_horizontal`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 2.332 ms | 1.839 ms | 1.954 ms | 1.915 ms | 1.924 ms |
| patched | 0.751 ms | 0.716 ms | 0.748 ms | 0.687 ms | 0.716 ms |

Patched faster in 5/5 pairs. Median baseline 1.924 ms, median patched 0.716 ms (~2.7x faster).

**`flip_horizontal_in_place`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 2.793 ms | 2.589 ms | 2.736 ms | 2.419 ms | 2.985 ms |
| patched | 1.737 ms | 1.591 ms | 1.831 ms | 1.699 ms | 2.138 ms |

Patched faster in 5/5 pairs. Median baseline 2.736 ms, median patched 1.737 ms (~1.6x faster).

**`flip_horizontal_in_place`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 1.546 ms | 1.637 ms | 1.603 ms | 1.658 ms | 1.619 ms |
| patched | 1.055 ms | 1.046 ms | 1.069 ms | 1.032 ms | 1.059 ms |

Patched faster in 5/5 pairs. Median baseline 1.619 ms, median patched 1.055 ms (~1.5x faster).

## Result

Both functions show a real, consistent win across all 20 paired runs (5 single-core + 5
multi-core, for each of the two functions - 5/5 in every group). `flip_horizontal_in` (out-of-place,
two independently generic type parameters, matching `flip_vertical_in`'s shape from candidate 001)
is the larger win, ~2.3-2.7x faster. `flip_horizontal_in_place` (in-place, single monomorphized
type, matching `rotate180_in_place`'s shape) is a smaller but still solid win, ~1.5-1.6x faster -
in between the two effect sizes candidate 001 measured for its own out-of-place/in-place pair
(~7.5-9x vs. ~11-27%), consistent with the same "single generic in-place type already gets closer
to memcpy-speed under LLVM" explanation candidate 001 proposed, though `flip_horizontal_in_place`
still keeps a real per-row-reversal cost (`.reverse()` on a pixel slice, not a straight
`copy_from_slice`) that `rotate180_in_place`'s whole-buffer reversal doesn't have to pay per row.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --lib` passes 331/331 both before and after (same count, no new
  failures), including the existing `test_flip_horizontal`/`test_flip_horizontal_in_place` unit
  tests in `affine.rs`.
- Additionally (session-only, not part of `patch.diff`): wrote and ran a temporary standalone
  check (`examples/_flip_h_check.rs`, removed before capturing `patch.diff`) comparing the fast
  path against the original per-pixel algorithm on (a) a full 37x29 `RgbImage` for both
  `flip_horizontal_in` and `flip_horizontal_in_place`, and (b) a `SubImage` crop (9x6 out of a
  19x13 parent buffer) for both functions - the crop case has `width_stride == pix_stride` (fast
  path fires) but `height_stride` belongs to the wider parent row, exercising the same
  "packed row, non-contiguous buffer" case candidate 001 checked for `flip_vertical_in`. All
  variants matched the reference algorithm exactly, and pixels outside the crop were confirmed
  untouched.
- **Is this a breaking change?** No - `imageops::affine`'s public functions keep their existing
  signatures, panics, and numeric results.

## Why the result looks the way it does

Same mechanism as candidate 001: the original loops call `get_pixel`/`put_pixel` through the
generic `GenericImageView`/`GenericImage` trait methods once (`flip_horizontal_in`) or twice per
pixel-pair (`flip_horizontal_in_place`, since it swaps two positions per iteration over half the
row), each a full trait-method call including index computation and Option-based bounds checking.
The fast path replaces this with direct pixel-slice operations (`copy_from_slice`-style
iteration with `.rev()` for `flip_horizontal_in`, in-place `.reverse()` per row for
`flip_horizontal_in_place`) once per row. `flip_horizontal_in`'s two independently generic type
parameters (source view + destination buffer) match `flip_vertical_in`'s shape and get a similarly
large win; `flip_horizontal_in_place`'s single monomorphized type matches `rotate180_in_place`'s
shape and gets a smaller-but-still-real win for the same reason candidate 001 inferred (not
independently confirmed via `perf`/disassembly here either - `perf` remains unavailable in this
environment, see `README.md`).

## What's not done / open questions

- `rotate90_in`/`rotate270_in` still have the same per-pixel pattern (per `IDEAS.md`'s original
  entry for this row) but were not touched here - they transpose x<->y, which doesn't reduce to a
  row or linear reversal and needs a different (blocked-transpose) fast-path shape; that remains a
  separate, not-yet-tried candidate (kept in the backlog).
- The bench addition (`flip_horizontal_in_place` case in `benches/imageops.rs`) is bundled into
  `patch.diff` alongside the real fix (see `README.md`'s "Instrumentation added" section) since it
  was needed to reach the code at all; it is not itself a performance change.
- Did not investigate the exact codegen difference between `flip_horizontal_in_place`'s per-row
  `.reverse()` and `rotate180_in_place`'s whole-buffer `.reverse()` via disassembly - `perf` is
  not installed in this environment; the explanation above is a plausible reading of the numbers,
  not independently confirmed.
