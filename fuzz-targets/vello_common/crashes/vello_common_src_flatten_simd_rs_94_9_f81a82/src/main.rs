use fearless_simd::Level;
use kurbo::Affine;
use kurbo::PathEl;
use peniko::Fill;
use vello_common::clip::ClipContext;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_context = ClipContext::default();
    let mut strip_generator = StripGenerator::new(0u16, 0u16, Level::new());
    clip_context.push_clip_path(vec![PathEl::ClosePath], &mut strip_generator, Fill::NonZero, Affine::scale_non_uniform(0.0f64, 0.0f64), None);
}
