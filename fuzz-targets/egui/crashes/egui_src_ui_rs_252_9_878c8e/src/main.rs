use egui::Context;
use egui::Hyperlink;
use egui::Ui;
use egui::UiBuilder;
use egui::ViewportId;
use emath::Rect;

fn main() {
    let context = Context::default();
    let mut ui = Ui::new(context.clone(), ViewportId::ROOT.root_ui_id(), UiBuilder::default());
    let _ = ui.place(Rect::everything_above(0.0f32).with_max_x(f32::NAN), Hyperlink::new("".to_string()));
}
