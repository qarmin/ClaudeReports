use tiny_skia_path::NonZeroRect;

fn main() {
    let _ = NonZeroRect::from_xywh(0.0f32, 0.0f32, 1.0f32, 10000000000.0f32).unwrap().to_int_rect();
}
