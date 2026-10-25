use tiny_skia_path::PathBuilder;
use tiny_skia_path::PathStroker;
use tiny_skia_path::Stroke;

fn main() {
    let mut path_builder = PathBuilder::default();
    path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    path_builder.cubic_to(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32);
    let _ = PathStroker::default().stroke(&path_builder.clone().finish().unwrap(), &Stroke::default(), 1.0f32);
}
