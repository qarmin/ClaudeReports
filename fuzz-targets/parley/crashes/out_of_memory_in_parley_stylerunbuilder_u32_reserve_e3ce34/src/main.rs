use parley::FontContext;

fn main() {
    let mut layout_context = <parley::LayoutContext<u32> as ::core::default::Default>::default();
    let mut font_context = FontContext::default();
    let mut style_run_builder = layout_context.style_run_builder(&mut font_context, "", 0.0f32, false);
    style_run_builder.reserve(0usize, 1000000000usize);
}
