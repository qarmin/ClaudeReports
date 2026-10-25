use egui::Context;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;
use emath::Vec2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::NULL, UiBuilder::new());
    ui.set_max_size(Vec2::from([0.0f32, f32::MIN]));
}
