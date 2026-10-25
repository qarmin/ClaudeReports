use tiny_skia_path::Size;

fn main() {
    let size = Size::from_wh(1.0f32, 1.1754944e-38f32).unwrap();
    let _ = size.to_non_zero_rect(0.0f32, 1.0f32);
}
