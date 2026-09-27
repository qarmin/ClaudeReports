//! Repro for candidate 042 (COLR_CPAL_PALETTE_INDEX_UNCHECKED_SLICE).
//!
//! Renders glyph 84 of `assets/test_glyphs-glyf_colr_1-glyph84-oob-palette-index.ttf` - a real
//! copy of the project's own `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` test
//! font with exactly one 2-byte field patched: glyph 84's `PaintSolid.paletteIndex` field was
//! changed from its original value (4, a valid in-range CPAL index) to 300 (see
//! `tools/font_patch/` for the tool that made this edit and how it located the field). The
//! font's CPAL table has only 42 total color records (`numPaletteEntries=14 * numPalettes=3`),
//! so 300 is out of range but not the sentinel `0xFFFF` ("use foreground color") that
//! `palette_index_to_color` already special-cases. Every other byte, table offset and length in
//! the font is untouched, so this is a minimal, realistic "malformed COLRv1 font" - exactly the
//! kind of adversarial input a font fuzzer or a hostile document could hand to any application
//! that embeds glifo/vello for text rendering.
//!
//! This drives the *real* public entry point a real caller uses
//! (`vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)`), which is the only way
//! `glifo::colr::ColrPainter::palette_index_to_color` is ever invoked in this codebase - there
//! is no hand-authored call directly into it.

use std::iter;
use std::sync::Arc;

use glifo::Glyph;
use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{RenderContext, Resources};

const PATCHED_FONT: &[u8] =
    include_bytes!("../assets/test_glyphs-glyf_colr_1-glyph84-oob-palette-index.ttf");
const GLYPH_ID: u32 = 84;

fn main() {
    println!(
        "loaded patched font ({} bytes), rendering glyph {GLYPH_ID} (PaintSolid with \
         palette_index patched from 4 to 300, CPAL color_records_array has only 42 entries)",
        PATCHED_FONT.len()
    );

    let font = FontData::new(Blob::new(Arc::new(PATCHED_FONT.to_vec())), 0);

    let mut ctx = RenderContext::new(64, 64);
    let mut resources = Resources::new();

    ctx.set_transform(Affine::translate((10.0, 40.0)));
    ctx.glyph_run(&mut resources, &font)
        .font_size(40.0)
        .fill_glyphs(iter::once(Glyph {
            id: GLYPH_ID,
            x: 0.0,
            y: 0.0,
        }));

    println!(
        "did NOT panic: rendered glyph {GLYPH_ID} from the malformed (out-of-range \
         palette_index) font successfully"
    );
}
