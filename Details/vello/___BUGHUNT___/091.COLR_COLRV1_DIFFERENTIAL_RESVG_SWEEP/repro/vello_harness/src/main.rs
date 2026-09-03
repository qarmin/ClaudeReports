//! Renders a single glyph from the real COLRv1 test font
//! (`test_glyphs-glyf_colr_1.ttf`, the same public-domain font used by
//! Chromium/Skia's own COLRv1 tests, and already vendored in this repo under
//! `examples/assets/colr_test_glyphs/`) through vello_cpu's real, public
//! `RenderContext::glyph_run` glyph-rendering entry point - the same API path
//! `vello_sparse_tests` and the example scenes use.
//!
//! Pairs with `../compare.py`, which renders the same glyph via `resvg` (an
//! independent, ttf-parser/usvg-based COLRv1 implementation) and produces a
//! side-by-side comparison image.
//!
//! Usage: colr_diff_harness <glyph_id> <out.png>

use std::env;
use std::fs;
use std::sync::Arc;

use glifo::Glyph;
use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{RenderContext, Resources};

const TEST_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../clone/examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf"
));

/// The glyph's own design-space origin (0, 0) is placed at pixel (`CX`, `CY`) in a
/// `SIZE`x`SIZE` canvas, with `font-size = upem` so 1 font unit == 1 device pixel -
/// this matches the SVG rendering set up in `../compare.py` (font-size=1000, text x=CX y=CY).
const SIZE: u16 = 2000;
const CX: f64 = 700.0;
const CY: f64 = 1300.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let glyph_id: u32 = args[1].parse().expect("glyph id");
    let out_path = &args[2];

    let font = FontData::new(Blob::new(Arc::new(TEST_FONT)), 0);

    let mut ctx = RenderContext::new(SIZE, SIZE);
    let mut resources = Resources::new();

    ctx.set_transform(Affine::translate((CX, CY)));
    ctx.glyph_run(&mut resources, &font)
        .font_size(1000.0)
        .hint(false)
        .fill_glyphs(std::iter::once(Glyph {
            id: glyph_id,
            x: 0.0,
            y: 0.0,
        }));

    let mut target = vello_common::pixmap::Pixmap::new(SIZE, SIZE);
    ctx.flush();
    ctx.render(&mut target, &mut resources);

    let png = target.into_png().expect("encode png");
    fs::write(out_path, png).expect("write png");
}
