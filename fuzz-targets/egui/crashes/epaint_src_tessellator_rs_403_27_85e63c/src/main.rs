use egui::AreaState;
use emath::Pos2;
use emath::Rect;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    path.add_open_points(&[AreaState::default().left_top_pos(), Pos2::new(0.0f32, 1.0f32), Rect::everything_below(0.0f32).right_center()]);
}
