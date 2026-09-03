// Standalone repro for: the crate's own `cargo test --lib` suite has no test that asserts
// resizing a uniform-color image preserves that color, so a wrong constant folded into every
// resized pixel (candidate 003's confirmed `vertical_sample` +1.0 accumulator-seed bias) passes
// the full 331-test suite unnoticed.
//
// This binary performs, through the crate's real public API (`image::imageops::resize`), the
// exact same check as the regression test added in `patch.diff`
// (`test_resize_uniform_color_preserved` in `src/imageops/sample.rs`). It is the same single
// -process logic check as the added unit test - no second participant/peer/wire input is
// involved (CONFIRMATION.md section 11 does not apply), so running it here as an external public
// -API caller is equivalent to running the in-tree test itself.
//
// Usage: `cargo run --release` prints GAP DEMONSTRATED (bug present, no test would have caught
// it) or `Gap closed` (assertion holds - either the accumulator is fixed, or this binary is being
// run against a tree where the bug never existed).

use image::{imageops, imageops::FilterType, ImageBuffer, Rgb};

fn check(w: u32, h: u32, nw: u32, nh: u32, value: u8, filter: FilterType) -> bool {
    let src: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(w, h, Rgb([value, value, value]));
    let out = imageops::resize(&src, nw, nh, filter);

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
    println!("Uniform-color resize preservation check (mirrors the added regression test):");
    let filters = [
        FilterType::Nearest,
        FilterType::Triangle,
        FilterType::CatmullRom,
        FilterType::Gaussian,
        FilterType::Lanczos3,
    ];

    let mut any_wrong = false;
    for filter in filters {
        any_wrong |= !check(6, 6, 3, 3, 100, filter);
    }

    if any_wrong {
        println!(
            "GAP DEMONSTRATED: resizing a uniform-color image did not preserve that color - this \
             is exactly the bug class the crate's own test suite (331 tests, pristine tree) does \
             not catch."
        );
        std::process::exit(1);
    } else {
        println!("Gap closed: every resize preserved the uniform source color.");
    }
}
