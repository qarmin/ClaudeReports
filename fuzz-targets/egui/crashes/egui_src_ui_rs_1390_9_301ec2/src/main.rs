use egui::Context;
use egui::Id;
use egui::PopupAnchor;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use emath::Pos2;

fn main() {
    let mut ui = Ui::new(Context::default(), Id::new(0u32), UiBuilder::default());
    let _ = ui.allocate_rect(PopupAnchor::from(Pos2::NAN).rect(egui::accesskit_root_id(), &Context::default()).unwrap(), Sense::from_bits(0u8).unwrap());
}
