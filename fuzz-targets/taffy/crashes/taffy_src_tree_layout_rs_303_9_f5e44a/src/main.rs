use taffy::prelude::AlignItems;
use taffy::tree::AxisStaticPosition;

fn main() {
    let _ = AxisStaticPosition::from_alignment(AlignItems::SAFE_SELF_START, <taffy::geometry::Line<f32> as taffy::prelude::FromLength>::from_length::<f64>(1.0f64), false);
}
