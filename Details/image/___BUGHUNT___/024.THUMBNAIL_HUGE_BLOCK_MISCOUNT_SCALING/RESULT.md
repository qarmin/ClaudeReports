# THUMBNAIL_HUGE_BLOCK_MISCOUNT_SCALING

Backlog item from `IDEAS.md` (top row, sourced from code reading during candidate 020): re-examine
candidate 020's confirmed `thumbnail_sample_block` huge-block divisor bug at a much larger scale
(many sub-blocks per dimension, e.g. a 10000x10000 source) rather than candidate 020's small
boundary shapes (600x600, 257x257, 300x1000, 65537x2), to empirically check whether the bug's
relative error actually shrinks as the true sub-block count grows, as the code-reading-derived
`((m+1)/m)^2` model (`m = floor(span/256)`) predicts.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| In-tree scratch probe: a temporary `#[test]` fn (`scratch_thumbnail_huge_block_scaling_probe`) calling the private `thumbnail_sample_block` directly against a hand-written O(1)-memory `GenericImageView` (a virtual constant-color image, no real buffer materialized), for `m = floor(side/256)` in `{2,5,10,39,100}` (`side = 256*m + 55`, one STEP_SIZE short of the next multiple - the worst case per dimension), comparing measured output against the analytically predicted `((m+1)/m)^2` inflation factor | `cargo test --lib scratch_thumbnail_huge_block_scaling_probe -- --nocapture`, debug build, 1 run, 29.29s | Matched the predicted factor to within f32 rounding at every scale: `m=2` +125.0%, `m=5` +44.0%, `m=10` +21.0%, `m=39` +5.19%, `m=100` +2.01% - relative error shrinks monotonically as predicted, never zero. |
| Same probe extended to `m` in `{2,5,10,39,100,300,1000}` (`side` up to ~256,055) | `cargo test --release --lib scratch_thumbnail_huge_block_scaling_probe -- --nocapture`, release build, 1 run, 41.64s | Trend continues to match the `((m+1)/m)^2` model to within f32 rounding all the way to `m=1000`: +0.667% at `m=300`, +0.200% at `m=1000` - the bug is still present, just increasingly hard to notice by eye. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::DynamicImage::thumbnail_exact`) with two **real materialized** `Luma32F` images: candidate 020's own 600x600 -> 1x1 case (`m=2`) and a 10000x10000 -> 1x1 case (`m=39`, the exact scale named in this candidate's backlog entry), run via `just clone`/`reproduce-before`/`apply`/`reproduce-after` against a pristine local clone pinned at `base_commit`, 2 repeat runs each before and after | `cargo run --release` in `repro/`, via `just`; also a full fresh-clone `just test` run | Before: reproduces 2/2 runs (exit code 1); measured relative error at 10000x10000 (+5.194%) matches the predicted `(40/39)^2-1` exactly, and is visibly smaller than at 600x600 (+125.0%). After (patched clone): 2/2 runs pass (exit code 0, both scales exact). Full `just test` (fresh clone -> reproduce-before -> apply -> reproduce-after) completes cleanly end-to-end, exit code 0. |

The two escalating approaches (direct-call virtual-image scaling sweep, then the real-public-API
materialized-image repro at the named 10000x10000 scale) reached agreement with each other and
with the analytic model on the first attempt - no negative results to report, so the soft cap of
"3 escalating approaches" was not exhausted; there was nothing left to escalate to once the
hypothesis was confirmed quantitatively at both an intermediate scale (virtual-image sweep, up to
`m=1000`, ~256,000 px per dimension - materializing a real buffer at that size for every pixel
type would cost tens of gigabytes and wasn't necessary since the divisor bug has no
memory-dependent behavior) and the literal scale named in the backlog entry (a real, materialized
10000x10000 buffer through the actual public API).

## Status

**Confirmed.** This is the same underlying bug as candidate 020 (`RESIZE_THUMBNAIL_NO_ALPHA_TYPES_HUGE_BLOCK_MISCOUNT`,
already `Confirmed` and already fixed there), re-verified specifically at a much larger scale to
close that candidate's own "not checked" item. Reproduces deterministically: 1/1 in-tree
debug-build run and 1/1 release-build run of the virtual-image scaling sweep (both fully
deterministic by construction - no randomness anywhere in the input or code path, so a repeat run
is not expected nor needed to add information beyond the first), plus 2/2 release-build runs of
the standalone `repro/` against a pristine clone at `base_commit` (both before and after the fix),
plus a full clean `just test` end-to-end run.

**Fully reproducible: Yes.** The primary claim ("the bug still fires at a real 10000x10000-pixel
source, with a smaller-but-nonzero relative error than at 600x600") rests on `repro/`'s real,
materialized `ImageBuffer` handed to the crate's real public `DynamicImage::thumbnail_exact` entry
point - the same reasoning as candidate 020's own "Fully reproducible: Yes" justification (a
uniform-color image is genuine external input, not an artificial construction of internal state;
no second participant/peer/wire input is involved, so `CONFIRMATION.md` section 11's stricter bar
does not apply). The supplementary virtual-image sweep (up to `m=1000`) that extends the
measurement beyond what a real buffer could economically test is not part of `repro/` and is not
the basis for this "Fully reproducible" claim - it is included only as additional evidence that
the analytic `((m+1)/m)^2` model holds at scales too large to materialize as real memory, see
`output.txt`.

## Class

logic

## Root cause

Identical to candidate 020's (`___BUGHUNT___/020.RESIZE_THUMBNAIL_NO_ALPHA_TYPES_HUGE_BLOCK_MISCOUNT/RESULT.md`):
`thumbnail_sample_block` (`src/imageops/sample.rs`, around line 716) computes its huge-block
sub-block-count divisor via floor division (`span / STEP_SIZE` per dimension) instead of counting
the actual number of `step_by(STEP_SIZE)` iterations, which is one higher per dimension (`ceil`
instead of `floor`) whenever that dimension isn't an exact multiple of 256. This candidate's own
contribution is purely quantitative: for a block whose true sub-block count per dimension is
`m = floor(span / 256)`, the measured inflation factor matches `((m+1)/m)^2` exactly (to f32
rounding) across seven orders of magnitude of `m` (2 through 1000), confirming:

- The bug's magnitude is a **pure function of `m`**, independent of the image's actual pixel
  values or dimensions beyond determining `m` - not something that "gets better" for other
  unrelated reasons at scale, just an arithmetic consequence of `(m+1)/m -> 1` as `m -> infinity`.
- It **never fully vanishes** for a non-256-aligned source, no matter how large: the relative
  error is always strictly positive (`> 0`) whenever either span isn't an exact multiple of 256,
  asymptotically approaching (but never reaching) 0 as `1/m` for large `m` in each dimension
  (`(1+1/m)^2 - 1 ~= 2/m` for large `m`).
- At the specific 10000x10000 scale named in this candidate's own backlog entry, the error is
  +5.19% - an order of magnitude smaller than candidate 020's 600x600 case (+125%, which
  saturates to the pixel type's max for 8/16-bit integer types, masking the true multiplicative
  nature of the error), but still very much a real, measurable, non-saturating drift for a
  realistic photo-to-thumbnail size ratio - not a merely theoretical concern that "only shows up in
  tiny toy images."

## Introduced by

61c7622d "Fix overflow in `thumbnail`" (2026-05-28)

## The fix

Same fix as candidate 020 (reapplied and re-verified against this candidate's own larger-scale
repro, not re-derived): track the actual number of `sum.add_pixel` calls made (`block_count`,
incremented once per loop iteration) instead of recomputing an estimate from floor division.

```rust
let mut block_count: u64 = 0;
for y in (bottom..top).step_by(STEP_SIZE as usize) {
    for x in (left..right).step_by(STEP_SIZE as usize) {
        let k = thumbnail_sample_block(image, x, x.saturating_add(STEP_SIZE).min(right), y, y.saturating_add(STEP_SIZE).min(top));
        sum.add_pixel(k);
        block_count += 1;
    }
}
block_count
```

Verified:
- `repro/`'s 10000x10000 case (this candidate's own added scale) now returns the exact,
  unchanged value (measured relative error `0.000000` where before it was `+0.051940`, matching
  the predicted `+0.051940` exactly) - confirmed 2/2 standalone `repro/` runs against a freshly
  patched pristine clone, and via a full clean `just test` run.
- `repro/`'s 600x600 case (candidate 020's original scale, included here for a direct side-by-side
  comparison) also returns the exact value after the fix, as already established by candidate 020.
- Project test suite: `cargo test --no-fail-fast` (default features; `dav1d`/`avif-native` system
  library unavailable in this environment so `--all-features` could not be run either before or
  after - unrelated to this change, same environment limitation noted by candidate 020/022).
  **Before fix (pristine tree)**: 622 passed, 1 failed, 4 ignored (331 lib + 0 avif_termination +
  1 bad_images + 7 conversions + 6 limits + 2 limits_anim + 6 metadata + 189 reference_images + 8
  regression [1 failing] + 1 save_jpeg + 2 save_pnm + 14 truncate_images + 55 doctests). **After
  fix**: identical counts, 622 passed, 1 failed, 4 ignored - no new failures, no fixed-by-accident
  failures. The one failing test both before and after, `regression::resizing_with_catmul`, is the
  same pre-existing, unrelated failure candidate 020/021/022 already documented (confirmed present
  on a clean `main` checkout with no changes at all).

## What's not done / open questions

- **Not checked**: the alpha-bearing pixel types at this larger scale (candidate 022 already
  confirmed they share the exact same bug and fix at the small 600x600 scale; by the same
  "no type-specific logic in the buggy divisor" argument used there, the `((m+1)/m)^2` scaling
  model is expected to apply identically to them too, but this candidate's own added scale probes
  only used `Luma32F`).
- **Not checked**: whether an even more extreme `m` (e.g. `m` in the tens of millions, requiring a
  source dimension in the billions of pixels) could interact with `u64`/subpixel-`Larger`-type
  overflow in a way that changes this picture - the existing `thumbnail_huge_block` regression
  test already covers overflow-safety (not correctness) up to `4096x4113`/`1024x1024`-scale
  sources, and this candidate's own virtual-image sweep went up to `m=1000` (~256,000 px per
  dimension) without any sign of a different failure mode emerging, but genuinely astronomical `m`
  was not probed (materializing or even virtually iterating over that many pixels was judged not
  worth the runtime for a purely arithmetic model that already matches to f32 precision across
  three decades of `m`).
- No new suspect code paths were noticed while working on this candidate beyond what candidates
  020/021/022/023 already logged.
