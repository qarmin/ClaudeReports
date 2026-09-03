# SAMPLE_RS_ACCUMULATOR_SEED_SWEEP

Follow-up sweep from candidate 003's "What's not done": `vertical_sample` (`src/imageops/sample.rs:547`)
was confirmed to seed its per-pixel accumulator to the wrong identity element (`Rgba([1.0; 4])`
instead of `[0.0; 4]`) for a sum-of-normalized-weights operation. This candidate checks whether the
same seed-value mistake exists anywhere else in `src/imageops/sample.rs`, `src/imageops/filter_1d.rs`,
or neighboring `imageops` filter/convolution code (`fast_blur.rs`, `colorops.rs`, `affine.rs`,
`resize.rs`) that wasn't swept during candidate 003.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading: every `let mut <acc> = ...` / `zeroed()` / `Default::default()` seed site in `src/imageops/sample.rs` (`horizontal_sample`, `vertical_sample`, `interpolate_bilinear`, `thumbnail`/`ThumbnailSum`, `thumbnail_sample_block`/`_fraction_horizontal`/`_fraction_vertical`/`_fraction_both`, `filter3x3`, `get_gaussian_kernel_1d`), `src/imageops/filter_1d.rs` (`filter_symmetric_column`, `filter_symmetric_row`), and a grep sweep (`\[1\.0\|1\.0; \|Rgba(\[1`) across `src/imageops/*.rs` and `src/math/*.rs` | Static read of `src/imageops/sample.rs` (1946 lines), `src/imageops/filter_1d.rs` (953 lines), plus `fast_blur.rs`/`colorops.rs`/`affine.rs`/`resize.rs` on current `main` (44ce9226) | Found exactly one instance of the "accumulator seeded to the wrong identity element for a sum-of-normalized/convolution-weights operation" shape: the already-known `vertical_sample` line 547 (candidate 003). Every other accumulator in these files is seeded to the correct identity: `horizontal_sample`'s `t = [0.0f32; MAX_CHANNEL]`, `filter3x3`'s `t = [0.0; MAX_CHANNEL]`, `ThumbnailSum::zeroed()` (`[S::Larger::zero(); MAX_CHANNEL]`), `get_gaussian_kernel_1d`'s `sum_norm: f32 = 0f32`, `interpolate_bilinear`'s `sxx = [[0.; MAX_CHANNEL]; MAX_CHANNEL]` (a lookup table, not an accumulator, but correctly zeroed anyway), and `filter_1d.rs`'s `filter_symmetric_column`/`filter_symmetric_row`, which don't zero-seed at all - they seed the accumulator directly from `center_tap * coeff` (`v_src[i].as_().mul(coeff)`), which is the correct pattern for that code's structure (no separate zero-then-add-center-tap step to get wrong). |
| Differential empirical check: standalone `repro/` (real public API - `image::imageops::{thumbnail, blur, filter3x3, resize}`) resizes/filters a uniform-color 20x16 `Rgb<u8>` image (value 100) through every swept code path and asserts every output pixel is still exactly `[100,100,100]` - any wrong accumulator seed shows up immediately as a uniform-input regression, the same signature candidate 003 used to catch `vertical_sample`'s bug | `cargo run --release` in `repro/` against a pristine clone at `base_commit`, run twice (`just test` equivalent x2) | `thumbnail`: 0/20 wrong. `blur` (Gaussian, `get_gaussian_kernel_1d` + `filter_1d.rs`'s separable convolution): 0/320 wrong. `filter3x3`: 0/320 wrong. All three: byte-identical across both runs. `resize` (all `FilterType`s, including `Nearest`) is wrong as expected (42/42 / 1200/1200, matching candidate 003 exactly) - printed for context/harness-sanity but deliberately excluded from this candidate's own pass/fail, since that's the already-reported bug, not a new finding. |

Two approaches used (manual code reading, then a differential empirical check that would have
caught the same bug class if present) - the soft cap of 3 was not needed since the first approach
already found nothing and the second corroborated it with a real, sensitive-enough (it does catch
`vertical_sample`'s known bug) empirical test.

## Status

**Not reproducible.** No accumulator-seed bug beyond the already-confirmed `vertical_sample`
(candidate 003) was found anywhere in `src/imageops/sample.rs`, `src/imageops/filter_1d.rs`, or
`fast_blur.rs`/`colorops.rs`/`affine.rs`/`resize.rs`. Covered: full manual read of every
accumulator-seed site in both primary files plus a grep sweep across the whole `imageops`/`math`
directories for the literal `1.0`-seed pattern, and a uniform-image differential empirical check
(2/2 identical runs) across `thumbnail`, `blur` (Gaussian, which exercises `filter_1d.rs`'s
separable convolution), and `filter3x3` - the three swept operations reachable via public API that
share the same "normalized-weight accumulation" or "sum-then-average" shape as `vertical_sample`.

**Fully reproducible**: N/A (Not reproducible - no additional defect was found to reproduce).

## Class

N/A (not reproduced).

## Root cause

Not applicable in the sense of "why does it fail" - nothing here fails. The reason the suspicion
(that the same mistake might repeat elsewhere in the file) doesn't hold: every other accumulator in
`sample.rs`/`filter_1d.rs` is either explicitly zero-initialized (`[0.0f32; MAX_CHANNEL]`,
`S::Larger::zero()`, `f32 = 0f32`) or, in `filter_1d.rs`'s symmetric-convolution routines, seeded
directly from the center-tap product (`center_sample * coeff`) rather than from a separate
"start at an identity value, then add" step - there is no second `let mut ... = <wrong constant>;`
line anywhere else shaped like the one candidate 003 found. The `vertical_sample` bug appears to be
an isolated regression from a single refactor (per candidate 003's `git log -L` finding, commit
`20a2308218d110925bd3c3ff3b9ebee61354c4b0`, "Remove Pixel channels4/from_channels") that only
touched that one function, not a copy-pasted pattern that spread to sibling code.

## The fix

Not applicable - no defect found to fix in this candidate. (The one known instance of this bug
class, `vertical_sample`, already has its own fix recorded in
`003.VERTICAL_SAMPLE_PLUS_ONE_BIAS/patch.diff`.)

- Project test suite: not run for this candidate - no source change was made (nothing to verify
  before/after).

## What's not done / open questions

- `src/imageops/resize.rs` (the `pic-scale-safe`-backed implementation used by
  `DynamicImage::resize`/`resize_exact`/`resize_to_fill`) was only grep-swept, not read
  line-by-line or exercised with the uniform-image differential check, since candidate 003 already
  established it is a separate implementation from `vertical_sample`/`horizontal_sample` and
  outside this candidate's "sibling code in sample.rs/filter_1d.rs" scope - it delegates most of
  its numeric work to the external `pic-scale-safe` crate, which is out of scope for this
  repository's own source sweep.
- `src/imageops/affine.rs` and `src/imageops/colorops.rs` were grep-swept for the literal
  `1.0`-seed pattern only, not read in full or exercised empirically - neither showed any
  accumulator-seed site at all in the grep, but a full manual read (as was done for `sample.rs`/
  `filter_1d.rs`) wasn't performed on them this session.
- `filter_2d_separable`/`filter_2d_separable_ring_queue` in `filter_1d.rs` (the arena-row
  construction / plumbing around `filter_symmetric_column`/`filter_symmetric_row`) were not read
  in as much depth as the two `filter_symmetric_*` functions themselves, though the empirical
  `blur` check exercises the full path end-to-end including this plumbing.
