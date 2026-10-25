use tiny_skia_path::NonZeroRect;
use tiny_skia_path::Transform;

fn main() {
    let non_zero_rect = NonZeroRect::from_ltrb(0.0f32, f32::MIN, 1.0f32, -1e30f32).unwrap();
    let _ = non_zero_rect.transform(Transform::from_skew(0.0f32, 1.0f32));
}
