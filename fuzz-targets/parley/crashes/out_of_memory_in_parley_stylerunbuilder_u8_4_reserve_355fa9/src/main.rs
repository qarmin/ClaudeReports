use fontique::Collection;
use fontique::SourceCache;
use fontique::SourceCacheOptions;
use parley::FontContext;

fn main() {
    let mut layout_context = <parley::LayoutContext<[u8; 4]> as ::core::default::Default>::default();
    let mut font_context = FontContext { collection: Collection::default(), source_cache: SourceCache::new(SourceCacheOptions::default()) };
    let mut style_run_builder = layout_context.style_run_builder(&mut font_context, "", 0.0f32, false);
    style_run_builder.reserve(0usize, 1000000000usize);
}
