use egui::Context;
use egui::Ui;
use egui::UiBuilder;
use egui::ViewportId;

fn main() {
    let mut ui = Ui::new(Context::default(), ViewportId::from_hash_of(0u8).root_ui_id(), UiBuilder::new());
    ui.set_height(-3.5527137e-15f32);
}
