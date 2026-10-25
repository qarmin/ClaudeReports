use tiny_skia::Mask;
use tiny_skia::Pixmap;

fn main() {
    let mut pixmap = Pixmap::new(1u32, 1u32).unwrap();
    let mut pixmap_mut = pixmap.as_mut();
    pixmap_mut.apply_mask(&Mask::new(1000000000u32, 8u32).unwrap());
}
