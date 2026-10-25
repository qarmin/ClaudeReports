use egui::Context;
use egui::Layout;
use emath::Align;
use emath::Vec2;
use emath::Vec2b;

fn main() {
    let _ = Layout::top_down_justified(Align::TOP).align_size_within_rect(Vec2::from(Vec2b::TRUE), Context::default().globally_used_rect());
}
