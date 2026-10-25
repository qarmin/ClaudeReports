use parley::FontContext;

fn main() {
    let mut layout_context = <parley::LayoutContext<u8> as ::core::default::Default>::default();
    let mut font_context = FontContext::new();
    let mut style_run_builder = layout_context.style_run_builder(&mut font_context, "", 1.0f32, false);
    style_run_builder.reserve(1000000000usize, 0usize);
}
