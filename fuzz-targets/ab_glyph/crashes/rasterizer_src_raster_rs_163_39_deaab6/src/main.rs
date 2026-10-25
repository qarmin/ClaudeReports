use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let point = <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(Point::default(), Point { x: 255.0f32, y: -53.3f32 });
    let mut rasterizer = Rasterizer::new(100usize, 100usize);
    rasterizer.draw_cubic(point, Point { x: 0.0f32, y: 0.0f32 }, ab_glyph_rasterizer::point(f32::NAN, 0.0f32), Point { x: 0.0f32, y: 256.0f32 });
}
