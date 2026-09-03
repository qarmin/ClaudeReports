//! Standalone reproduction / differential-testing harness for ___BUGHUNT___ candidate 015.
//!
//! Follow-up to candidate 014, which confirmed that patching `image-webp` to a git rev containing
//! upstream PR image-rs/image-webp#171 ("fix: dispose transparent animated frames") fixes 014's own
//! repro but flips this repo's own `cargo test --test reference_images` fixture
//! `tests/images/webp/extended_images/anim.webp` frame 2 (0-indexed frame 1) from passing to
//! failing against the checked-in reference PNG. Two hypotheses were left open: (a) the checked-in
//! reference PNG is simply stale, produced under the pre-#171 buggy "canvas never cleared"
//! behavior, or (b) PR #171's fix has its own gap specific to lossy `VP8`+`ALPH` frames.
//!
//! This harness decodes `anim.webp`'s frame index 1 through `image`'s real public
//! `image::codecs::webp::WebPDecoder` entry point - once against whatever `image-webp` version this
//! crate is currently built against (pristine pinned `0.2.4`, or patched via `Cargo.patched.toml`,
//! see the candidate's `justfile`) - and diffs the result against two baked-in PNGs:
//!
//!   - `checked_in_reference_anim_02.png`: this repo's own checked-in reference (produced long ago,
//!     presumably against an old/buggy `image-webp`).
//!   - `trusted_reference_libwebp_frame_index1.png`: the same frame decoded once via Python
//!     Pillow's bundled `libwebp` (a real, independent, trusted third-party WebP decoder - not
//!     part of the `image`/`image-webp` codebase at all), captured once during this candidate's
//!     investigation and checked in here as a fixed comparison target.
//!
//! Exit code convention: this is a diagnostic tool, not a crash repro - it always exits 0 and
//! prints its findings to stdout for the reader (and `output.txt`) to interpret.

use image::{DynamicImage, ImageReader};
use std::fs;
use std::io::BufReader;

fn decode_frame_index1(webp_path: &str) -> DynamicImage {
    let file = fs::File::open(webp_path).expect("open anim.webp");
    let reader = ImageReader::new(BufReader::new(file)).expect("guess format");
    let frames = reader
        .into_frames()
        .collect_frames()
        .expect("decode all animation frames");
    assert_eq!(frames.len(), 6, "expected 6 ANMF frames in anim.webp");
    DynamicImage::from(frames[1].clone().into_buffer())
}

/// Returns (alpha_mismatches, rgb_mismatches_where_both_sides_visible, total_pixels,
/// max_rgb_abs_diff_where_both_sides_visible).
fn diff(a: &DynamicImage, b: &DynamicImage) -> (usize, usize, usize, i32) {
    let a = a.to_rgba8();
    let b = b.to_rgba8();
    assert_eq!(a.dimensions(), b.dimensions());
    let mut alpha_mismatches = 0usize;
    let mut rgb_mismatches_visible = 0usize;
    let mut max_rgb_abs_diff_visible = 0i32;
    let total = (a.width() * a.height()) as usize;
    for (pa, pb) in a.pixels().iter().zip(b.pixels().iter()) {
        if pa[3] != pb[3] {
            alpha_mismatches += 1;
        }
        if pa[3] > 0 && pb[3] > 0 {
            let d = (0..3)
                .map(|i| (pa[i] as i32 - pb[i] as i32).abs())
                .max()
                .unwrap();
            if d > 0 {
                rgb_mismatches_visible += 1;
            }
            max_rgb_abs_diff_visible = max_rgb_abs_diff_visible.max(d);
        }
    }
    (alpha_mismatches, rgb_mismatches_visible, total, max_rgb_abs_diff_visible)
}

fn main() {
    let decoded = decode_frame_index1("anim.webp");
    decoded
        .save("actual_output_frame_index1.png")
        .expect("save actual output");

    let checked_in_reference = image::open("checked_in_reference_anim_02.png")
        .expect("open checked_in_reference_anim_02.png");
    let trusted_reference = image::open("trusted_reference_libwebp_frame_index1.png")
        .expect("open trusted_reference_libwebp_frame_index1.png");

    println!("== This build's image-webp decode of anim.webp frame index 1 (0-indexed) ==");

    let (a_mm, rgb_mm, total, max_d) = diff(&decoded, &checked_in_reference);
    println!(
        "vs this repo's checked-in reference PNG (tests/reference/.../anim.webp.anim_02.png):"
    );
    println!(
        "  alpha mismatches: {a_mm}/{total} ({:.2}%)  |  rgb mismatches (both sides visible): {rgb_mm}  |  max rgb abs diff (both visible): {max_d}",
        100.0 * a_mm as f64 / total as f64
    );

    let (a_mm, rgb_mm, total, max_d) = diff(&decoded, &trusted_reference);
    println!(
        "vs trusted third-party reference (Pillow/libwebp decode of the same frame, baked in):"
    );
    println!(
        "  alpha mismatches: {a_mm}/{total} ({:.2}%)  |  rgb mismatches (both sides visible): {rgb_mm}  |  max rgb abs diff (both visible): {max_d}",
        100.0 * a_mm as f64 / total as f64
    );

    println!();
    println!(
        "Interpretation: if this build's decode matches the trusted third-party reference far \
         more closely than it matches this repo's own checked-in reference PNG (near-zero alpha \
         mismatches and rgb diffs bounded by ~1 for genuinely visible pixels vs the trusted \
         reference), that means the checked-in reference PNG is the one that's wrong/stale, not \
         this build's image-webp - i.e. hypothesis (a), not (b)."
    );
}
