use vello_common::pixmap::Pixmap;

fn main() {
    let mut pixmap = Pixmap::new(0u16, 0u16);
    pixmap.resize(10000u16, u16::MAX);
}
