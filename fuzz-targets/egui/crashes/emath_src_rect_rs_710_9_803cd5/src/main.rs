use emath::Pos2;
use emath::Rect;
use epaint::MarginF32;

fn main() {
    let margin_f32 = MarginF32::ZERO;
    let rect = Rect::everything_below(0.0f32);
    let _ = rect.intersects_ray(Pos2::ZERO, margin_f32.left_top());
}
