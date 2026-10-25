use emath::Align2;
use emath::Rect;
use epaint::WHITE_UV;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    path.add_line_loop(&[WHITE_UV, Align2::LEFT_BOTTOM.pos_in_rect(&Rect::everything_above(1.0f32))]);
}
