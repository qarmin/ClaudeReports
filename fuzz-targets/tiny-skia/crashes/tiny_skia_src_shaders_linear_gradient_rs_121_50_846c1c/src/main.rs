use tiny_skia::Color;
use tiny_skia::ColorU8;
use tiny_skia::GradientStop;
use tiny_skia::LinearGradient;
use tiny_skia::SpreadMode;
use tiny_skia_path::Point;
use tiny_skia_path::Transform;

fn main() {
    let color_u8 = ColorU8::from_rgba(0u8, 0u8, 0u8, 0u8);
    let u8_v = color_u8.alpha();
    let color = Color::BLACK;
    let _ = LinearGradient::new(Point::zero(), Point::default(), vec![GradientStop::new(1.0f32, color), GradientStop::new(0.0f32, Color::from_rgba8(u8_v, 0u8, 0u8, 0u8)), GradientStop::new(0.0f32, Color::WHITE)], SpreadMode::Reflect, Transform::default());
}
