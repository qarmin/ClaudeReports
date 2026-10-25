use ecolor::Color32;
use emath::Pos2;
use epaint::Shape;

fn main() {
    let _ = Shape::dotted_line(&[Pos2 { x: 0.0f32, y: 1.0f32 }, Pos2::from([0.0f32, 0.0f32])], Color32::from_black_alpha(0u8), 0.0f32, 0.0f32);
}
