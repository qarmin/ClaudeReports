use emath::Pos2;
use epaint::ColorMode;
use epaint::CubicBezierShape;
use epaint::FontColorTransferFunction;
use epaint::PathStroke;
use epaint::StrokeKind;

fn main() {
    let pos2 = Pos2::default();
    let cubic_bezier_shape = CubicBezierShape { points: [pos2, pos2, pos2, pos2], closed: false, fill: FontColorTransferFunction::LIGHT_MODE_DEFAULT.color_from_coverage(0.0f32), stroke: PathStroke { width: 0.0f32, color: ColorMode::TRANSPARENT, kind: StrokeKind::Inside } };
    let _ = cubic_bezier_shape.split_range(0.0f32..256.0f32);
}
