use vello_common::pixmap::Pixmap;

fn main() {
    let pixmap = Pixmap::new(1u16, 0u16);
    let _ = pixmap.sample(0u16, 0u16);
}
