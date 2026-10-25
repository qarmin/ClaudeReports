use egui::Visuals;
use epaint::Shape;

fn main() {
    let _ = Shape::dashed_line_with_offset(&[], Visuals::default().window_stroke(), &[], &[0.0f32], 0.0f32);
}
