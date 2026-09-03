//! Standalone reproduction for candidate 062
//! (GLIFO_UNWRAP_EXPECT_SWEEP_CONTINUATION).
//!
//! Drives `glifo`'s own real, public API directly (this crate is published on
//! its own, independent of `vello_cpu`/`vello_hybrid` - see its Cargo.toml
//! description "Glifo provides APIs for efficiently rendering text") to cache
//! exactly one ordinary, well-formed COLR (color) glyph via the same
//! production code path `vello_cpu`/`vello_hybrid` use internally
//! (`GlyphRunBuilder`/`GlyphRunBackend::fill_glyphs` -> `GlyphRun::build` ->
//! `GlyphRunRenderer::fill_glyphs`), using the project's own bundled
//! `NotoColorEmoji-Subset.ttf` test font with default atlas-cache settings
//! (`GlyphCacheConfig::default()`, `max_cached_font_size: 128.0`, at font size
//! 16.0 - an entirely ordinary configuration and font size).
//!
//! `glifo`'s real caching code (`glyph.rs`) tags every cached COLR glyph's
//! `GlyphCacheKey.subpixel_x` with the sentinel `SUBPIXEL_COLR = 4` (bitmap
//! glyphs get `SUBPIXEL_BITMAP = 5`), specifically so COLR/bitmap keys can
//! never collide with an outline key (whose `subpixel_x` is always in
//! `0..SUBPIXEL_BUCKETS`, i.e. `0..4`). `GlyphAtlas::stats()` - `glifo`'s own
//! public, debug-only glyph-cache diagnostics API - allocates a
//! `[usize; SUBPIXEL_BUCKETS as usize]` (a 4-element array) and indexes it
//! with the raw `key.subpixel_x` for every cached entry with no bounds check,
//! so calling `.stats()` after caching even one COLR or bitmap glyph panics
//! with an out-of-bounds index (4 or 5 into a length-4 array). No malformed
//! font content is needed at all - this is completely ordinary usage of a
//! completely well-formed color font.
//!
//! Exit code convention: 0 = did not reproduce (no panic), the process
//! aborting with a panic (SIGABRT/non-zero exit from an unwinding panic) =
//! reproduced.

use std::sync::Arc;

use glifo::{AtlasCacher, AtlasConfig, Glyph, GlyphAtlas, GlyphPrepCache, GlyphRun, GlyphRunBackend};
#[cfg(debug_assertions)]
use glifo::atlas::GlyphCacheStats;
use peniko::color::{AlphaColor, Srgb};
use peniko::color::palette::css::BLACK;
use peniko::kurbo::{Affine, BezPath, Rect};
use peniko::{Blob, BlendMode, FontData};
use vello_common::image_cache::ImageCache;
use vello_common::paint::{Image, ImageId, ImageSource, PaintType, Tint};

const NOTO_COLR_FONT: &[u8] =
    include_bytes!("../../clone/examples/assets/noto_color_emoji/NotoColorEmoji-Subset.ttf");

/// A do-nothing rendering backend - mirrors the shape of `glifo`'s own
/// `#[cfg(test)]` `NoopRenderer` and `vello_cpu`'s real
/// `CpuGlyphRunBackend`/`RenderContext` implementations of the same public
/// traits. This is the one, single participant driving `glifo`'s real code;
/// nothing here stands in for a second participant's behavior - the only
/// external input is the real font file above.
struct NoopRenderer;

static BLACK_PAINT: PaintType = PaintType::Solid(BLACK);

impl glifo::DrawSink for NoopRenderer {
    fn set_transform(&mut self, _t: Affine) {}
    fn set_paint(&mut self, _paint: glifo::AtlasPaint) {}
    fn set_paint_transform(&mut self, _t: Affine) {}
    fn fill_path(&mut self, _path: &BezPath) {}
    fn fill_rect(&mut self, _rect: &Rect) {}
    fn push_clip_layer(&mut self, _clip: &BezPath) {}
    fn push_blend_layer(&mut self, _blend_mode: BlendMode) {}
    fn pop_layer(&mut self) {}
    fn width(&self) -> u16 {
        512
    }
    fn height(&self) -> u16 {
        512
    }
}

impl glifo::GlyphRenderer for NoopRenderer {
    type SavedState = ();
    fn save_state(&mut self) -> Self::SavedState {}
    fn restore_state(&mut self, _state: Self::SavedState) {}
    fn stroke_path(&mut self, _path: &BezPath) {}
    fn set_paint_image(&mut self, _image: Image) {}
    fn set_tint(&mut self, _tint: Option<Tint>) {}
    fn get_context_color(&self) -> AlphaColor<Srgb> {
        BLACK
    }
    fn current_paint(&self) -> &PaintType {
        &BLACK_PAINT
    }
    fn atlas_image_source(&self, atlas_slot: &glifo::AtlasSlot) -> ImageSource {
        ImageSource::opaque_id(ImageId::new(atlas_slot.page_index))
    }
    fn atlas_paint_transform(&self, atlas_slot: &glifo::AtlasSlot) -> Affine {
        Affine::translate((-(atlas_slot.x as f64), -(atlas_slot.y as f64)))
    }
}

/// The minimal `GlyphRunBackend` a real third-party consumer of `glifo`
/// would write (mirrors `vello_cpu`'s real `CpuGlyphRunBackend` in
/// `sparse_strips/vello_cpu/src/text.rs`).
struct MyBackend<'a> {
    atlas: &'a mut GlyphAtlas,
    image_cache: &'a mut ImageCache,
    prep_cache: &'a mut GlyphPrepCache,
    renderer: &'a mut NoopRenderer,
}

impl<'a> GlyphRunBackend<'a> for MyBackend<'a> {
    fn atlas_cache(self, _enabled: bool) -> Self {
        self
    }

    fn fill_glyphs<Glyphs>(self, run: GlyphRun<'a>, glyphs: Glyphs)
    where
        Glyphs: Iterator<Item = Glyph> + Clone,
    {
        let atlas_cacher = AtlasCacher::Enabled(self.atlas, self.image_cache);
        let mut glyph_run = run.build(glyphs, self.prep_cache.as_mut(), atlas_cacher);
        glyph_run.fill_glyphs(self.renderer);
    }

    fn stroke_glyphs<Glyphs>(self, _run: GlyphRun<'a>, _glyphs: Glyphs)
    where
        Glyphs: Iterator<Item = Glyph> + Clone,
    {
        unreachable!("not exercised by this repro")
    }

    fn render_decoration<Glyphs>(
        self,
        _run: GlyphRun<'a>,
        _glyphs: Glyphs,
        _x_range: std::ops::RangeInclusive<f32>,
        _baseline_y: f32,
        _offset: f32,
        _size: f32,
        _buffer: f32,
    ) where
        Glyphs: Iterator<Item = Glyph> + Clone,
    {
        unreachable!("not exercised by this repro")
    }
}

fn main() {
    let font = FontData::new(Blob::new(Arc::new(NOTO_COLR_FONT.to_vec())), 0);

    let mut atlas = GlyphAtlas::new(); // GlyphCacheConfig::default(): max_cached_font_size = 128.0
    let mut image_cache = ImageCache::new_with_config(AtlasConfig {
        atlas_size: (512, 512),
        ..AtlasConfig::default()
    });
    let mut prep_cache = GlyphPrepCache::default();
    let mut renderer = NoopRenderer;

    // Glyph id 3 in NotoColorEmoji-Subset.ttf is a real COLR glyph (used by
    // glifo's own `colr_glyph_is_cached_when_atlas_cache_is_enabled` unit
    // test, which asserts this exact glyph gets cached).
    let glyph = Glyph {
        id: 3,
        x: 0.0,
        y: 20.0,
    };

    let backend = MyBackend {
        atlas: &mut atlas,
        image_cache: &mut image_cache,
        prep_cache: &mut prep_cache,
        renderer: &mut renderer,
    };

    let transform = Affine::translate((0.0, 20.0));
    glifo::GlyphRunBuilder::new(font, transform, Affine::IDENTITY, backend)
        .font_size(16.0)
        .fill_glyphs(std::iter::once(glyph));

    println!(
        "cached entries after rendering one COLR glyph: {}",
        atlas.len()
    );
    assert_eq!(
        atlas.len(),
        1,
        "expected exactly one real COLR glyph cache entry"
    );

    // The real, public, debug-only diagnostics API - this is what panics.
    // `GlyphCacheStats`/`GlyphAtlas::stats` are gated
    // `#[cfg(all(debug_assertions, feature = "std"))]` in glifo itself, so
    // this only exists (and only panics) in a debug build - see RESULT.md.
    #[cfg(debug_assertions)]
    {
        let stats: GlyphCacheStats = atlas.stats(1);
        println!("did not reproduce: {stats:?}");
    }
    #[cfg(not(debug_assertions))]
    {
        println!(
            "GlyphAtlas::stats is debug_assertions-gated; run without --release to reproduce"
        );
    }
}
