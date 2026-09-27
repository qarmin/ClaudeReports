//! Real end-to-end repro: encode a real AVIF with the crate's own real encoder (ravif), decode it
//! with the crate's own real dav1d-backed decoder (`AvifDecoder`), and observe that a caller-set
//! `Limits{max_image_width, max_image_height}` never rejects an oversized AVIF -- both because
//! `AvifDecoder::set_limits` never checks width/height at all (unlike PNG/JPEG/GIF/TIFF/HDR), and
//! because the full dav1d frame decode already ran unconditionally inside `AvifDecoder::new()`
//! before any limit could possibly apply.
use image::{ImageEncoder, ImageReaderOptions, Limits};
use std::io::Cursor;

fn make_avif(width: u32, height: u32) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height * 3) as usize];
    for (i, px) in pixels.chunks_exact_mut(3).enumerate() {
        px[0] = (i % 256) as u8;
        px[1] = ((i / 7) % 256) as u8;
        px[2] = ((i / 13) % 256) as u8;
    }
    let mut avif_bytes = Vec::new();
    let encoder = image::codecs::avif::AvifEncoder::new(&mut avif_bytes);
    encoder
        .write_image(&pixels, width, height, image::ExtendedColorType::Rgb8)
        .expect("encode failed");
    avif_bytes
}

fn main() {
    let width = 400u32;
    let height = 400u32;
    let avif_bytes = make_avif(width, height);
    println!("encoded avif: {} bytes, {}x{}", avif_bytes.len(), width, height);

    let tiny = make_avif(8, 8);

    // Scenario A: the documented raw-decoder API (`into_decoder()` + `ImageDecoder::set_limits`).
    use image::ImageDecoder;
    let t0 = std::time::Instant::now();
    let mut decoder = image::codecs::avif::AvifDecoder::new(Cursor::new(&avif_bytes))
        .expect("AvifDecoder::new failed");
    let new_elapsed = t0.elapsed();

    let t0_tiny = std::time::Instant::now();
    let _tiny_decoder = image::codecs::avif::AvifDecoder::new(Cursor::new(&tiny))
        .expect("AvifDecoder::new (tiny) failed");
    let new_elapsed_tiny = t0_tiny.elapsed();

    println!(
        "AvifDecoder::new() [full dav1d decode happens inside this call]: {}x{} took {:?}; 8x8 baseline took {:?}",
        width, height, new_elapsed, new_elapsed_tiny
    );

    let mut limits = Limits::default();
    limits.max_image_width = Some(10);
    limits.max_image_height = Some(10);
    let set_limits_result = decoder.set_limits(limits);
    println!(
        "decoder.set_limits({{max_image_width: 10, max_image_height: 10}}) on the already-decoded {}x{} AvifDecoder => {:?}",
        width, height, set_limits_result
    );
    let bug_a = set_limits_result.is_ok();
    match set_limits_result {
        Ok(()) => println!(
            "BUG A: AvifDecoder::set_limits() returned Ok() for a {}x{} image despite a 10x10 limit -- \
             unlike PNG/JPEG/GIF/TIFF/HDR decoders, AvifDecoder's set_limits() never checks width/height at all, \
             AND the full dav1d decode already ran unconditionally before this call.",
            width, height
        ),
        Err(e) => println!("(no bug A -- fixed) limits correctly rejected: {e}"),
    }

    // Scenario B: the documented *defensive* builder pattern -- set limits BEFORE calling
    // decode(), via ImageReaderOptions.
    let mut opts = ImageReaderOptions::new(Cursor::new(&avif_bytes));
    opts.set_format(image::ImageFormat::Avif);
    let mut limits_b = Limits::default();
    limits_b.max_image_width = Some(10);
    limits_b.max_image_height = Some(10);
    opts.limits(limits_b);

    let t1 = std::time::Instant::now();
    let decode_result = opts.decode();
    let decode_elapsed = t1.elapsed();
    println!(
        "ImageReaderOptions::new(..).limits(10x10).decode() on a real {}x{} AVIF => is_err={}, took {:?} (vs {:?} full-decode-alone baseline above)",
        width, height, decode_result.is_err(), decode_elapsed, new_elapsed
    );
    match &decode_result {
        Ok(img) => println!(
            "BUG B: decode() SUCCEEDED and returned a {}x{} image despite the 10x10 limit set via the documented pre-decode builder API.",
            img.width(), img.height()
        ),
        Err(e) => println!(
            "(no output-level leak) decode() ultimately rejected the image via a redundant reader-level check: {e} -- \
             but note this happened AFTER `into_reader()`'s call to `make_decoder()` already ran the full dav1d decode \
             (~{:?}, same cost as the unprotected baseline above) since AvifDecoder::new() itself is not gated by \
             any limit -- the CPU cost was already spent regardless of the eventual Err.",
            new_elapsed
        ),
    }

    println!();
    if bug_a {
        println!("RESULT: BUG REPRODUCED (AvifDecoder::set_limits ignores width/height limits)");
        std::process::exit(1);
    } else {
        println!("RESULT: set_limits correctly enforces width/height limits (fix present)");
    }
}
