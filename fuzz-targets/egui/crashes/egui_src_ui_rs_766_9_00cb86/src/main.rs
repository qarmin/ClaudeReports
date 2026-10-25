use egui::Context;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::unique("".to_string()), UiBuilder::new());
    ui.set_min_width(-9.67e-31f32);
}
