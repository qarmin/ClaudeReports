# COLR_SWEEP_GRADIENT_DEGENERATE_ANGLE_TRANSPARENT

`vello/src/scene.rs`'s `conv_brush` (classic vello's `skrifa::color::Brush` -> `peniko::Brush`
conversion, used by `Scene::draw_glyphs(..).draw(..)` to render COLRv1 glyphs on the GPU) builds a
`peniko::Gradient::new_sweep` directly from skrifa's `start_angle`/`end_angle` with no handling for
`start_angle == end_angle`. This looked suspicious because `glifo` (the CPU/hybrid renderers'
equivalent COLR painter) already has an explicit workaround for exactly this case in
`glifo/src/colr.rs` - an asymmetry between two implementations of the same COLR paint-graph
traversal is a strong "fixed once, missed the other" smell (see `projects/vello.md`'s risk map).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `skrifa::color::traversal` and OpenType COLR spec's "sweep gradients" section | manual code reading | Confirmed `startAngle == endAngle` with `Extend::Pad` is a real, spec-legal construct skrifa forwards as-is, not a font error |
| Compared `glifo/src/colr.rs`'s `ColorPainter` against `vello/src/scene.rs`'s `conv_brush` | manual code reading | `glifo` nudges `end_angle` by 0.01 degrees for this exact case; classic vello's `conv_brush` has no equivalent nudge |
| Built a real end-to-end differential repro (see below) | release build, real wgpu GPU render vs real vello_cpu render, 3 repeat runs each of before/after | Confirmed - reproduces 3/3 runs against pristine base_commit, fixed 3/3 runs after the patch |

## Status

**Confirmed** - reproduces deterministically 3/3 runs (release build, real GPU render via this
environment's default wgpu adapter, real vello_cpu CPU render, same mutated font both times - no
randomness in either code path for this scene/font/glyph).

**Fully reproducible**: **Yes**. `repro/` drives both real public entry points directly on a real
(byte-patched) COLRv1 font: `vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)` for the
reference, and a real `vello::Scene::draw_glyphs(..).draw(..)` + real `wgpu` device/queue +
`vello::Renderer::render_to_texture` for classic vello's GPU path - reading back real rendered
pixels from both. Per `CONFIRMATION.md` section 11 this is a single-participant case (a font file
handed to each renderer's real public API), not one where a second participant's behavior needs to
be faked.

## Class

logic (silently drops a whole glyph layer - not a crash/panic)

## Root cause

skrifa's real COLR paint-graph traversal legitimately forwards a `PaintSweepGradient` with
`startAngle == endAngle` (`Extend::Pad`) to `ColorPainter::fill` - a real, spec-permitted COLRv1
construct meaning "paint everything with a single solid color," not degenerate/invalid input.
`vello/src/scene.rs`'s `conv_brush` passes `start_angle`/`end_angle` straight through to
`peniko::Gradient::new_sweep` unmodified. `vello_encoding::Encoding::encode_sweep_gradient`'s own
`(t0 - t1).abs() < SKIA_DEGENERATE_THRESHOLD` check then treats the equal-angle sweep as fully
degenerate and encodes it as fully transparent instead of the intended solid color, so the entire
glyph layer silently disappears. `glifo`'s `ColorPainter` (used by `vello_cpu`/`vello_hybrid`)
already has a workaround for this exact case (nudging `end_angle` by 0.01 degrees) - classic
vello's independent `conv_brush` implementation of the same paint-graph conversion never got the
equivalent fix.

## Introduced by

886604b2 "Render COLR Glyphs (Emoji) (#615)" (2024-06-21) - the commit that introduced classic
vello's `conv_brush` COLR-to-`peniko::Brush` conversion, including the unmodified sweep-gradient
`start_angle`/`end_angle` passthrough.

## The fix

`vello/src/scene.rs`'s `conv_brush`: when `start_angle == end_angle`, nudge `end_angle` by 0.1
degrees before constructing the sweep gradient (mirroring `glifo`'s workaround for the identical
case). Uses 0.1 degrees rather than `glifo`'s own 0.01 - `vello_encoding`'s degenerate check
compares angles as a turn fraction (degrees / 360), so a 0.01-degree nudge (turn fraction ~2.8e-5)
is still below that check's own ~3.05e-5 threshold and would still collapse to fully transparent;
0.1 degrees clears it with margin.

Verified: `repro/` no longer reproduces after the patch (3/3 runs, both renderers now report the
same 15552 opaque pixels - the full solid disc). See `output.txt` for real captured before/after
output.

- Project test suite: `vello_tests --test emoji` (`VELLO_SKIP_LFS_SNAPSHOTS=all` - this
  environment has no git-lfs data pulled, so the LFS-backed binary snapshot comparisons are
  skipped; unrelated to the patch) - 6 passed / 0 failed before, 6 passed / 0 failed after. None of
  the existing emoji/COLR tests happen to exercise a degenerate equal-angle sweep gradient, which is
  why this went undetected; the fix introduces no regression in the cases they do cover.

## What's not done / open questions

- Only classic vello's (`vello` crate) `conv_brush` was checked and fixed. `vello_hybrid`'s own COLR
  handling was not audited for a similar independent conversion path that might share this gap -
  worth a quick check in a future session (grep for other `SweepGradient` match arms outside
  `glifo` and `vello/src/scene.rs`).
- Only `Extend::Pad` was exercised (the case skrifa's traversal doc calls out) - `Extend::Repeat`/
  `Reflect` with an equal-angle sweep were not separately tested, though the same `conv_brush` code
  path and fix apply regardless of `extend` mode.
- The git-lfs snapshot PNGs could not be pulled in this environment (no network-backed LFS store
  configured) - a maintainer with LFS access should also re-run the full (non-skipped) snapshot
  suite before merging, in case any *existing* snapshot happens to already cover this glyph/font at
  higher fidelity than this candidate's own repro checks.
