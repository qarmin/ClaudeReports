use egui::Context;
use egui::Id;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Vec2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::new(0u8), UiBuilder::default());
    let _ = ui.allocate_response(Vec2::LEFT, Sense::CLICK);
}
