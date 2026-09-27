// Standalone repro for: the crate's own `cargo test --lib` suite has no test that asserts
// resizing a uniform-color *f32* image (`Rgb32FImage`) preserves that color - a sibling gap to
// candidate 009's `Rgb<u8>` test. Candidate 003's confirmed `vertical_sample` +1.0
// accumulator-seed bias applies before any clamp step, so for `f32` output (which is normally in
// `[0.0, 1.0]`) the bug pushes every resized pixel above 1.0, where `S::clamp_nearest_from`
// (`src/primitive_sealed.rs`) then saturates it to exactly `1.0` (pure white) - a much larger,
// more visible corruption than the +1/255 the `u8` case showed, yet still uncaught by the
// existing 331/332-test suite because no f32-specific uniform-color test exists either.
//
// This binary performs, through the crate's real public API (`image::imageops::resize` on a
// `Rgb32FImage`), the exact same check as the regression test added in `patch.diff`
// (`test_resize_uniform_color_f32_preserved` in `src/imageops/sample.rs`). It is the same
// single-process logic check as the added unit test - no second participant/peer/wire input is
// involved (CONFIRMATION.md section 11 does not apply).
//
// Usage: `cargo run --release` prints GAP DEMONSTRATED (bug present, no test would have caught
// it) or `Gap closed` (assertion holds).

use image::{imageops, imageops::FilterType, ImageBuffer, Pixel, Rgb, Rgb32FImage};

fn check(w: u32, h: u32, nw: u32, nh: u32, value: f32, filter: FilterType) -> bool {
    let src: Rgb32FImage = ImageBuffer::from_pixel(w, h, Rgb([value, value, value]));
    let out = imageops::resize(&src, nw, nh, filter);

    let mut all_ok = true;
    for (x, y, p) in out.enumerate_pixels() {
        for &c in p.channels() {
            if (c - value).abs() >= 1e-5 {
                all_ok = false;
            }
        }
        if x == 0 && y == 0 {
            println!(
                "  {w}x{h} -> {nw}x{nh} ({filter:?}): pixel(0,0) = {:?} (expected [{value}, {value}, {value}])",
                p.0
            );
        }
    }
    all_ok
}

fn main() {
    println!("Uniform-color f32 resize preservation check (mirrors the added regression test):");
    let filters = [
        FilterType::Nearest,
        FilterType::Triangle,
        FilterType::CatmullRom,
        FilterType::Gaussian,
        FilterType::Lanczos3,
    ];

    let mut any_wrong = false;
    for filter in filters {
        any_wrong |= !check(6, 6, 3, 3, 0.4, filter);
    }

    if any_wrong {
        println!(
            "GAP DEMONSTRATED: resizing a uniform-color f32 image did not preserve that color \
             (saturated to 1.0 instead) - this is exactly the bug class the crate's own test \
             suite does not catch for the f32 pixel type."
        );
        std::process::exit(1);
    } else {
        println!("Gap closed: every resize preserved the uniform source color.");
    }
}
