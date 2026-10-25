use egui::Context;
use egui::Ui;
use egui::UiBuilder;
use epaint::FontFamily;
use epaint::FontId;

fn main() {
    let context = Context::default();
    let harness = <egui_kittest::Harness<u8>>::new_ui_state(|_, _| (), 0u8);
    let mut ui = Ui::new(context.clone(), harness.ui_id(), UiBuilder::new());
    let mut ui2 = ui.new_child(UiBuilder::new());
    let mut font_id = FontId::new(0.0f32, FontFamily::Monospace);
    egui::introspection::font_id_ui(&mut ui2, &mut font_id);
}
