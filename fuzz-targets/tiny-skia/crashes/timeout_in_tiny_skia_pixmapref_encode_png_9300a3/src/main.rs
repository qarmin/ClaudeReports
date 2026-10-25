use tiny_skia::Pixmap;

fn main() {
    let pixmap = Pixmap::new(1u32, 100000000u32).unwrap();
    let pixmap_ref = pixmap.as_ref();
    let _ = pixmap_ref.encode_png();
}
