use tiny_skia_path::NonZeroRect;
use tiny_skia_path::Rect;

fn main() {
    let _ = Rect::from_xywh(f32::MAX, 0.0f32, 0.0f32, 0.0f32).unwrap().bbox_transform(NonZeroRect::from_xywh(0.0f32, 0.0f32, 1000.0f32, 1.0f32).unwrap());
}
