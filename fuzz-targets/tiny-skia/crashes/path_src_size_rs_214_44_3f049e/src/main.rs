use tiny_skia_path::IntSize;
use tiny_skia_path::Size;

fn main() {
    let size = Size::from_wh(f32::MAX, 1.0f32).unwrap();
    let _ = size.expand_to(IntSize::from_wh(1u32, 100000000u32).unwrap().to_size());
}
