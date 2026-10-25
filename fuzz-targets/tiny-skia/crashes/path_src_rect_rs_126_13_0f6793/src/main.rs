use tiny_skia_path::IntRect;

fn main() {
    let int_rect = IntRect::from_xywh(0i32, 0i32, 1u32, 1u32).unwrap();
    let int_rect2 = int_rect.translate_to(0i32, 0i32).unwrap();
    let _ = int_rect2.inset(0i32, i32::MIN);
}
