use ecolor::Color32;
use emath::Pos2;
use epaint::CubicBezierShape;
use epaint::PathStroke;

fn main() {
    let cubic_bezier_shape = CubicBezierShape::from(CubicBezierShape::from_points_stroke([Pos2::from((0.0f32, 0.0f32)), Pos2::ZERO, Pos2::from([1.0f32, 1.0f32]), Pos2::default()], false, Color32::BROWN, PathStroke::default()));
    let _ = cubic_bezier_shape.to_path_shapes(None, None);
}
