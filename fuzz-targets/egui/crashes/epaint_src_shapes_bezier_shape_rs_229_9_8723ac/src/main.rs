use ecolor::Color32;
use emath::Pos2;
use epaint::CubicBezierShape;
use epaint::PathStroke;

fn main() {
    let pos2 = Pos2::default();
    let cubic_bezier_shape = CubicBezierShape { points: [Pos2 { x: 0.0f32, y: 0.0f32 }, pos2, pos2, pos2], closed: false, fill: Color32::from_black_alpha(0u8), stroke: PathStroke::default() };
    let _ = cubic_bezier_shape.sample(10000000000.0f32);
}
