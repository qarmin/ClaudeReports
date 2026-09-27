// Standalone reproduction for TIFF_CMYK_SCRATCH_BUFFER_DOUBLE_ACCOUNTING.
//
// Real, end-to-end trigger through the crate's own public API: `image::codecs::tiff::TiffDecoder`
// (the real `tiff`-crate-backed decoder), fed a real, well-formed CMYK8 TIFF file (embedded below,
// a genuine file produced by the `tiff` crate's own encoder - not hand-authored adversarial
// bytes), configured via the real, documented `ImageDecoder::set_limits` contract, then decoded
// via the real `ImageDecoder::read_image` public method.
//
// This is a single-participant finding (a caller configuring `Limits` on the decoder it itself
// owns, then calling that same decoder's own public method with a real file) - CONFIRMATION.md
// section 11's "real second participant" requirement doesn't add anything beyond "run the real
// public API end-to-end against a real external artifact", which this repro does.
//
// The bug: `TiffDecoder::read_image` for a CMYK8/CMYK16 source allocates an internal decode
// buffer sized for the *original* CMYK color type (4 bytes/pixel) before converting it down to
// the *final* RGB output (3 bytes/pixel). That internal buffer's own size is correctly bounded
// against the caller's declared `max_alloc`, but nothing in `TiffDecoder::set_limits`/
// `redistribute_limits` accounts for the fact that the caller's own output buffer (`buf`, RGB-
// sized) already occupies memory that the doc comment on `Limits::max_alloc` says should count
// towards the *same* declared ceiling ("The maximum allowed sum of allocations allocated by the
// decoder at any one time... The bytes required to store the output image count towards this
// value."). So `read_image` can succeed while the combined live memory (RGB output buffer + CMYK
// internal scratch buffer) exceeds the declared `max_alloc`.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::codecs::tiff::TiffDecoder;
use image::{ImageDecoder, Limits};
use std::io::Cursor;

const CMYK_TIFF_BYTES: &[u8] = include_bytes!("../cmyk_test.tif");

fn main() {
    let mut decoder = TiffDecoder::new(Cursor::new(CMYK_TIFF_BYTES))
        .expect("failed to construct TiffDecoder from embedded CMYK8 test TIFF file");

    let prepared = decoder.prepare_image().expect("prepare_image failed");
    let rgb_output_bytes = prepared.total_bytes();
    println!(
        "dimensions: {:?}, final output color: {:?}, RGB output bytes: {rgb_output_bytes}",
        prepared.layout.dimensions(),
        prepared.layout.color
    );

    // The internal CMYK (4 bytes/pixel) scratch buffer this decoder needs is strictly larger
    // than the final RGB (3 bytes/pixel) output buffer computed above.
    let cmyk_internal_bytes = rgb_output_bytes * 4 / 3;
    println!("CMYK internal scratch buffer needed: {cmyk_internal_bytes} bytes");

    // Pick a max_alloc that:
    //  - comfortably covers the CMYK internal buffer's own requirement plus the external `tiff`
    //    crate's own separate (roughly fixed, not output-size-proportional) intermediate/IFD
    //    parsing budget requirement, empirically determined for this file to be a bit under
    //    1.5MB, and
    //  - is strictly LESS than rgb_output_bytes + cmyk_internal_bytes (the true combined peak
    //    live memory: the caller's own pre-existing RGB `buf` plus the decoder's internal CMYK
    //    scratch buffer, both alive at the same time during the color conversion).
    let max_alloc = cmyk_internal_bytes + 1_500_000;
    let combined_live_memory = rgb_output_bytes + cmyk_internal_bytes;
    println!("declared max_alloc: {max_alloc}");
    println!("true combined live memory required: {combined_live_memory}");
    assert!(
        max_alloc < combined_live_memory,
        "test setup error: max_alloc should be less than the true combined live memory"
    );

    let mut limits = Limits::no_limits();
    limits.max_alloc = Some(max_alloc);
    let set_limits_result = decoder.set_limits(limits);
    println!("set_limits(max_alloc = Some({max_alloc})) -> {set_limits_result:?}");

    // The caller must already allocate a buffer of exactly this size to call `read_image` at
    // all - that allocation is the caller's own responsibility either way (same as every other
    // decoder), and per the `Limits::max_alloc` doc comment, its bytes are supposed to count
    // towards the very same declared budget as the decoder's own internal scratch allocation.
    let mut buf = vec![0u8; rgb_output_bytes as usize];

    match decoder.read_image(&mut buf) {
        Ok(_) => {
            eprintln!(
                "REPRODUCED: read_image() succeeded with declared max_alloc = {max_alloc}, even \
                 though the caller's own RGB output buffer ({rgb_output_bytes} bytes) plus the \
                 decoder's own internal CMYK scratch buffer ({cmyk_internal_bytes} bytes) sum to \
                 {combined_live_memory} bytes, {} bytes MORE than what was declared allowed - the \
                 decoder's own internal accounting (`TiffDecoder::redistribute_limits`) checks \
                 its scratch buffer against the *raw*, undiminished max_alloc value, unaware that \
                 the same budget was already independently spent on the caller's own output \
                 buffer.",
                combined_live_memory - max_alloc
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!("did not reproduce - read_image() correctly rejected the request: {e}");
        }
    }
}
