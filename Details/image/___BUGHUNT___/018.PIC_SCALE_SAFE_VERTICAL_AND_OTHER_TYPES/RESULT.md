# Candidate 017's `pic-scale-safe` window-misalignment bug also reproduces for the vertical resampling pass and for `Rgb8`/`Rgba8`/`Luma16`

Follow-up to candidate 017 (`PIC_SCALE_SAFE_RESAMPLING_WINDOW_MISALIGNMENT`, Confirmed), which
differentially tested only the horizontal pass (`height == 1` source rows) on `Luma8`, while noting
in its own "What's not done" that `generate_weights` (the buggy shared function) is called
identically for the vertical pass and for every other color type's fixed/floating-point variant,
without separately re-verifying either. This candidate (`IDEAS.md` line 32) re-runs the same
differential technique against the vertical pass and additional pixel types to confirm/document
that the same already-root-caused defect reproduces there too, per `___BUGHUNT___/IDEAS.md`'s note.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test against `image::imageops::resize` (independent oracle, same technique as candidate 017), real public `DynamicImage::resize_exact` API, release build, scratch `examples/` binary in the main working tree | Vertical pass (`width == 1` source column resized in height only) on `Luma8`, `FilterType::Triangle`, `(src_h, dst_h)` in `{(3,30),(4,40),(5,40),(10,777),(50,4000)}` | Confirmed: max disagreement 58/39/29/15/4 gray levels (out of 255) - numerically identical to candidate 017's horizontal-pass results for the same size pairs, as expected since it's the same 1D `generate_weights` call applied along the other axis. |
| Same technique, horizontal pass, `Rgb8` and `Rgba8` (ramp replicated into R/G/B, alpha fixed at 255), same size pairs | | Confirmed: identical 58/39/29/15/4 max disagreement pattern for both types (checked on the R channel; G/B/A are copies of the ramp/constant so carry the same result). |
| Same technique, horizontal pass, `Luma16` (fixed-point path, 16-bit ramp), same size pairs | | Confirmed: max disagreement 14746/9831/7169/3632/662 out of 65535 (~57/38/28/14/3 on a 255-equivalent scale) - same relative magnitude as the 8-bit cases. |
| Same technique, horizontal pass, `Luma32F` (floating-point `generate_weights` path) | `(src_w, dst_w)` same size pairs | Not usable as evidence either way - see "What's not done" below: the oracle itself (`imageops::resize`) has its own known, already-documented, unrelated bug (candidate 003's `vertical_sample` `+1.0`-per-channel accumulator seed, still unfixed in the working tree) that saturates large stretches of float output to exactly `1.0`, dominating and confounding any signal from this candidate's bug on the `f32` scale. Investigated and excluded rather than reported as a false positive; not carried into `repro/`. |

One approach (differential testing against the same independent oracle candidate 017 already
validated) was sufficient - it reproduced the bug with the same clear, large, non-noise magnitude
in every orientation/type combination on the first try, matching `HEURISTICS.md`'s "logic bugs:
differential testing against a reference implementation" guidance; no further escalation needed.

## Status

**Confirmed.** Deterministic, no layout/seed/timing dependence: `just test`'s
`reproduce-before`/`reproduce-after` cycle run 2/2 times independently from a fresh `git clone`,
byte-identical printed diagnostics both times (see `output.txt`; the only non-identical lines
between the two raw runs were `cargo`'s own build-progress noise - dependency compile order/cache
state and build timings - not the repro's actual measured output).

**Fully reproducible: Yes.** `repro/`'s `main.rs` calls only `image::imageops::resize` (the
oracle) and `image::DynamicImage::resize_exact` (the real subject, the crate's actual public
resize API) on real `ImageBuffer`s built through the crate's own public constructors - same
reasoning as candidate 017: a pure, single-process computational logic bug, not a reaction to
another participant's data, so `CONFIRMATION.md` section 11's "real second participant"
requirement does not apply.

## Class

logic

## Root cause

Identical to candidate 017's - `pic-scale-safe`'s `compute_weights.rs::generate_weights` derives
the source-pixel window (`start`/`end`) from `center_x` (a pixel-corner-convention point) while
every per-tap weight is computed from `center = center_x - 0.5` (the pixel-center-convention
point), a half-pixel mismatch that silently drops the correct bracketing source pixel for roughly
half of all output positions. This candidate does not find a *new* mechanism - it confirms that
the single shared `generate_weights` function candidate 017 already root-caused is reached
identically by:

- the vertical pass: `resize_fixed_point.rs`/`resize_floating_point.rs` call `generate_weights`
  once for the horizontal (width) axis and once for the vertical (height) axis, with no
  axis-specific logic in `generate_weights` itself - it only ever sees a 1D `in_size`/`out_size`
  pair, so a `width == 1` vertical-only case exercises byte-for-byte the same code as candidate
  017's `height == 1` horizontal-only case, just walking the image's height dimension instead of
  its width. The measured max-disagreement values per `(src, dst)` pair came out numerically
  identical to candidate 017's horizontal-pass numbers for the same pairs, which is exactly what
  this shared-code-path theory predicts.
- `Rgb8`/`Rgba8`/`Luma16`: `resize_fixed_point.rs` calls the same `generate_weights` regardless of
  channel count or bit depth - the weights themselves don't depend on the pixel type at all, only
  on `(in_size, out_size, filter)`, so every channel of every fixed-point color type is resampled
  with the identical (buggy) weight/window pairing.

## Introduced by

Not determined - bug resides in external crate `pic-scale-safe`'s own source (`compute_weights.rs`), not tracked in this repo's git history.

## The fix

Identical to candidate 017's fix (reused unmodified from
`___BUGHUNT___/017.PIC_SCALE_SAFE_RESAMPLING_WINDOW_MISALIGNMENT/patch.diff`, applied here against
a fresh `pic-scale-safe` clone at the same `dep_base_commit`): derive `start`/`end` from `center`
instead of `center_x`, so the window bounds and the per-tap weight calculation are measured from
the same reference point.

Verified against the new cases this candidate adds (not just the original horizontal-`Luma8` ones
candidate 017 already verified): all 4 verified type/orientation combinations (`Luma8` vertical,
`Rgb8` horizontal, `Rgba8` horizontal, `Luma16` horizontal) go from a max disagreement of
`58`/`39`/`29`/`15`/`4` (out of 255, or the `Luma16` equivalent) down to `1` (ordinary rounding
noise between two independently-implemented resamplers) after the fix, on every `(src, dst)` size
pair tried - see `output.txt`.

- Project test suite (`cargo test --lib --release`, run in the main project's own working tree,
  dependency resolution swapped to the local patched `pic-scale-safe` clone via
  `[patch.crates-io]` + `cargo update -p pic-scale-safe`): **331 passed, 0 failed** after the
  patch. (Before the patch: identical **331 passed, 0 failed**, already independently verified in
  candidate 017's own session against the same unpatched dependency version - not re-run a second
  time this session since nothing in this candidate's own probe touches the test suite itself, and
  re-running an already-established "before" number would not add information.) No regressions,
  no new failures either way; as with candidate 017, no existing test in this repo's own suite
  exercises resize output closely enough to catch this class of bug.

## What's not done / open questions

- `Luma32F` (the floating-point `generate_weights` path) was probed but its result is
  inconclusive due to a confound: the oracle (`imageops::resize`'s `vertical_sample`) has its own
  separate, already-documented, unfixed `+1.0`-per-channel accumulator bias (candidate 003), which
  is invisible in u8/u16 output (rounds away as +-1) but saturates large stretches of `f32` output
  to exactly `1.0`, swamping any signal from this candidate's bug. Re-checking the floating-point
  `generate_weights` path cleanly would need either an oracle unaffected by candidate 003's bug
  (e.g. a hand-verified closed-form linear-ramp expectation instead of `imageops::resize`) or
  candidate 003's fix applied first - neither was done this session.
- `Rgba32F`/`Rgb16`/`Rgba16`/`LumaA8`/`LumaA16` were not separately tested - by the same
  shared-`generate_weights`-with-no-type-dependence reasoning above there's no code-level reason to
  expect a different result, but this wasn't empirically re-verified for those specific types.
- Only `FilterType::Triangle` was used (matching candidate 017's own choice for its clean
  differential signal); `CatmullRom`/`Lanczos3`/`Gaussian` were not separately re-checked for the
  vertical pass or the new pixel types - candidate 017 already found `Gaussian` behaves
  differently even after the fix for unrelated reasons (see candidate 017's own "What's not done"),
  which is out of scope here too.
- Same as candidate 017: this is an upstream fix, not something that ships in this repo alone - no
  PR was opened against `awxkee/pic-scale-safe` this session.
