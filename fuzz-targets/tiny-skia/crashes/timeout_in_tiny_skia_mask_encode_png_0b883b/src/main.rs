use tiny_skia::Mask;

fn main() {
    let mask = Mask::new(1u32, 100000000u32).unwrap();
    let _ = mask.encode_png();
}
