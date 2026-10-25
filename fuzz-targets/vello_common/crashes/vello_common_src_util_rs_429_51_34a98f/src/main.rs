use fearless_simd::Level;
use kurbo::Triangle;
use vello_common::clip::ClipContext;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_context = ClipContext::new();
    let mut strip_generator = StripGenerator::new(u16::MAX, 100u16, Level::baseline());
    clip_context.push_clip_rect(&<kurbo::Circle as kurbo::Shape>::bounding_box(&Triangle::ZERO.inscribed_circle()), &mut strip_generator);
}
