use egui::Context;
use egui::Hyperlink;
use egui::Id;
use egui::Ui;
use egui::UiBuilder;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::unique(0u8), UiBuilder::default());
    let _ = ui.add(Hyperlink::new("".to_string()));
}
