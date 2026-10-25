use tiny_skia_path::IntRect;

fn main() {
    let int_rect = IntRect::from_xywh(0i32, 1i32, 1u32, 1u32).unwrap();
    let _ = int_rect.inset(0i32, i32::MAX);
}
