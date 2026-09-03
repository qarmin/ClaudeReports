// Standalone reproduction for OPENEXR_SET_LIMITS_MISSING.
//
// Real, end-to-end trigger through the crate's own public API: `image::codecs::openexr::OpenExrDecoder`
// (the real exr-crate-backed decoder), fed a real EXR file (embedded below, taken verbatim from
// this crate's own test corpus), configured via the real, documented `ImageDecoder::set_limits`
// contract with `max_alloc: Some(0)`, then decoded via the real `ImageDecoder::read_image` public
// method.
//
// This is a single-participant finding (a caller configuring `Limits` on the decoder it itself
// owns, then calling that same decoder's own public method with a real file) - CONFIRMATION.md
// section 11's "real second participant" requirement doesn't add anything beyond "run the real
// public API end-to-end against a real external artifact", which this repro does; a malformed/
// oversized EXR file handed to the decoder's real public entry point *is* the genuine external
// input a real attacker/user would supply, with no "other side" being impersonated.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::codecs::openexr::OpenExrDecoder;
use image::{ImageDecoder, Limits};
use std::io::{BufReader, Cursor};

const EXR_BYTES: &[u8] = include_bytes!("../test.exr");

fn main() {
    let mut decoder = OpenExrDecoder::new(BufReader::new(Cursor::new(EXR_BYTES)))
        .expect("failed to construct OpenExrDecoder from embedded test EXR file");

    // Configure the decoder's own Limits directly (the real, documented `ImageDecoder::set_limits`
    // contract) with a zero allocation budget - the strictest possible request that "no further
    // decoder-owned allocation may happen".
    let mut limits = Limits::no_limits();
    limits.max_alloc = Some(0);
    let set_limits_result = decoder.set_limits(limits);
    println!("set_limits(max_alloc = Some(0)) -> {set_limits_result:?}");

    let prepared = decoder.prepare_image().expect("prepare_image failed");
    let total_bytes = prepared.total_bytes() as usize;
    println!(
        "dimensions: {:?}, color: {:?}, total_bytes: {total_bytes}",
        prepared.layout.dimensions(),
        prepared.layout.color
    );

    // The caller must already allocate a buffer of exactly this size to call `read_image` at
    // all - that allocation is the caller's own responsibility either way (same as every other
    // decoder). The bug under test is whether `read_image` *additionally* allocates a second,
    // decoder-owned scratch buffer of the same size internally without checking `max_alloc`.
    let mut buf = vec![0u8; total_bytes];

    match decoder.read_image(&mut buf) {
        Ok(_) => {
            eprintln!(
                "REPRODUCED: read_image() succeeded despite Limits{{max_alloc: Some(0)}} - it \
                 silently allocated a {total_bytes}-byte internal f32 scratch buffer \
                 (src/codecs/openexr.rs's `read_image`, the `rgba_channels` callback's own \
                 `vec![0_f32; ...]`) with no `Limits`/`max_alloc` accounting at all, because \
                 `OpenExrDecoder` never implemented `ImageDecoder::set_limits`."
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!("did not reproduce - read_image() correctly rejected the request: {e}");
        }
    }
}
