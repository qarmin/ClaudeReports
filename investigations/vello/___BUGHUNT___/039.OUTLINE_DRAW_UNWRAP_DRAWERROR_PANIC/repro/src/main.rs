//! Standalone reproduction for candidate 039
//! (OUTLINE_DRAW_UNWRAP_DRAWERROR_PANIC).
//!
//! Drives vello_cpu's real, public glyph-rendering entry point
//! (`RenderContext::glyph_run(...).fill_glyphs(...)`) with a hand-built but
//! well-formed TrueType font whose glyph id 2 is a composite glyph combining
//! two references to a 40,000-point simple glyph (glyph id 1), for a total
//! of 80,000 points in a single outline - over skrifa's 64k (u16::MAX)
//! per-outline point limit.
//!
//! `skrifa::outline::OutlineGlyphCollection::get(gid)` succeeds for this
//! glyph (the point-count limit isn't checked during the counting/traversal
//! phase that backs `get`), so `glifo::glyph.rs`'s
//! `if let Some(outline) = outlines.get(glyph_id) else { continue }` guard
//! does not filter it out. The panic happens one line later, in
//! `OutlineCache::get_or_insert`
//! (`outline_glyph.draw(draw_settings, &mut drawing_buf).unwrap()`), when
//! skrifa's real internal buffer-sizing check detects the overflow and
//! returns `Err(DrawError::TooManyPoints(_))`.
//!
//! Exit code convention: 0 = did not reproduce (no panic), the process
//! aborting with a panic (SIGABRT/non-zero exit from an unwinding panic) =
//! reproduced.

use std::sync::Arc;

use glifo::Glyph;
use peniko::{Blob, FontData};
use vello_cpu::{Level, RasterizerSettings, RenderContext, RenderSettings, Resources};

// A hand-built, well-formed TrueType font (see `../justfile`/`RESULT.md` for
// exactly how it was generated with the `write-fonts` crate): glyph id 1 is
// a simple glyph with 40,000 coincident on-curve points (unusual, but not
// out-of-spec - the format sets no lower bound on point count and doesn't
// require distinct coordinates), and glyph id 2 is a composite glyph
// combining two references to glyph id 1, for 80,000 total points.
const TOO_MANY_POINTS_FONT: &[u8] = include_bytes!("../too_many_points.ttf");

fn main() {
    let font = FontData::new(Blob::new(Arc::new(TOO_MANY_POINTS_FONT.to_vec())), 0);

    let width = 64_u16;
    let height = 64_u16;
    let settings = RenderSettings {
        level: Level::baseline(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(width, height, settings);
    let mut resources = Resources::new();

    // Glyph id 2 is the over-64k-points composite glyph in the hand-built
    // font (glyph order: [.notdef, 40k-point simple glyph, composite]).
    let glyph = Glyph {
        id: 2,
        x: 8.0,
        y: 40.0,
    };

    // An entirely ordinary call: no internal-API shortcuts, no cache-config
    // tampering, just a real application feeding an externally-supplied
    // font to the real renderer at a normal font size.
    ctx.glyph_run(&mut resources, &font)
        .font_size(16.0)
        .fill_glyphs(std::iter::once(glyph));

    ctx.flush();

    let mut pixmap = vello_common::pixmap::Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    // If we get here, the process did not panic.
    println!("did not reproduce: rendering completed without panicking");
}
