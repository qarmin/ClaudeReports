// Standalone reproduction for GitHub issue #587: `index out of bounds: the len is 4809 but the
// index is 4809` in rawler/src/pumps.rs, hit while decoding a CR2 (LJPEG) raw file.
//
// This calls rawler's real public LJPEG entry point directly:
//   rawler::decompressors::ljpeg::LjpegDecompressor::new(src)  ->  .decode(...)
// which is *exactly* the same call Cr2Decoder::raw_image makes (see cr2.rs: `LjpegDecompressor::
// new(src)?` then `decompressor.decode(...)`), and matches the original crash's own stack trace
// frame-for-frame:
//   HuffTable::huff_decode -> decode_ljpeg -> LjpegDecompressor::decode -> Cr2Decoder::raw_image
//
// The `src` byte buffer built below is a minimal but spec-shaped LJPEG bitstream (SOI, SOF3, DHT,
// SOS, followed by a 3-byte truncated entropy-coded segment ending in 0xFF) - the same kind of
// bytes a real (corrupted/truncated) CR2 file's raw strip data would contain and that
// `Cr2Decoder::raw_image` hands to `LjpegDecompressor` unvalidated (`file.subview_until_eof
// (offset)` reads to the physical end of the file with no length check). This is a single
// external artifact handed to its real public parsing entry point - no second participant is
// being faked here, per CONFIRMATION.md section 11.
//
// Root cause: BitPumpJPEG::peek_bits's slow-refill loop (rawler/src/pumps.rs) reads
// `self.buffer[self.pos]`, finds it's 0xFF (the JPEG byte-stuffing marker byte), and then
// unconditionally indexes `self.buffer[self.pos + 1]` to check whether it's the stuffed 0x00
// byte - without checking that `pos + 1` is still inside the buffer. If the entropy-coded segment
// ends in a bare (non-stuffed) 0xFF as its very last byte, `pos + 1 == buffer.len()` and the
// index panics.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use rawler::decompressors::ljpeg::LjpegDecompressor;

fn build_truncated_ljpeg_stream() -> Vec<u8> {
    let mut buf = Vec::new();

    // SOI
    buf.extend_from_slice(&[0xFF, 0xD8]);

    // SOF3 (lossless): precision=10, height=1, width=4, 1 component (id=1, no subsampling)
    let sof_payload: Vec<u8> = {
        let mut p = Vec::new();
        p.push(10u8); // precision
        p.extend_from_slice(&1u16.to_be_bytes()); // height
        p.extend_from_slice(&4u16.to_be_bytes()); // width
        p.push(1u8); // components
        p.extend_from_slice(&[1u8, 0x11, 0u8]); // id, subsampling, (unused) quant table id
        p
    };
    buf.extend_from_slice(&[0xFF, 0xC3]);
    buf.extend_from_slice(&((2 + sof_payload.len()) as u16).to_be_bytes());
    buf.extend_from_slice(&sof_payload);

    // DHT: table 0, a single 1-bit code (bits[1] = 1), huffval[0] = 0
    let dht_payload: Vec<u8> = {
        let mut p = Vec::new();
        p.push(0x00u8); // table class/id
        let mut bits = [0u8; 16];
        bits[0] = 1; // one code of length 1
        p.extend_from_slice(&bits);
        p.push(0u8); // huffval[0]
        p
    };
    buf.extend_from_slice(&[0xFF, 0xC4]);
    buf.extend_from_slice(&((2 + dht_payload.len()) as u16).to_be_bytes());
    buf.extend_from_slice(&dht_payload);

    // SOS: 1 component (cs=1, td=0), predictor=1, Se/Ah unused, point transform=0
    let sos_payload: Vec<u8> = vec![1u8, 1u8, 0x00u8, 1u8, 0u8, 0u8];
    buf.extend_from_slice(&[0xFF, 0xDA]);
    buf.extend_from_slice(&((2 + sos_payload.len()) as u16).to_be_bytes());
    buf.extend_from_slice(&sos_payload);

    // Entropy-coded data: truncated to exactly 3 bytes, the last of which is a bare 0xFF.
    // BitPumpJPEG::peek_bits's slow-refill loop reads this last byte, sees 0xFF, and then
    // indexes one past the end of the buffer to check for the 0x00 stuffing byte.
    buf.extend_from_slice(&[0x00, 0x00, 0xFF]);

    buf
}

fn main() {
    let src = build_truncated_ljpeg_stream();
    println!("built {}-byte truncated LJPEG stream (entropy segment: {:02x?})", src.len(), &src[src.len() - 3..]);

    let result = std::panic::catch_unwind(|| {
        let decompressor = LjpegDecompressor::new(&src).expect("well-formed SOI/SOF3/DHT/SOS headers must parse");
        let width = decompressor.width();
        let height = decompressor.height();
        let mut out = vec![0u16; width * height];
        // Mirrors Cr2Decoder::raw_image's own call shape exactly.
        let _ = decompressor.decode(&mut out, 0, width, width, height, false);
    });

    match result {
        Ok(()) => {
            println!("did not reproduce - LjpegDecompressor::decode completed (or returned Err) without panicking");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
