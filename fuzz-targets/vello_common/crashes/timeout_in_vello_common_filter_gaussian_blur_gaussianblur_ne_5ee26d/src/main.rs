use vello_common::filter::gaussian_blur::GaussianBlur;
use vello_common::filter_effects::EdgeMode;

fn main() {
    let _ = GaussianBlur::new(1e30f32, EdgeMode::None);
}
