// Standalone reproduction for the integer overflow in
// hayro-jbig2/src/huffman_table.rs (`leaf.range_low + range_offset`).
//
// `crash_input.jb2` is a real, standalone JBIG2 embedded-stream file (not a PDF) - it is fed
// straight to hayro-jbig2's own public `Image::new`/`decode` entry point, exactly the same entry
// point a real caller (e.g. hayro's PDF JBIG2Decode filter, or any other consumer of this crate)
// would use for a malformed/adversarial JBIG2 stream. No second participant is faked here: a
// malformed file handed to a parser's own public decode entry point is the genuine external
// input, there is no "peer" being impersonated.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (bug present).

struct NullDecoder;

impl hayro_jbig2::Decoder for NullDecoder {
    fn push_pixel(&mut self, _black: bool) {}
    fn push_pixel_chunk(&mut self, _black: bool, _chunk_count: u32) {}
    fn next_line(&mut self) {}
}

fn main() {
    let data: &[u8] = include_bytes!("../crash_input.jb2");

    let result = std::panic::catch_unwind(|| {
        if let Ok(image) = hayro_jbig2::Image::new(data) {
            let mut decoder = NullDecoder;
            let _ = image.decode(&mut decoder);
        }
    });

    match result {
        Ok(_) => {
            println!("did not reproduce - decode completed without panicking");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
