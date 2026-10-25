use tiny_skia_path::NonZeroRect;
use tiny_skia_path::Size;

fn main() {
    let non_zero_rect = NonZeroRect::from_ltrb(0.0f32, 0.0f32, 1.1920929e-7f32, 1.0f32).unwrap();
    let size = Size::from_wh(1.0f32, 1e-40f32).unwrap();
    let _ = size.scale_to(non_zero_rect.size());
}
