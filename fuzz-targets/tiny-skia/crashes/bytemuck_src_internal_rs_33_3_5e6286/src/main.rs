use tiny_skia::PixmapRef;

fn main() {
    let vec = vec![50u8, 50u8, 50u8, 50u8, 50u8];
    let pixmap_ref = PixmapRef::from_bytes(&vec, 1u32, 1u32).unwrap();
    let pixmap = pixmap_ref.to_owned();
    let pixmap_ref2 = pixmap.as_ref();
    let _ = pixmap_ref2.pixel(0u32, 0u32);
}
