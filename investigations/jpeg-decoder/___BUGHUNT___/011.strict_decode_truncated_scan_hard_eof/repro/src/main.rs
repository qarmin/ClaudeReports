// Real-world truncated/corrupted JPEG (image-rs/jpeg-decoder issue #251's own attachment) fed
// straight into the crate's real public API (`Decoder::new` + `decode`), exactly the same path a
// real caller (an image viewer, the `image` crate, etc.) uses to open an arbitrary file from disk.
// This is a single-participant repro (a malformed file handed to a parser's own public entry
// point) - CONFIRMATION.md section 11's exception applies directly, there is no second
// participant to fake here.
//
// Exit code convention: 0 = did not reproduce (decode succeeded, bug fixed), non-zero = reproduced
// (decode returned the hard `UnexpectedEof`/"failed to fill whole buffer" I/O error).

use std::io::BufReader;

fn main() {
    let bytes: &[u8] = include_bytes!("../fixtures/gh251-failed-to-fill-whole-buffer.jpg");
    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(bytes));

    match decoder.decode() {
        Ok(pixels) => {
            let info = decoder.info().unwrap();
            let expected_len =
                info.width as usize * info.height as usize * info.pixel_format.pixel_bytes();
            println!(
                "did not reproduce - decode() succeeded: {}x{} {:?}, pixels.len()={} (expected {})",
                info.width, info.height, info.pixel_format, pixels.len(), expected_len
            );
            if pixels.len() != expected_len {
                eprintln!("WRONG OUTPUT: decoded buffer size does not match image dimensions");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("REPRODUCED - decode() returned a hard error: {}", e);
            std::process::exit(1);
        }
    }
}
