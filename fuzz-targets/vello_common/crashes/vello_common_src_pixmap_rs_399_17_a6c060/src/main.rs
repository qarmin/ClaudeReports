use vello_common::pixmap::Pixmap;

fn main() {
    let pixmap = Pixmap::new(0u16, 0u16);
    let _ = pixmap.sample_idx(0u32);
}
