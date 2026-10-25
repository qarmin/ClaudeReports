use egui::Context;
use egui::Ui;
use egui::UiBuilder;
use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let harness = <egui_kittest::Harness<u32>>::new_ui_state(|_, _| (), 0u32);
    let _ = Ui::new(Context::default(), harness.ui_id(), UiBuilder::new().max_rect(Mesh::with_texture(TextureId::User(0u64)).calc_bounds()));
}
