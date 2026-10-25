use egui::AreaState;

fn main() {
    epaint::tessellator::path::add_circle_quadrant(&mut vec![], AreaState::default().left_top_pos(), 3.0f32, 467.0f32);
}
