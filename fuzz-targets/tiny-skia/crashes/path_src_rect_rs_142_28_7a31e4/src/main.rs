use tiny_skia_path::IntRect;

fn main() {
    let int_rect = IntRect::from_ltrb(1i32, 0i32, 10000i32, 1i32).unwrap();
    let _ = int_rect.translate(i32::MAX, 0i32);
}
