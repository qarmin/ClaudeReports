use tiny_skia_path::NonZeroRect;

fn main() {
    let non_zero_rect = NonZeroRect::from_xywh(0.0f32, 0.0f32, 1.0f32, 1.1754944e-38f32).unwrap();
    let _ = non_zero_rect.bbox_transform(NonZeroRect::from_xywh(0.0f32, 1.0f32, 1.0f32, 1.0f32).unwrap());
}
