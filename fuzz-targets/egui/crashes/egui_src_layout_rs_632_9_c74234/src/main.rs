use egui::Context;
use egui::Id;
use egui::Layout;
use egui::Ui;
use egui::UiBuilder;
use egui_extras::Size;
use emath::Align;

fn main() {
    let context = Context::default();
    let mut ui = Ui::new(context.clone(), Id::NULL, UiBuilder::default().layout(Layout::left_to_right(Align::Max)));
    ui.set_height_range(Size::relative(1.0f32).range());
}
