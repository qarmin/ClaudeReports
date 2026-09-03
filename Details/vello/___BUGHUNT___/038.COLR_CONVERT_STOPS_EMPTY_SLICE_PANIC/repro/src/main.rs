//! Repro for candidate 038 (COLR_CONVERT_STOPS_EMPTY_SLICE_PANIC).
//!
//! Renders glyph 8 of `assets/test_glyphs-glyf_colr_1-glyph8-zero-stops.ttf` - a real copy of
//! the project's own `examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf` test font
//! with exactly one 2-byte field patched: the `numStops` field of the `ColorLine` table used by
//! glyph 8's `PaintLinearGradient` was changed from 2 to 0 (see `tools/font_patch/` for the tool
//! that made this edit and how it located the field). Every other byte, table offset and length
//! in the font is untouched, so this is a minimal, realistic "malformed COLRv1 font" - exactly
//! the kind of adversarial input a font fuzzer or a hostile document could hand to any
//! application that embeds glifo/vello for text rendering.
//!
//! This drives the *real* public entry point a real caller uses
//! (`vello_cpu::RenderContext::glyph_run(..).fill_glyphs(..)`), which is the only way
//! `glifo::colr::ColrPainter::fill` (and therefore `convert_stops`) is ever invoked in this
//! codebase - there is no hand-authored call directly into `convert_stops`.
//!
//! Result (see RESULT.md for the full analysis): this does NOT panic. skrifa 0.44.0's own COLR
//! paint traversal (`skrifa::color::traversal::traverse_with_callbacks`) already guards against
//! an empty resolved color-stop list for all three gradient kinds (`(None, _) | (_, None) => {}`
//! for Linear/Radial, an explicit `if resolved_stops.is_empty() { return Ok(()); }` for Sweep)
//! and simply never calls `ColorPainter::fill` in that case - so `convert_stops` never actually
//! receives an empty slice through the real rendering path in the skrifa version glifo depends
//! on. Run with `--verbose` to print which fill() calls the traversal actually performs.

use std::iter;
use std::sync::Arc;

use glifo::Glyph;
use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{RenderContext, Resources};

const PATCHED_FONT: &[u8] =
    include_bytes!("../assets/test_glyphs-glyf_colr_1-glyph8-zero-stops.ttf");
const GLYPH_ID: u32 = 8;

fn main() {
    println!(
        "loaded patched font ({} bytes), rendering glyph {GLYPH_ID} (LinearGradient with a \
         patched numStops=0 ColorLine)",
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
        "did NOT panic: rendered glyph {GLYPH_ID} from the malformed (zero-stop ColorLine) font \
         successfully - the empty-stops path in glifo's convert_stops is unreachable through the \
         real skrifa 0.44.0 paint traversal, see RESULT.md"
    );
}
