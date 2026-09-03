# COLR/COLRv1 differential-testing harness (vello_cpu vs resvg)

This is the differential-testing setup this candidate built and ran (see `../RESULT.md`). It
renders individual glyphs of Google's public-domain COLRv1 test font
(`examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` - the same font Chromium/Skia use
for their own COLRv1 conformance tests) through two independent implementations:

- **vello**: `vello_harness/`, a real, standalone Rust program that calls vello_cpu's real public
  `RenderContext::glyph_run` API (the same entry point `vello_sparse_tests` and the example scenes
  use) - not a hand-built stand-in for the paint graph.
- **resvg**: an independent, `ttf-parser`/`usvg`-based COLR/COLRv1 implementation (a completely
  separate codebase from `skrifa`/fontations, which vello uses), driven via a tiny generated SVG
  `<text>` element referencing the glyph by its cmap codepoint.

`compare.py` renders both, crops to the union bounding box, and writes a side-by-side PNG.

## Prerequisites

- `resvg` CLI on `PATH` (tested with 0.46.0; `cargo install resvg` gets you a compatible one).
- Python 3 with `fontTools`, `Pillow`, `numpy` (`pip install fonttools pillow numpy`).

## Usage

```
cd vello_harness && cargo build --release && cd ..
python3 compare.py <glyph_id> /tmp/out_prefix
# -> /tmp/out_prefix_combo.png (vello on the left, resvg on the right)
```

Glyph IDs and their descriptive names come straight from the font's own glyph-order table (dump
with `fontTools.ttLib.TTFont(...).getGlyphOrder()`) - the font's authors named every glyph after
the specific COLRv1 feature it exercises (`rotate_25_center_500.0_500.0`,
`radial_contained_gradient_extend_mode_reflect`, `paint_glyph_nested_rotate_center_rotate_center`,
etc.), which is what makes it practical to point a differential test at specific spec features.

## Known reference-renderer (resvg) limitations - not vello bugs

Don't mistake these for vello divergences if re-running this sweep:

- resvg's own `usvg::text::colr::GlyphPainter` prints "sweep gradients are not supported" and
  draws nothing for any `PaintSweepGradient` (glyph IDs 12-83, 152-153, 181-204, ...).
- resvg maps every COLRv1 `CompositeMode` other than `SourceOver` to a CSS `mix-blend-mode` string,
  and silently falls back the majority of Porter-Duff modes it can't express that way
  (`Clear`/`Src`/`Dest`/`SrcIn`/`DestIn`/`SrcOut`/`DestOut`/`SrcAtop`/`DestAtop`/`Xor`/`Plus`) to
  plain `"normal"` (glyph IDs 120-132).
- resvg hardcodes the COLR "foreground color" (palette index `0xFFFF`) to opaque black
  (`usvg-0.46.0/src/text/flatten.rs`'s `RgbaColor::new(0, 0, 0, 255)`) regardless of the actual
  `fill`/`color` on the `<text>` element (glyph IDs 148-155 - only comparable when vello also
  paints with black, which is what this harness's default paint does).
- resvg's own `paint_linear_gradient` has a `// TODO: We ignore x2, y2` comment - it does not apply
  `PaintLinearGradient`'s `p2` (the perpendicular/skew reference point), unlike `skrifa` (which
  normalizes `p0`/`p1`/`p2` into a single already-skewed `p0`/`p1` pair before handing it to the
  client - see `skrifa-0.44.0/src/color/mod.rs`'s `Brush::LinearGradient` doc comment). Glyph 167
  (`gradient_p2_skewed`) diverges for exactly this reason - `RESULT.md` verifies vello's rendering
  against the spec formula by hand and confirms vello is the one that's correct here.
