use egui::Context;
use egui::ViewportId;

fn main() {
    let context = Context::default();
    let _ = context.cumulative_frame_nr_for(ViewportId::from_hash_of(0u8));
}
