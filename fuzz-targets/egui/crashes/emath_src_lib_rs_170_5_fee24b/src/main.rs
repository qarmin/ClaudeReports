use egui::PointerState;
use emath::Align2;
use emath::Pos2;
use emath::RectTransform;

fn main() {
    let align2 = Align2::CENTER_TOP;
    let rect = align2.anchor_size(Pos2 { x: 0.0f32, y: 0.0f32 }, PointerState::default().direction());
    let rect2 = rect.with_min_y(0.0f32);
    let _ = RectTransform::identity(rect2).transform_rect(rect);
}
