use tiny_skia_path::IntRect;

fn main() {
    let int_rect = IntRect::from_ltrb(0i32, -1000000000i32, 1i32, 0i32).unwrap();
    let _ = int_rect.translate(0i32, i32::MIN);
}
