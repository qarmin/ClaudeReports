use egui::Context;
use egui::Ui;
use egui::UiBuilder;
use egui::ViewportId;
use egui::style::Interaction;
use emath::Rect;

fn main() {
    let mut interaction = Interaction::default();
    let mut ui = Ui::new(Context::default(), ViewportId::from_hash_of(0u8).root_ui_id(), UiBuilder::default().max_rect(Rect::EVERYTHING));
    let mut ui2 = ui.new_child(UiBuilder::new());
    interaction.ui(&mut ui2);
}
