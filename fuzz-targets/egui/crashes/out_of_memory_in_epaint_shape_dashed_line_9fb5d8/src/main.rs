use emath::Pos2;
use emath::Rect;
use epaint::Shape;
use epaint::Stroke;

fn main() {
    let _ = Shape::dashed_line(&[Pos2 { x: 0.0f32, y: 0.0f32 }, Rect::EVERYTHING.left_top()], Stroke::NONE, 0.0f32, 1.0f32);
}
