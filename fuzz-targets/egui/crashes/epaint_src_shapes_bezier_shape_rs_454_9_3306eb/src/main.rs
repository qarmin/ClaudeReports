use ecolor::Color32;
use emath::Align2;
use emath::Pos2;
use emath::Rect;
use epaint::PathStroke;
use epaint::QuadraticBezierShape;

fn main() {
    let align2 = Align2::RIGHT_TOP;
    let quadratic_bezier_shape = QuadraticBezierShape { points: [Pos2 { x: 0.0f32, y: 0.0f32 }, align2.pos_in_rect(&Rect::everything_above(0.0f32)), Pos2::new(0.0f32, 0.0f32)], closed: false, fill: Color32::ORANGE, stroke: PathStroke::default() };
    let _ = quadratic_bezier_shape.sample(10000000000.0f32);
}
