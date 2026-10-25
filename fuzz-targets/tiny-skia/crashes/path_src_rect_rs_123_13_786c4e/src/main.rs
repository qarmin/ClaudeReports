use tiny_skia_path::IntRect;

fn main() {
    let int_rect = IntRect::from_xywh(1i32, 0i32, 1u32, 1u32).unwrap();
    let _ = int_rect.inset(i32::MAX, 0i32);
}
