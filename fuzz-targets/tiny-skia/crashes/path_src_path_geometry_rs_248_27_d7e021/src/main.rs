use tiny_skia_path::LineCap;
use tiny_skia_path::LineJoin;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::PathStroker;
use tiny_skia_path::Stroke;
use tiny_skia_path::Transform;

fn main() {
    let line_cap = LineCap::default();
    let mut path_stroker = PathStroker::new();
    let _ = path_stroker.stroke(&PathBuilder::from_circle(0.0f32, 0.0f32, 1.0f32).unwrap().transform(Transform::from_scale(-1e30f32, 4.5584144e30f32)).unwrap(), &Stroke { width: 1.0f32, miter_limit: 0.0f32, line_cap: line_cap, line_join: LineJoin::Round, dash: None }, 0.0f32);
}
