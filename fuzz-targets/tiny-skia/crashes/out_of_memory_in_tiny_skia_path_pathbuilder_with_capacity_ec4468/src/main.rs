use tiny_skia_path::PathBuilder;
use tiny_skia_path::Point;
use tiny_skia_path::path_geometry::CubicCoeff;

fn main() {
    let point = Point::from((1.0f32, 0.0f32));
    let _ = CubicCoeff { a: tiny_skia_path::f32x2::splat(0.0f32), b: point.to_f32x2(), c: tiny_skia_path::f32x2::new(0.0f32, 0.0f32), d: PathBuilder::with_capacity(100000000000000usize, 0usize).last_point().unwrap().to_f32x2() };
}
