# COLR_RADIAL_GRADIENT_ZERO_END_RADIUS_SOLID_FALLBACK

`vello/src/scene.rs`'s `conv_brush` (classic vello's `skrifa::color::Brush` -> `peniko::Brush`
conversion) and `glifo/src/colr.rs`'s `ColorPainter::fill` (used by `vello_cpu`/`vello_hybrid`) are
two independent Rust re-implementations of the same skrifa COLR paint-graph conversion.
Candidate 081 already found a one-sided `SweepGradient` divergence fixed only in `glifo`; this
candidate diffs `RadialGradient`/`LinearGradient`/`Solid` conversion for the same kind of gap and
finds one in the opposite direction: `glifo`'s own `RadialGradient` arm has an extra special case
classic vello's `conv_brush` doesn't have, and that special case is itself wrong for a real,
reachable input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `vello/src/scene.rs`'s `conv_brush` alongside `glifo/src/colr.rs`'s `ColorPainter::fill` for the `Solid`/`LinearGradient`/`RadialGradient` arms | manual code reading | `Solid` matches. `LinearGradient`: `glifo` has no `p0 == p1` guard, but `skrifa::color::traversal::traverse_with_callbacks`'s own `LinearGradient` handling already intercepts `p1 == p0 \|\| p2 == p0 \|\| cross_product(...) == 0.0` **before** ever calling `ColorPainter::fill`, substituting `Brush::Solid` itself - so this specific gap is not reachable through the real skrifa paint-graph traversal (ruled out, not a real divergence). `RadialGradient`: `glifo` has `if r1 <= 0.0 \|\| stops.len() == 1 { fill_solid(stops[0].color) }`; `conv_brush` has no equivalent check at all. Confirmed skrifa's own traversal has **no** degenerate-radius guard for `RadialGradient` (unlike `LinearGradient`) - it forwards `r1 == 0.0` (and even negative radii) straight through to `ColorPainter::fill` unmodified, so this is real, reachable input |
| Read the downstream gradient-encoding machinery both renderers ultimately rely on (`vello_shaders/shader/draw_leaf.wgsl`'s `DRAWTAG_FILL_RAD_GRADIENT` case, its CPU-shim twin `vello_shaders/src/cpu/draw_leaf.rs`, and `sparse_strips/vello_common/src/encode.rs`'s `FocalData`/`validate`) | manual code reading | All three independently port Skia's two-point-conical-gradient algorithm, which has a **dedicated, working code path for `r1 == 0.0`** (`draw_leaf.wgsl`: `if r1 == 0.0 { swap points/radii }`; `vello_common`'s `FocalData::create`: `f_focal_x = r0/(r0-r1)` hits exactly `1.0` and takes its own documented swap branch) - `r1 == 0.0` is not degenerate to the actual rendering pipeline on either side, only to `glifo`'s own early-return |
| Built a real end-to-end differential repro (see below) | release build, real wgpu GPU render vs real vello_cpu render, 3 repeat runs each of before/after | Confirmed - reproduces 3/3 runs against pristine base_commit, fixed 3/3 runs after the patch |

No `miri`/ASAN/fuzzing was needed - this is a pure logic/rendering-divergence bug (no `unsafe`, no
arithmetic on untrusted sizes, no crash), so a real font-driven differential render between the two
sibling COLR paint converters was the right tool and gave a clean, deterministic, large-magnitude
signal on the first attempt.

## Status

**Confirmed** - reproduces deterministically 3/3 runs (release build, real GPU render via this
environment's default wgpu adapter, real vello_cpu CPU render, same mutated font both times - no
randomness in either code path for this scene/font/glyph).

**Fully reproducible**: **Yes**. `repro/` drives both real public entry points directly on a real
(byte-patched) COLRv1 font: `vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)` for the
reference (uses `glifo` internally), and a real `vello::Scene::draw_glyphs(..).draw(..)` + real
`wgpu` device/queue + `vello::Renderer::render_to_texture` for classic vello's GPU path - reading
back real rendered pixels from both. Per `CONFIRMATION.md` section 11 this is a single-participant
case (a font file handed to each renderer's real public API), not one where a second participant's
behavior needs to be faked.

## Class

logic (one renderer silently collapses a real gradient shape to a flat solid fill; not a
crash/panic)

## Root cause

A `PaintRadialGradient` whose end circle has radius 0 (`radius1` is stored as a plain `UFWORD` in
the font file, and 0 is an ordinary, directly encodable value - no variable-font deltas needed) is
a real, spec-legal COLRv1 "focal point" two-point-conical gradient: the end circle degenerates to a
point while the start circle keeps its own real center/radius, producing a cone/wedge-shaped
gradient. Skia's own two-point-conical-gradient algorithm - which this project ports independently
in three places (`vello_shaders/shader/draw_leaf.wgsl` for classic vello's GPU path,
`vello_shaders/src/cpu/draw_leaf.rs` for classic vello's CPU fallback, and
`sparse_strips/vello_common/src/encode.rs`'s `FocalData` for `vello_cpu`/`vello_hybrid`) - has a
dedicated, correct code path for exactly this case (a "swap start/end" trick when the computed
focal-x lands exactly on 1.0).

`glifo/src/colr.rs`'s `ColorPainter::fill` (`Brush::RadialGradient` arm) pre-empts all of that with
its own early check, `if r1 <= 0.0 || stops.len() == 1`, which treats `r1 == 0.0` as fully
degenerate and short-circuits to `fill_solid` using only the **first** color stop's color -
discarding the entire gradient shape in favor of a single flat color covering the whole glyph area.
`vello/src/scene.rs`'s `conv_brush` has no such check at all and passes `r0`/`r1` straight through
to `peniko::Gradient::new_two_point_radial`, letting the real gradient-encoding machinery (the same
machinery `glifo` itself ultimately relies on for every *other* radial gradient) render the correct
cone shape.

`skrifa::color::traversal`'s own `RadialGradient` handling has no degenerate-radius guard at all
(unlike its `LinearGradient` handling, which does intercept `p0 == p1` before calling
`ColorPainter::fill`) - `r1 == 0.0` (and even negative radii, per the standing
`// TODO: Radial gradients with negative r0` comment directly above this code in `glifo`) reaches
`ColorPainter::fill` completely unmodified, confirming this is real, reachable input through the
genuine font -> skrifa -> `ColorPainter`/`conv_brush` pipeline, not a hand-invented one.

## Introduced by

743efe11 "Add support for rendering bitmap and COLR glyphs (#937)" (2025-05-08) - the commit that
introduced this exact `if r1 <= 0.0 { fill_solid(...) }` check, then in
`sparse_strips/vello_common/src/colr.rs` (the code now living in the separate `glifo` crate,
carried forward unchanged through 5796226b "Import glifo into the vello workspace (#1539)" and
46cc30b7 "Migrate vello sparse strips renderers to use glifo (#1562)").

## The fix

`glifo/src/colr.rs`'s `ColorPainter::fill`, `Brush::RadialGradient` arm: change the condition from
`r1 <= 0.0` to `r1 < 0.0`. COLR radii are stored as `UFWORD` (only reachable as negative through a
variable font's own delta-adjusted instance), so this narrows the solid-fallback to genuinely
invalid input while letting `r1 == 0.0` flow into the same gradient-encoding path every other
radial gradient already uses (which, as shown above, already has a correct, dedicated code path for
this exact case). `stops.len() == 1` is untouched (that check is correct and not part of this
divergence - `vello_encoding`'s own `add_ramp`/`RampStops::One` handles the single-stop case
identically on classic vello's side).

Verified: `repro/` no longer reproduces after the patch (3/3 runs; both renderers now report
near-identical opaque-pixel/distinct-color counts for glyph 96 - 6930/131 vs 6982/133, the small
remainder being ordinary antialiasing noise between the two independent rasterizers, not a
flat-vs-gradient disagreement). See `output.txt` for the real captured before/after output, and
`reference_vello_cpu_BEFORE_FIX.png` / `reference_vello_cpu_AFTER_FIX.png` /
`classic_vello_gpu.png` for the actual rendered images.

- Project test suite: `cargo test --workspace --lib --release` (this repo's full workspace unit
  test suite) - 441 passed / 0 failed / 1 ignored before, byte-for-byte identical (441 passed / 0
  failed / 1 ignored) after. No unit test anywhere in the workspace happens to exercise a
  zero-end-radius radial gradient, which is why this went undetected; the fix introduces no
  regression in the cases they do cover.
- `cargo test -p vello_sparse_tests --release --test tests glyphs_colr_test_glyphs` (the one
  integration test that renders every glyph of the exact same test font, including glyph 96, via a
  checked-in reference PNG) could not be used to verify pixel-for-pixel correctness either way:
  this environment never pulled git-lfs content, so its reference PNGs are still raw git-lfs
  pointer text, and every variant of this test panics identically (`Unsupported(UnsupportedError {
  format: Unknown, kind: Format(Unknown) })` while decoding the pointer file as an image) both
  before and after this fix - see `output.txt`. A maintainer with real git-lfs access should
  re-run this specific test before merging, since the checked-in reference image for
  `glyphs_colr_test_glyphs` was almost certainly captured with the pre-fix (flat-fill) behavior for
  glyph 96 and would need regenerating.

## What's not done / open questions

- Only `RadialGradient` was found to diverge; `LinearGradient` and `Solid` conversion were checked
  and found to match (or, for `LinearGradient`'s theoretical `p0 == p1` case, found unreachable
  through the real skrifa traversal - see "What was tried" above).
- The standing `// TODO: Radial gradients with negative r0` comment directly above this fix in
  `glifo/src/colr.rs` is a separate, already-self-documented gap not addressed here: neither
  `glifo` nor `conv_brush` guards against a negative `r0` (only `glifo`'s `r1` had an - overly
  broad - guard). Whether a variable font can actually drive `r0` negative via delta application,
  and what either renderer's downstream gradient-encoding machinery does with it, was not
  investigated this session.
- A further, more pathological edge case was noticed but not chased down: if a font ever encodes
  **both** `r0 == 0.0` and `r1 == 0.0` with distinct centers (as opposed to this candidate's
  `r0 == 100, r1 == 0`), `sparse_strips/vello_common/src/encode.rs`'s own `validate()` function does
  not treat this as degenerate (its check only fires when centers are *also* nearly equal), so it
  would reach `FocalData::create(0.0, 0.0, ..)`, computing `f_focal_x = 0.0 / (0.0 - 0.0)` = `NaN`.
  This looks like a latent, pre-existing bug shared identically by *both* renderers (classic
  vello's `conv_brush` already has zero protection against it either way), not a new divergence
  introduced by this candidate's fix - but it was not reproduced or root-caused this session and
  would make a reasonable future candidate.
- Only classic vello's GPU rendering path (`Renderer::new` with `use_cpu: false`) was exercised for
  the "classic vello" side of the differential repro; its CPU-fallback shim
  (`vello_shaders/src/cpu/draw_leaf.rs`, `use_cpu: true`) implements the identical Skia-derived
  math (confirmed by reading, see "What was tried" above) but was not separately run in this
  candidate's repro.
