use emath::Rect;

fn main() {
    epaint::tessellator::path::add_circle_quadrant(&mut vec![], Rect::everything_right_of(0.0f32).left_top(), 65536.0f32, f32::MAX);
}
