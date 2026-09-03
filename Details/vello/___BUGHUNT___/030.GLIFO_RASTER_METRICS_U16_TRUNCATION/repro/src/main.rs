//! Standalone reproduction for candidate 030
//! (GLIFO_RASTER_METRICS_U16_TRUNCATION).
//!
//! Drives vello_cpu's real, public glyph-rendering entry point
//! (`RenderContext::glyph_run(...).atlas_cache(true).fill_glyphs(...)`) with a
//! pathological-but-well-formed TrueType font (small `unitsPerEm`, an outline
//! whose coordinates are still within the format's own i16 range) at a
//! perfectly ordinary, atlas-cacheable font size (128px, exactly the default
//! `max_cached_font_size`). No config tampering, no internal API - just a
//! real application feeding a hostile font file to the real renderer, the
//! same as it would feed any other externally-supplied font.
//!
//! `glifo::renderer::calculate_raster_metrics` computes the glyph's rasterized
//! pixel width/height as `i32` and then narrows to `u16` with a bare `as u16`
//! (glifo/src/renderer.rs). When the physically-scaled glyph bbox exceeds
//! 65535px (attainable here because a tiny `unitsPerEm` combined with
//! near-i16-limit outline coordinates blows up the font-units-to-pixel
//! scale), the cast silently wraps. The wrapped, too-small size is what the
//! atlas allocator (`GlyphAtlas::insert_entry`, glifo/src/atlas/cache.rs)
//! sizes the slot with, while the actual outline is still recorded/rasterized
//! at its real, huge size - an atlas-slot under-allocation.
//!
//! Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use std::sync::Arc;

use glifo::Glyph;
use peniko::{Blob, FontData};
use vello_common::pixmap::Pixmap;
use vello_cpu::{Level, RasterizerSettings, RenderContext, RenderSettings, Resources};

// A hand-built, well-formed TrueType font: `unitsPerEm` is 16 (the OpenType
// spec's own documented minimum), and glyph id 1 ("A") has a single triangular
// contour with coordinates close to +/-4099 font units - comfortably inside
// the i16 range every glyf-table coordinate/delta is allowed to use. At
// font_size 128 (the default `max_cached_font_size`), the physical scale
// factor (font_size / unitsPerEm = 8) blows the rasterized glyph bbox up to
// roughly 65585 physical pixels per axis - just past the u16 range that
// `calculate_raster_metrics` narrows it into.
const HUGE_BBOX_FONT: &[u8] = include_bytes!("../huge_bbox.ttf");

fn main() {
    let font = FontData::new(Blob::new(Arc::new(HUGE_BBOX_FONT.to_vec())), 0);

    let width = 64_u16;
    let height = 64_u16;
    let settings = RenderSettings {
        level: Level::baseline(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(width, height, settings);
    let mut resources = Resources::new();

    // Glyph id 1 is "A" in the hand-built font (glyph order: [.notdef, A]).
    let glyph = Glyph {
        id: 1,
        x: 8.0,
        y: 40.0,
    };

    // font_size = 64.0 is an entirely ordinary UI text size, well under the
    // default `max_cached_font_size` (128.0), so atlas caching is enabled by
    // vello_cpu's real code (`GlyphCacheConfig::default()`) exactly as it
    // would be for any real application - no test-only knob is being turned
    // here.
    ctx.glyph_run(&mut resources, &font)
        .font_size(128.0)
        .atlas_cache(true)
        .fill_glyphs(std::iter::once(glyph));

    ctx.flush();

    let mut pixmap = Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    // If we got this far without panicking, check whether the pixmap
    // actually contains any rasterized coverage from the glyph at all - a
    // real under-allocated atlas slot typically manifests as the glyph being
    // silently dropped/garbled rather than a hard crash, since the CPU
    // atlas-page pixmap only grows in fixed page-sized increments and the
    // (wrapped) tiny requested slot is what gets tracked/allocated.
    let has_any_coverage = pixmap.data().iter().any(|px| px.a != 0);

    if !has_any_coverage {
        println!(
            "REPRODUCED: rendering the pathological font produced a fully blank pixmap \
             (glyph rasterization silently lost/corrupted after atlas-slot \
             under-allocation from the wrapped u16 raster metrics)"
        );
        std::process::exit(1);
    }

    println!("did not reproduce: pixmap has nonzero coverage, as expected");
}
