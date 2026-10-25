use tiny_skia_path::PathBuilder;
use tiny_skia_path::PathStroker;
use tiny_skia_path::Rect;
use tiny_skia_path::Stroke;

fn main() {
    let mut path_stroker = PathStroker::new();
    let _ = path_stroker.stroke(&PathBuilder::from_rect(Rect::from_xywh(0.0f32, 0.0f32, 0.0f32, 0.0f32).unwrap()), &Stroke::default(), -74.0f32);
}
