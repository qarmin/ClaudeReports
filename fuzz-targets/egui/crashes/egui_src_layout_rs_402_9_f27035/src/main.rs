use egui::Context;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;
use emath::Rect;

fn main() {
    let _ = Ui::new(Context::default(), Id::new("".to_string()), UiBuilder::default().max_rect(Rect::NAN));
}
