use tiny_skia_path::IntSize;

fn main() {
    let int_size = IntSize::from_wh(1u32, u32::MAX).unwrap();
    let _ = int_size.to_int_rect(0i32, 0i32);
}
