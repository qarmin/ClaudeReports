// Standalone reproduction for AVIF_10BIT_UNALIGNED_BUFFER_ALLOC_UNACCOUNTED.
//
// Real, end-to-end trigger through the crate's own public API: `image::codecs::avif::AvifDecoder`
// (the real dav1d-backed decoder), fed a real AVIF file (produced by ffmpeg/libaom, embedded
// below), decoded via `AvifDecoder::read_image` with a caller-supplied output buffer that is
// deliberately misaligned to 2 bytes (a legitimate, reachable use of the public `read_image`/
// `decode_into`-style API: nothing requires the caller's buffer to start on an even address).
//
// This is a single-participant finding (a caller configuring `Limits` on the decoder it itself
// owns, then calling that same decoder's own public method) - CONFIRMATION.md section 11's "real
// second participant" requirement doesn't add anything beyond "run the real public API
// end-to-end", which this repro does.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::{ImageDecoder, Limits};
use std::io::Cursor;

const AVIF_10BIT: &[u8] = include_bytes!("../test10bit.avif");

fn main() {
    let mut decoder = image::codecs::avif::AvifDecoder::new(Cursor::new(AVIF_10BIT))
        .expect("failed to construct AvifDecoder from embedded 10-bit AVIF test file");

    // Configure the decoder's own Limits directly (the real, documented `ImageDecoder::set_limits`
    // contract) with a zero allocation budget - the strictest possible request that "no further
    // decoder-owned allocation may happen".
    let mut limits = Limits::no_limits();
    limits.max_alloc = Some(0);
    decoder
        .set_limits(limits)
        .expect("set_limits itself should not fail for a supported strict-limit-free request");

    let prepared = decoder.prepare_image().expect("prepare_image failed");
    let total_bytes = prepared.total_bytes() as usize;
    println!(
        "dimensions: {:?}, color: {:?}, total_bytes: {total_bytes}",
        prepared.layout.dimensions(),
        prepared.layout.color
    );

    // Force a misaligned (odd-address) output buffer: a sub-slice starting at offset 1 of a
    // larger allocation. This is a legitimate value a caller of the public API can pass to
    // `read_image` (e.g. via `decode_into` with a buffer view into a larger arena/mmap region);
    // nothing in the public API contract requires 2-byte alignment.
    let mut backing = vec![0u8; total_bytes + 8];
    let buf = &mut backing[1..1 + total_bytes];
    assert_ne!(
        buf.as_ptr() as usize % 2,
        0,
        "test setup error: buffer unexpectedly ended up 2-byte aligned"
    );

    match decoder.read_image(buf) {
        Ok(_) => {
            eprintln!(
                "REPRODUCED: read_image() succeeded despite Limits{{max_alloc: Some(0)}} - it \
                 silently allocated a {total_bytes}-byte `aligned_store` fallback buffer \
                 (src/codecs/avif/decoder.rs's 10/12-bit `read_image` path) with no call to \
                 `Limits::reserve`/`reserve_usize` at all."
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!("did not reproduce - read_image() correctly rejected the request: {e}");
        }
    }
}
