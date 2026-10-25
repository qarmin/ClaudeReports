use egui::Context;
use egui::Id;
use egui::InteractOptions;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Rect;

fn main() {
    let context = Context::default();
    let ui = Ui::new(context.clone(), Id::new(0u32), UiBuilder::default());
    let _ = ui.interact_opt(Rect::NAN, Context::default().viewport_id().root_ui_id(), Sense::CLICK, InteractOptions { move_to_top: false });
}
