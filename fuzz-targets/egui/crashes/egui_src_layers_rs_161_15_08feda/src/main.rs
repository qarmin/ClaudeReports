use ecolor::Color32;
use egui::Context;
use egui::layers::PaintList;
use epaint::Stroke;
use epaint::WHITE_UV;

fn main() {
    let mut paint_list = PaintList::default();
    paint_list.reset_shape(Context::default().debug_painter().circle_stroke(WHITE_UV, 0.0f32, Stroke::new(0.0f32, Color32::from_rgb(0u8, 0u8, 0u8))));
}
