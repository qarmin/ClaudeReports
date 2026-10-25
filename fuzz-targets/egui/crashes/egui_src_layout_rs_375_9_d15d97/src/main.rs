use egui::Layout;
use emath::Align2;
use emath::Rect;
use emath::RectAlign;

fn main() {
    let vec2 = <emath::Vec2 as emath::GuiRounding>::round_to_pixel_center(RectAlign::TOP_START.child().to_sign(), 0.0f32);
    let _ = Layout::top_down_justified(Align2::CENTER_CENTER.y()).align_size_within_rect(vec2, Rect::ZERO);
}
