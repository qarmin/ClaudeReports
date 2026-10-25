use egui::Context;
use egui::Hyperlink;
use egui::Ui;
use egui::UiBuilder;
use emath::Rect;

fn main() {
    let harness = <egui_kittest::Harness<u32>>::new_ui_state(|_, _| (), 0u32);
    let mut ui = Ui::new(Context::default(), harness.ui_id(), UiBuilder::new());
    let _ = ui.put(Rect::everything_below(1.0f32), Hyperlink::new("".to_string()));
}
