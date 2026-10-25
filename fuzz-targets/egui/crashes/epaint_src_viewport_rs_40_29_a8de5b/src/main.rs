use egui::Context;
use emath::Rect;
use epaint::PaintCallbackInfo;

fn main() {
    let paint_callback_info = PaintCallbackInfo { viewport: Rect::everything_above(0.0f32), clip_rect: Context::default().globally_used_rect(), pixels_per_point: 0.0f32, screen_size_px: [0u32, u32::MAX] };
    let _ = paint_callback_info.clip_rect_in_pixels();
}
