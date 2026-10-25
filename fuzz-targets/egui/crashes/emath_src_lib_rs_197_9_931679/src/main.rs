use egui::Context;
use egui::PointerState;
use emath::Align2;
use emath::Pos2;
use emath::RectTransform;

fn main() {
    let rect_transform = RectTransform::identity(Align2::CENTER_CENTER.align_size_within_rect(PointerState::default().velocity(), Context::default().content_rect()));
    let _ = rect_transform.transform_pos_clamped(Pos2::NAN);
}
