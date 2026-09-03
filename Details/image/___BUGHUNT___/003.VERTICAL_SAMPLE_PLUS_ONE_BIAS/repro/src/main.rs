// Standalone repro for: `image::imageops::resize` adds a flat +1 (per channel, for u8 images) to
// every pixel of a vertically-resized image, because `vertical_sample` in
// `src/imageops/sample.rs` initializes its per-pixel accumulator to `Rgba([1.0; 4])` instead of
// `Rgba([0.0; 4])` before summing the already-normalized (sum-to-1) weighted filter taps.
//
// Calls only the crate's real public API (`image::imageops::resize`) - no hand-constructed
// internal state, no second participant involved (this is a pure single-process logic bug in
// how a caller-supplied image is resampled, not a reaction to another participant's input, so
// CONFIRMATION.md section 11's "real second participant" requirement does not apply).
//
// Expected (if the accumulator started at 0.0, as the weights already sum to 1): every uniform
// source resizes to the same uniform value. Actual (pristine tree): output is biased by +1.

use image::{imageops, imageops::FilterType, ImageBuffer, Rgb};

fn check(w: u32, h: u32, nw: u32, nh: u32, value: u8, filter: FilterType) -> bool {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {
            img.put_pixel(x, y, Rgb([value, value, value]));
        }
    }

    let out = imageops::resize(&img, nw, nh, filter);

    let mut all_ok = true;
    for (x, y, p) in out.enumerate_pixels() {
        if p.0 != [value, value, value] {
            all_ok = false;
        }
        if x == 0 && y == 0 {
            println!(
                "  {w}x{h} -> {nw}x{nh} ({filter:?}): pixel(0,0) = {:?} (expected [{value},{value},{value}])",
                p
            );
        }
    }
    all_ok
}

fn main() {
    println!("Uniform-color resize through image::imageops::resize (public API):");
    let mut any_wrong = false;

    // The exact case from the original report.
    any_wrong |= !check(4, 4, 2, 2, 100, FilterType::Triangle);
    // A handful of boundary variants: non-square, width-unchanged, width == 1, a different filter.
    any_wrong |= !check(10, 10, 3, 3, 50, FilterType::Lanczos3);
    any_wrong |= !check(8, 6, 8, 3, 50, FilterType::Lanczos3);
    any_wrong |= !check(1, 100, 1, 10, 50, FilterType::Lanczos3);

    if any_wrong {
        println!(
            "BUG REPRODUCED: expected every output pixel to equal the uniform input value \
             (normalized filter weights sum to 1), but at least one deviated - see \
             src/imageops/sample.rs's vertical_sample accumulator init."
        );
        std::process::exit(1);
    } else {
        println!("Not reproduced on this build (see RESULT.md for the affected version).");
    }
}
