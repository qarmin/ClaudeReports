use tiny_skia::Mask;

fn main() {
    let mask = Mask::new(803485030u32, 1u32).unwrap();
    let _ = mask.encode_png();
}
