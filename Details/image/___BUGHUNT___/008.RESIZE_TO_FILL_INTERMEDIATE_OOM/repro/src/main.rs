// Standalone reproduction: `DynamicImage::resize_to_fill` (the exact public entry point named in
// the candidate) attempts to allocate an intermediate resize buffer of ~40,000,000,000 bytes for
// a completely ordinary, small, valid 1x1_000_000 RGBA8 source image (4,000,000 bytes) resized to
// a modest 100x100 target - because `resize_dimensions`'s `fill` ratio is driven by whichever
// axis needs the *larger* scale factor, and that intermediate buffer is thrown away almost
// entirely by the crop that follows it.
//
// Driven entirely through the crate's public API (`RgbaImage::new` + `DynamicImage::resize_to_fill`)
// - a caller resizing a thin/tall (or short/wide) but otherwise completely valid decoded image to
// a "fill" thumbnail size is a completely ordinary, non-malicious use of this API, no crafted
// file/codec parsing involved.
//
// The actual failure mode is a huge allocation request handled by Rust's global allocator, which
// calls `handle_alloc_error` and aborts the process (SIGABRT) - this is not a catchable panic, so
// this binary is meant to be run under a virtual-memory cap (see this candidate's `justfile`,
// which runs it under `ulimit -v`) to make the "would try to allocate ~40GB for a 4MB input"
// behavior deterministic and safe to demonstrate without actually needing dozens of GB of real
// RAM on the machine running the repro.
//
// Exit code convention: process exits/aborts abnormally (non-zero, e.g. 134 from SIGABRT) =
// reproduced (bug present, huge allocation attempted). Exit 0 with a printed (100, 100) result =
// did not reproduce (bug fixed, no oversized intermediate buffer needed).

use image::{DynamicImage, GenericImageView, RgbaImage};

fn main() {
    let (w, h): (u32, u32) = (1, 1_000_000);
    let (nwidth, nheight): (u32, u32) = (100, 100);

    println!(
        "building source image {w}x{h} ({} pixels, {} bytes as RGBA8)",
        u64::from(w) * u64::from(h),
        u64::from(w) * u64::from(h) * 4
    );
    let buf = RgbaImage::new(w, h);
    let mut img = DynamicImage::ImageRgba8(buf);

    println!("calling resize_to_fill({nwidth}, {nheight}, Triangle)");
    img.resize_to_fill(nwidth, nheight, image::imageops::FilterType::Triangle);

    println!("did not reproduce - result dims: {:?}", img.dimensions());
}
