// Differential check: does any accumulator in `src/imageops/sample.rs` (outside the
// already-confirmed `vertical_sample` bug at line 547, candidate 003) share the same
// "wrong seed value" mistake? Every one of these operations is documented to preserve a
// uniform-color image (a weighted average / box average / convolution normalized to sum
// to 1.0 of the same value everywhere must equal that value), so any accumulator seeded
// wrong shows up immediately as a uniform-input regression, exactly like candidate 003's
// vertical_sample did.
//
// `resize` (all FilterType variants) is EXPECTED to fail here - it goes through
// vertical_sample, which candidate 003 already confirmed and reported separately. This
// repro's purpose is everything else in the file: thumbnail, blur (Gaussian), filter3x3.

use image::imageops::{blur, filter3x3, resize, thumbnail, FilterType};
use image::{ImageBuffer, Rgb};

fn uniform(w: u32, h: u32, v: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_pixel(w, h, Rgb([v, v, v]))
}

fn count_wrong(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, expected: u8) -> (usize, usize) {
    let total = (img.width() * img.height()) as usize;
    let wrong = img.pixels().iter().filter(|p| p.0 != [expected, expected, expected]).count();
    (wrong, total)
}

fn main() {
    let src = uniform(20, 16, 100);
    let mut any_new_bug = false;

    let (wrong, total) = count_wrong(&thumbnail(&src, 5, 4), 100);
    println!("thumbnail: {wrong}/{total} pixels wrong (expected [100;3])");
    any_new_bug |= wrong != 0;

    let (wrong, total) = count_wrong(&blur(&src, 2.0), 100);
    println!("blur (gaussian): {wrong}/{total} pixels wrong (expected [100;3])");
    any_new_bug |= wrong != 0;

    let (wrong, total) = count_wrong(&filter3x3(&src, &[1.0 / 9.0; 9]), 100);
    println!("filter3x3: {wrong}/{total} pixels wrong (expected [100;3])");
    any_new_bug |= wrong != 0;

    // Reference-only: resize is known-buggy via vertical_sample (candidate 003), not this
    // candidate's target. Printed for context, not counted towards `any_new_bug`.
    for filt in [FilterType::Nearest, FilterType::Triangle, FilterType::Lanczos3] {
        let (wrong, total) = count_wrong(&resize(&src, 7, 6, filt), 100);
        println!("resize {filt:?} (down, KNOWN candidate-003 bug, not counted here): {wrong}/{total} pixels wrong");
    }

    if any_new_bug {
        println!("BUG REPRODUCED: an accumulator outside vertical_sample is seeded wrong");
        std::process::exit(1);
    } else {
        println!("Not reproduced: no additional accumulator-seed bug found outside the already-confirmed vertical_sample (candidate 003)");
    }
}
