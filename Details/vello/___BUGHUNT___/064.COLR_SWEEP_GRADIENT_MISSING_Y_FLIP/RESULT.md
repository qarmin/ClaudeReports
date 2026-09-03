# COLR_SWEEP_GRADIENT_MISSING_Y_FLIP

`vello/src/scene.rs`'s `conv_brush` (the `skrifa::color::Brush` -> `peniko::Brush` conversion used
by classic vello's `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs) constructs a
`peniko::Gradient::new_sweep` directly from the `skrifa`-provided center/angles, with a standing
`// TODO: This is upside-down, see https://github.com/linebender/vello/pull/1221` comment sitting
directly above it. PR #1221 ("Change the direction of the angle in sweep gradients", merged
2025-09-15) fixed this exact orientation bug for `vello_cpu`/`vello_hybrid` (in what was then
`sparse_strips/vello_common/src/colr.rs`, later relocated into the separate `glifo` crate) by
negating the gradient center's y-coordinate and folding an extra `Affine::scale_non_uniform(1.0,
-1.0)` into the paint transform - but the matching fix was never applied to classic vello's own,
separate `DrawColorGlyphs`/`conv_brush` implementation, which still has the pre-fix (unflipped)
behavior. This is exactly the kind of already-self-documented gap
`___BUGHUNT___/048.COLR_BLEND_MODE_INCOMPLETE_MAPPING` (a different bug in the same function
family) suggested checking for via a differential-rendering pass against a reference COLR
renderer - here, `vello_cpu`/`glifo` (already independently validated by this project's own COLR
snapshot tests, and itself carrying PR #1221's fix) serves as that reference.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Source reading + comparison against the correct sibling implementation | Read `vello/src/scene.rs`'s `conv_brush` (`SweepGradient` arm) alongside `glifo/src/colr.rs`'s `ColorPainter::fill`/`fill_glyph` (`Brush::SweepGradient` arm), and the real diff of the upstream PR (`gh pr diff 1221 --repo linebender/vello`) that fixed the same bug for `vello_common`/`vello_cpu`/`vello_sparse_shaders` | Confirmed structurally: `glifo` negates the gradient center's y-coordinate (`Point::new(p0.x, -p0.y)`) and composes an extra `Affine::scale_non_uniform(1.0, -1.0)` into the paint transform "to bridge the gap between peniko and COLR"; `vello/src/scene.rs`'s `conv_brush` does neither - it passes `conv_point(c0)` straight through with no compensating transform, exactly matching the standing `// TODO: This is upside-down` comment |
| Initial (wrong) hypothesis, ruled out by direct measurement: a suspected non-canonical color-stop-offset handling gap in `vello_encoding`'s `make_ramp`/`RampCache` (raw font color-stop offsets like `0.2`/`0.8` not being clamped to `[0.0, 1.0]` the way `glifo`'s `convert_stops`/`vello_common::encode::EncodeExt::encode_into` explicitly do) | Built a real `linear_repeat_0.2_0.8` glyph (id 9) test via `vello_encoding::Resolver::resolve` (the exact function `vello::render::render_encoding` calls to build the real GPU-uploaded gradient ramp) and printed the real `resources.color_stops` offsets and the real `Ramps` data | Refuted: `resources.color_stops` for this glyph were already `[0.0, 1.0]` by the time they reach `vello_encoding` - `skrifa`'s own COLR paint-graph traversal already canonicalizes non-`[0,1]` gradient-line color-stop offsets (rescaling the gradient line's own endpoints to compensate) before invoking any `ColorPainter` callback, so both `glifo` and `vello/src/scene.rs` always receive already-canonical stops. No bug here (this candidate's real finding, below, was found next while investigating the same `conv_brush` function for other gaps) |
| Real, end-to-end reproduction: render the real COLRv1 test font's `sweep_0_360_pad_narrow` glyph (a real font-authored `PaintSweepGradient` with `Extend::Pad` and stops deliberately narrowed to `[0.25, 0.75]`, chosen by the test font's own authors - Google/nanoemoji - to exercise sweep-gradient direction/extend edge cases) through both `vello_cpu`'s real public `RenderContext::glyph_run(..).fill_glyphs(..)` (CPU, uses `glifo`, has PR #1221's fix) and classic vello's real public `Scene::draw_glyphs(..).draw(..)` fed through a real `wgpu` device via `vello::Renderer::render_to_texture` (the same public API a real embedding application uses) | Font: `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` (already used by this project's own `vello_sparse_tests` COLR snapshot tests), glyph id 12, 200x200 canvas, font_size 200, release build, 3 repeat runs each way | Deterministic 3/3: classic vello's real GPU render is the exact vertical mirror image of `vello_cpu`'s real render of the same glyph - 15,447 / 40,000 pixels (~39%) differ by more than 10/255 in some channel (max_diff=255); after applying the fix below, only 632 / 40,000 pixels differ (ordinary anti-aliasing noise near the circle's outer edge, max_diff=250 at a single edge pixel) |

No `miri`/ASAN/fuzzing was needed - this is a pure logic bug (a missing coordinate-space
transform, no `unsafe`, no arithmetic on untrusted sizes, no crash) in a real, already-documented
gap between two sibling renderers, so a real font-driven differential render against the correct
sibling implementation (`vello_cpu`/`glifo`) was the right tool and gave a clean, deterministic,
large-magnitude signal.

## Status

**Confirmed.**

Reproduces 3/3 runs, release build, fully deterministic (no seed/layout/timing dependency - both
renders are driven by the same fixed font file and glyph id, and `wgpu`'s CPU-computed vertex/paint
math for this scene has no GPU-timing-dependent behavior).

**Fully reproducible**: **Yes**. This is a single-participant case exempted from needing a second
real participant per `CONFIRMATION.md` section 11's own carve-out: the input is a single external
artifact (a real, unmodified COLRv1 font file already shipped in this repo) handed to each
renderer's real public entry point (`vello_cpu`'s `RenderContext::glyph_run(..)` and classic
vello's `Scene::draw_glyphs(..).draw(..)`, the latter driven all the way through a real `wgpu`
device/`Renderer::render_to_texture` call and a real pixel readback) - there is no second "peer"
being impersonated here.

## Class

logic

## Root cause

`skrifa`'s COLR paint-graph traversal invokes `ColorPainter::fill`/`fill_glyph` with a
`skrifa::color::Brush::SweepGradient { c0, start_angle, end_angle, .. }` describing the gradient in
COLR's own angle/coordinate convention. `peniko::Gradient`'s sweep-gradient convention has the
opposite angular direction/vertical orientation for the same inputs (`peniko`'s gradient math was
designed independently of COLR's spec-mandated convention). `glifo`'s `ColorPainter` implementation
(used by `vello_cpu`/`vello_hybrid`) bridges this gap explicitly - as its own comment says, "We
need to invert the direction of the gradient to bridge the gap between peniko and COLR" - by
negating the gradient center's y-coordinate and composing an additional
`Affine::scale_non_uniform(1.0, -1.0)` into the paint transform used when filling. Classic vello's
`vello/src/scene.rs`'s own, separate `conv_brush` function (used by `DrawColorGlyphs`, the
`ColorPainter` impl backing `Scene::draw_glyphs`) constructs the `peniko::Gradient::new_sweep`
directly from the raw COLR values with neither the coordinate negation nor the compensating
transform, leaving every `PaintSweepGradient` COLRv1 layer rendered as the vertical mirror image of
what `vello_cpu`/`vello_hybrid` (and the correct, spec-compliant rendering) produce for the exact
same font.

## Introduced by

edbcf4e3 "Update to latest Peniko main (#1224)" (2025-09-17) - this is the commit that rewrote
`vello/src/scene.rs`'s `conv_brush` to match a `peniko` API restructuring (`GradientKind` variants
moved into separate `LinearGradientPosition`/`RadialGradientPosition`/`SweepGradientPosition`
structs) and, in doing so, added the current `// TODO: This is upside-down, see
https://github.com/linebender/vello/pull/1221` comment - i.e. the author was aware of PR #1221
(merged two days earlier, 2025-09-15, which fixed the identical issue for `vello_common`/
`vello_cpu`/`vello_sparse_shaders`) but did not port the same fix into classic vello's copy at the
same time. The underlying sweep-gradient orientation handling in `vello/src/scene.rs` itself dates
back further, to 886604b2 "Render COLR Glyphs (Emoji) (#615)" (2024-06-21, per
`___BUGHUNT___/048`'s own investigation of the same function) - before PR #1221, both renderers
were consistently "upside-down" relative to the COLR spec (not yet a *cross-renderer* divergence);
edbcf4e3 is the commit after which the two renderers disagree with each other, which is the
specific, currently-still-present bug this candidate demonstrates.

## The fix

Changed `conv_brush` to return `(Brush, Affine)` instead of a bare `Brush` - the extra `Affine` is
an additional transform the caller must fold into whatever `brush_transform` it already passes to
`Scene::fill` (as the innermost/rightmost factor). For every brush kind except `SweepGradient` this
extra transform is `Affine::IDENTITY` (no behavior change). For `SweepGradient`, it mirrors
`glifo`'s already-proven-correct fix exactly: the gradient center's y-coordinate is negated
(`Point::new(c0.x, -c0.y)`) and the extra transform returned is `Affine::scale_non_uniform(1.0,
-1.0)`. Both of `conv_brush`'s two call sites (`DrawColorGlyphs::fill` and `::fill_glyph`) were
updated to compose this extra transform into their existing `brush_transform` argument to
`Scene::fill` (which already composes `transform * brush_transform` internally, exactly matching
the composition order `glifo` uses for its own `paint_transform`).

Verified by re-running `repro/` against the patched tree: the previously-15,447/40,000-pixel
divergence between classic vello's GPU render and `vello_cpu`'s reference render drops to
632/40,000 (visually confirmed as ordinary anti-aliasing noise near the circle's outer edge, not a
remaining orientation/logic difference - see the before/after PNGs the repro writes), 3/3 repeat
runs, both against the checked-out working tree and against a completely fresh clone of the repo at
`base_commit` (via this candidate's own `just test`).

- Project test suite (`VELLO_SKIP_LFS_SNAPSHOTS=all cargo test -p vello_tests --release`, run
  against the same fresh clone before and after applying `patch.diff`): 64 passed, 0 failed, 1
  ignored (`compare_large_bin_count`, explicitly marked "takes a long time") - identical before and
  after the patch, byte-for-byte the same per-file breakdown. `cargo test -p vello` itself has 0
  unit tests and 1 ignored doc test, unchanged either way. No existing test in this project's suite
  currently exercises a COLRv1 sweep-gradient glyph through classic vello's renderer at all - which
  is exactly why this divergence has gone unnoticed since PR #1221 landed.

## What's not done / open questions

- Only the `Extend::Pad` case (`sweep_0_360_pad_narrow`, glyph id 12) was exercised end-to-end via
  a real font and real pixel comparison this session. The font also contains matching
  `sweep_*_reflect_narrow`/`sweep_*_repeat_narrow` glyphs (and non-narrow variants) for the same
  center/angle setup; the fix (a fixed coordinate-space transform applied uniformly regardless of
  `Extend` mode) has no reason to behave differently for those, but they were not separately
  measured this session.
- The candidate's own initial (refuted) hypothesis about non-canonical COLR gradient color-stop
  offsets not being clamped in `vello_encoding`'s `make_ramp` turned out to be a non-issue for COLR
  glyphs specifically, because `skrifa` already canonicalizes stop offsets to `[0, 1]` (rescaling
  the gradient line's endpoints to compensate) before any `ColorPainter` callback ever sees them.
  This does *not* rule out `make_ramp` mishandling a **non**-canonical-offset `peniko::Gradient`
  constructed directly by a caller through the ordinary public `Scene::fill` API (bypassing COLR/
  `skrifa` entirely) - that would be a different, non-COLR-specific candidate and was not
  separately investigated here.
- No project-level regression test was added for this fix (per the workspace rule that a fix is
  never applied for real by this skill - see `SKILL.md`'s "the one rule that matters most"); a
  maintainer taking this patch may want to add a `vello_tests` snapshot test rendering
  `test_glyphs-glyf_colr_1.ttf`'s `sweep_*` glyphs through classic vello (mirroring
  `vello_sparse_tests/tests/glyph.rs`'s existing `glyphs_colr_test_glyphs` test for
  `vello_cpu`/`vello_hybrid`) so this exact class of regression is caught by CI going forward, and
  to finally close out the standing `// TODO` this candidate fixes.
