use tiny_skia::Pixmap;

fn main() {
    let pixmap = Pixmap::new(1u32, 100000000u32).unwrap();
    let _ = pixmap.encode_png();
}
