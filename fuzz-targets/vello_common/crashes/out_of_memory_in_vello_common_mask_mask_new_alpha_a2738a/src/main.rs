use vello_common::mask::Mask;
use vello_common::pixmap::Pixmap;

fn main() {
    let _ = Mask::new_alpha(&Pixmap::new(u16::MAX, 8368u16));
}
