use emath::Pos2;
use emath::Rect;
use epaint::tessellator::Path;

fn main() {
    let pos2 = Pos2::default();
    let mut path = Path::default();
    path.add_open_points(&[pos2, Rect::everything_left_of(0.0f32).left_top(), pos2]);
}
