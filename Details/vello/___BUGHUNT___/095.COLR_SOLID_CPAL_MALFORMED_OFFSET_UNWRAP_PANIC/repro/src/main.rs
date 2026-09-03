//! Repro for candidate 095 (COLR_SOLID_CPAL_MALFORMED_OFFSET_UNWRAP_PANIC).
//!
//! Renders glyph 84 of `assets/test_glyphs-glyf_colr_1-cpal-oob-offset.ttf` - a real copy of the
//! project's own `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` test font with
//! exactly one 4-byte field patched: the CPAL table's `colorRecordsArrayOffset` field (see
//! `tools/font_patch/` for the tool that made this edit and verified the effect) was changed
//! from its original, valid value (30) to a value (0x0FFF_FFFF) that points far past the end of
//! the font file. Every other byte, table offset and length in the font is untouched - this is a
//! minimal, realistic "malformed COLRv1+CPAL font" (a font whose CPAL table header parses fine,
//! `FontRef::cpal()` succeeds, but whose actual color-records array is unreadable), exactly the
//! kind of adversarial input a font fuzzer or a hostile embedded font in a document could hand to
//! any application that embeds classic `vello` for COLR emoji/color-glyph text rendering.
//!
//! This drives the real public entry point a real caller uses
//! (`vello::Scene::draw_glyphs(..).draw(..)`), which is the only way
//! `vello::scene::conv_brush`/`color_index` is ever invoked in this codebase - there is no
//! hand-authored call directly into the private `conv_brush`/`color_index` functions. No GPU
//! device or renderer is needed: `Scene::draw_glyphs(..).draw(..)` performs the full COLR paint
//! traversal (including the `conv_brush` call that panics) synchronously while just encoding the
//! scene, before anything is ever submitted to a GPU.

use std::sync::Arc;

use vello::Glyph;
use vello::kurbo::Affine;
use vello::peniko::{Blob, Brush, Fill, FontData, color::palette};

const PATCHED_FONT: &[u8] =
    include_bytes!("../assets/test_glyphs-glyf_colr_1-cpal-oob-offset.ttf");
const GLYPH_ID: u32 = 84;

fn main() {
    println!(
        "loaded patched font ({} bytes), rendering glyph {GLYPH_ID} (CPAL colorRecordsArrayOffset \
         patched to point out-of-bounds; PaintSolid.paletteIndex for this glyph is untouched, a \
         normal in-range CPAL index)",
        PATCHED_FONT.len()
    );

    let font = FontData::new(Blob::new(Arc::new(PATCHED_FONT.to_vec())), 0);
    let mut scene = vello::Scene::new();

    scene
        .draw_glyphs(&font)
        .font_size(40.0)
        .transform(Affine::translate((10.0, 40.0)))
        .brush(&Brush::Solid(palette::css::WHITE))
        .draw(
            Fill::NonZero,
            std::iter::once(Glyph {
                id: GLYPH_ID,
                x: 0.0,
                y: 0.0,
            }),
        );

    println!(
        "did NOT panic: encoded glyph {GLYPH_ID} from the malformed (out-of-bounds CPAL \
         colorRecordsArrayOffset) font successfully"
    );
}
