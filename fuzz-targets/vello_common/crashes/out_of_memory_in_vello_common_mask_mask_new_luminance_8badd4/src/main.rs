use vello_common::mask::Mask;
use vello_common::pixmap::Pixmap;

fn main() {
    let _ = Mask::new_luminance(&Pixmap::new(u16::MAX, 10000u16));
}
