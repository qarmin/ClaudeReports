use emath::Pos2;
use emath::Rect;
use emath::Vec2;

fn main() {
    let rect = Rect::from_min_max(Pos2 { x: 0.0f32, y: 0.0f32 }, Pos2::ZERO);
    let _ = rect.intersects_ray_from_center(Vec2::default());
}
