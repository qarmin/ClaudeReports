use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    let normalized_f32_exclusive = NormalizedF32Exclusive::ANY;
    let point = <tiny_skia_path::Point as ::core::ops::Add<tiny_skia_path::Point>>::add(Point::from_xy(0.0f32, 0.0f32), Point::from_xy(0.0f32, 0.0f32));
    tiny_skia_path::path_geometry::chop_cubic_at2(&[point, point, point, point], normalized_f32_exclusive, &mut vec![point, point, point, point, point]);
}
