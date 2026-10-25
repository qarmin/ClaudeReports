use egui::Context;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::new("".to_string()), UiBuilder::default());
    ui.add_space(f32::NAN);
    ui.shrink_height_to_current();
}
