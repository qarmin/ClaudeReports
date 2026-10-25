use egui::Context;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let _ = Ui::new(Context::default(), Id::unique("".to_string()), UiBuilder::new().id(egui::accesskit_root_id()));
}
