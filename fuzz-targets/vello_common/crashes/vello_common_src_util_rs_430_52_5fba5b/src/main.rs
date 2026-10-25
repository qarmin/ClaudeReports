use fearless_simd::Level;
use kurbo::Rect;
use vello_common::clip::ClipContext;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_context = ClipContext::default();
    let mut strip_generator = StripGenerator::new(1u16, u16::MAX, Level::baseline());
    clip_context.push_clip_rect(&Rect { x0: 1.0f64, y0: 960000.0f64, x1: 0.0f64, y1: 0.0f64 }, &mut strip_generator);
}
