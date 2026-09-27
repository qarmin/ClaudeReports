# `pic-scale-safe`'s resampling window is computed from the wrong reference point, silently dropping the correct source pixel for roughly half of all output positions

`DynamicImage::resize`/`resize_exact` (`src/imageops/resize.rs`) delegate all real resampling work
to the external `pic-scale-safe` crate (`0.1.11` as locked in this repo's `Cargo.lock` at
`base_commit`, also the crates.io-latest `0.1.12` as of this session - both share the exact same
`compute_weights.rs`). This candidate was queued in `IDEAS.md` specifically because an earlier
session (candidate 010) had verified this repo's own glue code around `pic-scale-safe` is
accumulator-bug-free, but explicitly never swept `pic-scale-safe`'s own internal weight-generation
math.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Uniform-color invariant sweep, real public API (`DynamicImage::resize_exact`), large images (up to 4000x3000, 100003px-wide rows) and usize/u16-boundary-adjacent dimensions (65535/65536/65537, 131072), release build, scratch `examples/` binary | 25 size combinations x 5 `FilterType` variants | Clean - every output pixel exactly equal to the uniform input color in every case. This invariant is blind to a bug that mispairs weights with the wrong source pixel, since any weight set that still sums to 1 reproduces a constant regardless of which source pixel gets which weight. |
| Linear-ramp reproducing-property sweep (a normalized, symmetric resampling kernel must reproduce a linear ramp exactly away from image edges - much more sensitive, since it's sensitive to *which* source pixel a weight lands on, not just whether weights sum to 1), `Luma32F`, real public API, release build, scratch `examples/` binary | 16 (src_w, dst_w) pairs x 4 non-Nearest filters | Found real, non-noise deviations for `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3` - not a rounding-level effect (some individual points off by 0.02-0.04 on a 0-1 scale). Live signal, escalated rather than stopping at 2 approaches. |
| Differential test against `image::imageops::resize` (the crate's own independent, separately-implemented pure-Rust legacy resizer, kept in the crate purely as a public free function, unrelated to `pic-scale-safe`) on `Luma8` ramps, `FilterType::Triangle`, real public API on both sides, release build | `(src_w, dst_w)` in `{(2,20),(3,30),(4,40),(5,40),(10,777),(50,4000)}` | Confirmed: `DynamicImage::resize_exact` disagrees with the independent legacy oracle by up to 58 gray levels (out of 255) on a monotonic ramp, in a characteristic "plateau" pattern (several consecutive output pixels repeat the exact same value instead of advancing smoothly) - not ordinary +-1 rounding noise between two different algorithms. |

Two tool classes used before the differential test gave a clear, large, reproducible signal (the
soft cap of 3 wasn't needed - manual boundary/invariant construction plus one differential-oracle
comparison, both fitting `HEURISTICS.md`'s "logic bugs: differential testing against a reference
implementation" guidance, were enough).

## Status

**Confirmed.** Deterministic, no layout/seed/timing dependence: `just test`'s
`reproduce-before`/`reproduce-after` cycle run 2/2 times independently from a fresh `git clone`,
byte-identical printed diagnostics both times (see `output.txt`).

**Fully reproducible: Yes.** `repro/`'s `main.rs` calls only `image::imageops::resize` (the oracle)
and `image::DynamicImage::resize_exact` (the real subject, the crate's actual public resize API,
which is what any real caller uses) on a real `ImageBuffer` built through the crate's own public
constructor. This is a pure, single-process computational logic bug in how a caller-supplied
image is resampled, not a reaction to another participant's data - `CONFIRMATION.md` section 11's
"real second participant" requirement does not apply, the same reasoning as candidate 003 (a
sibling accumulator-class bug in this same repo, found in the pre-`pic-scale-safe` legacy resizer).

## Class

logic

## Root cause

`compute_weights.rs`'s non-area branch of `generate_weights` (used by every `FilterType` this
repo exposes except effectively-degenerate cases) computes two different things from two
different reference points that are supposed to be the same location:

```rust
let center_x = ((i.as_() + 0.5.as_()) * scale).min(in_size.as_());
// ...
let start: usize = (center_x - filter_radius).floor().max(0f32.as_()).as_();
let end: usize = (center_x + filter_radius)
    .ceil()
    .min(in_size.as_())
    .min(start.as_() + kernel_size.as_())
    .as_();

let center = center_x - 0.5.as_();       // <-- the -0.5 pixel-center offset

for (local_filter_iteration, k) in (start..end).enumerate() {
    let dx = k.as_() - center;           // per-tap weight is measured from `center`, not `center_x`
    // ...
}
```

`center_x` is a pixel-*corner*-convention sample point; `center = center_x - 0.5` converts it to
the pixel-*center*-convention point that `dx` (and therefore every per-tap weight) is actually
measured from - this conversion is standard and correct. The bug is that `start`/`end` (which
integer source pixels get included in the window at all) are derived from `center_x`, not from
`center`. This shifts the window by half a pixel relative to where the weights are actually
evaluated. For a 2-tap kernel (`Triangle`/`Bilinear`, `filter_radius == 1`), the two true
bracketing pixels are always `floor(center)` and `floor(center) + 1`; the buggy `start` instead
picks `floor(center_x - 1)`, which equals the correct value only when `frac(center)` happens to be
`>= 0.5` - a coin flip. When it lands wrong, the window includes one pixel that's *outside* the
kernel's support (weight computes to exactly `0`) and excludes the one pixel that should have
carried most or all of the weight. The `weights_sum != 0` renormalization then blindly rescales
whatever nonzero weight (if any) survived in the *wrong* window to sum to `1`, silently turning
what should be a real interpolation between two source pixels into a copy of a single, often
adjacent-but-wrong, source pixel - the "plateau" pattern seen in the differential test. The same
misalignment applies to every other filter that shares this code path (`CatmullRom`, `Gaussian`,
`Lanczos3`, and any other non-`Area` `ResamplingFunction` this crate defines), scaled by their
own, larger `filter_radius`.

This is not new in `0.1.12` (crates.io-latest as of this session) - `0.1.12`'s only change to
`compute_weights.rs` relative to `0.1.11` adds a fallback for the *unrelated*, more severe case
where a window's weights sum to exactly `0` (no valid tap at all), plus a unit test asserting every
window's weights sum to `1` - a test this specific bug does not fail, since the renormalization
step already forces the (wrong) window's weights to sum to exactly `1`.

Confirmed present (via the same differential technique, `bughunt_probe_017b.rs`) in:
`Luma8` (fixed-point path, `resize_plane8`) driven through `DynamicImage::resize_exact`. Not
separately re-verified this session for `Luma16`/`Rgb16`/`Rgba16` (fixed-point path) or
`Rgb8`/`Rgba8`, though `compute_weights.rs` is shared by every color type's fixed- and
floating-point path alike (`resize_fixed_point.rs` and `resize_floating_point.rs` both call the
same `generate_weights` - see those files' `use crate::compute_weights::generate_weights;`), so
there is no code-level reason this would be type-specific.

## Introduced by

Not determined - bug resides in external crate `pic-scale-safe`'s own source (`compute_weights.rs`), not tracked in this repo's git history.

## The fix

Compute `center` first, and derive `start`/`end` from `center` (the same reference point `dx` is
measured from) instead of `center_x`:

```diff
-            let start: usize = (center_x - filter_radius).floor().max(0f32.as_()).as_();
-            let end: usize = (center_x + filter_radius)
-                .ceil()
+            let center = center_x - 0.5.as_();
+
+            let start: usize = (center - filter_radius).ceil().max(0f32.as_()).as_();
+            let end: usize = (start.as_() + kernel_size.as_())
                 .min(in_size.as_())
-                .min(start.as_() + kernel_size.as_())
                 .as_();
-
-            let center = center_x - 0.5.as_();
```

(`end` is now derived directly from `start + kernel_size` clamped to `in_size`, rather than
independently from `ceil(center_x + filter_radius)` - since `start` is now correct, the window's
width is exactly `kernel_size` except where clamped by the image boundary, which is what
`kernel_size` was already sized for.)

This patch is against `pic-scale-safe`'s own repository (`awxkee/pic-scale-safe`,
`dep_base_commit` `83c4801428c436678d7c6b6bea74af800924a469`, == published tag `0.1.11`), not
against this repo - the bug is entirely inside the dependency. `patch.diff` applies cleanly to a
pristine clone of that commit (verified via `git apply --check`).

Verified against more than the one originally-triggering input, per `CONFIRMATION.md` section 7:
all 5 differential cases in `repro/` (`(3,30)`, `(4,40)`, `(5,40)`, `(10,777)`, `(50,4000)`) go
from `max |reference - subject|` of `58`/`39`/`29`/`15`/`4` gray levels down to `1` (ordinary
rounding noise between two independently-implemented resamplers) after the fix - see
`output.txt`. Also separately re-checked with the broader `bughunt_probe_017.rs`/`017b.rs`
sweeps used during triage (not carried into `repro/`, kept only as session scratch): the
`Triangle` linear-ramp-reproduction failures (321/897 sampled points, unpatched) drop to `0/716`
after the fix; `CatmullRom` similarly drops from a nonzero count to `0` once the sweep's edge
margin is widened enough to stop conflating this bug with ordinary boundary-clipping (`CatmullRom`
has a wider kernel than `Triangle`, so it needs a bigger away-from-the-edge margin to isolate the
same effect cleanly) - see "What's not done" below for `Gaussian`/`Lanczos3`, which were not as
cleanly resolved by this same fix.

- Project test suite (`cargo test --lib`, run in the main project's own working tree, dependency
  resolution swapped between real crates.io `pic-scale-safe` and the local patched path via
  `[patch.crates-io]` + `cargo update -p pic-scale-safe`): **331 passed, 0 failed** before the
  patch -> **331 passed, 0 failed** after the patch (identical pass count, no regressions, no new
  failures). As with candidate 003's sibling bug, no existing test in this repo's own suite
  exercises resize output closely enough to catch this class of bug either way.

## What's not done / open questions

- This is an upstream fix, not something that can ship in this repo alone - it needs to land in
  `pic-scale-safe` itself (and a subsequent version bump here) to actually reach users of
  `DynamicImage::resize`/`resize_exact`. No PR was opened against `awxkee/pic-scale-safe` this
  session.
- `Gaussian`'s linear-ramp sweep failures did not clear up with this fix (in fact got numerically
  larger on the same small-source/extreme-upscale cases, `src_w` 5 and 10 only) - but breaking
  down by `src_w` shows the *same* small-source cases fail both before and after the fix, just
  with different specific deviation values; this looks like `Gaussian`'s own fixed, scale-independent
  support width (it doesn't widen its kernel for upscaling the way `Triangle`/`CatmullRom`/
  `Lanczos3` do, per `is_resizable_kernel`/`min_kernel_size` in `sampler.rs`) being a real design
  limitation unrelated to this specific windowing bug, not a regression this fix introduced - not
  independently confirmed as a distinct bug this session, and not differentially tested against an
  independent oracle the way `Triangle` was.
- `Lanczos3`'s few remaining linear-ramp sweep failures were all confined to the single smallest
  source width tried (`src_w == 5`, a 6-tap kernel on a 5-pixel-wide image, i.e. inherently
  boundary-affected everywhere) - consistent with a probe-margin artifact, not re-verified with a
  wider margin or a differential oracle this session.
- Only horizontal resampling was differentially tested (`height == 1` / single-row images);
  `compute_weights` is also used for the vertical pass (same function, called with the image's
  height instead of width - see `resize_floating_point.rs`/`resize_fixed_point.rs`), so the same
  bug necessarily affects vertical resizing too, but this wasn't separately exercised with its own
  differential test this session.
- The `Area` resampling branch of `generate_weights` (a separate code path in the same function,
  used only for `ResamplingFunction::Area`, which this repo's `FilterType` enum doesn't currently
  expose) was read but not tested - it has its own, structurally different `start`/`end`
  computation and wasn't part of this bug.
