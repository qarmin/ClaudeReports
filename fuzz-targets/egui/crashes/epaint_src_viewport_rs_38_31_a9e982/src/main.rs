use emath::Rect;
use epaint::PaintCallbackInfo;

fn main() {
    let paint_callback_info = PaintCallbackInfo { viewport: Rect::everything_right_of(0.0f32), clip_rect: Rect::ZERO, pixels_per_point: 0.0f32, screen_size_px: [u32::MAX, 0u32] };
    let _ = paint_callback_info.clip_rect_in_pixels();
}
