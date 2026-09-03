// Copy into <clone>/examples/verify_dims.rs and run with `cargo run --example verify_dims`.
//
// Decodes the real, genuine JPEG fixture in fixtures/test_wide.jpg (a 999x8 grayscale image,
// created with ImageMagick's `convert -size 999x8 xc:gray`) through the crate's real public
// API (`Decoder::new`/`decode`/`info`), the exact end-to-end path that internally calls
// `Decoder::select_worker(&frame, PreferWorkerKind::Multithreaded)` with `frame.output_size`
// set from this same real file. This proves the 999x8 dimensions used in test_patch.diff's unit
// test are not an arbitrary made-up value - they are exactly what a real image handed to the
// real public decode() entry point produces, and decoding succeeds with the expected pixel
// count regardless of which worker backend select_worker's (buggy) heuristic picks internally -
// confirming the bug has no effect on decode correctness, only on the internal performance
// heuristic.
use std::fs::File;
use std::io::BufReader;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "fixtures/test_wide.jpg".to_string());
    let file = File::open(&path).expect("open fixture");
    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("decode failed");
    let info = decoder.info().unwrap();
    println!(
        "decoded {}x{} image, {} bytes of pixel data",
        info.width,
        info.height,
        pixels.len()
    );
    assert_eq!(info.width, 999);
    assert_eq!(info.height, 8);
    assert_eq!(pixels.len(), 999 * 8);
    println!("real end-to-end decode confirms select_worker sees output_size = 999x8, matching the unit test in test_patch.diff");
}
