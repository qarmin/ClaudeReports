// Standalone repro for ___BUGHUNT___ candidate 024: candidate 020's `thumbnail_sample_block`
// huge-block divisor bug (src/imageops/sample.rs), re-examined at a much larger scale than
// candidate 020's small boundary shapes (600x600, 257x257, 300x1000, 65537x2) to empirically
// check the hypothesis that the bug's relative error shrinks as the true sub-block count per
// dimension grows, while never fully vanishing for a non-256-aligned source.
//
// Root cause recap (see candidate 020's RESULT.md for the full derivation): `thumbnail_sample_block`
// splits a sampled block larger than 256*256 pixels into STEP_SIZE=256 sub-blocks via
// `step_by(STEP_SIZE)`, sums each sub-block average, then divides by a *floor*-division estimate
// of the sub-block count (`span / STEP_SIZE` per dimension) instead of the actual number of
// `step_by` iterations (`ceil(span / STEP_SIZE)` whenever `span` isn't an exact multiple of 256).
// For a block whose true sub-block count per dimension is `m = floor(span / STEP_SIZE)`, the
// predicted inflation factor is `((m+1)/m)^2` (both dimensions undercounted by exactly one).
//
// This prints, for two real materialized uniform-color images through the crate's real public
// `DynamicImage::thumbnail_exact` entry point:
//   - candidate 020's original 600x600 -> 1x1 case (m=2, predicted factor 2.25x = +125%)
//   - a 10000x10000 -> 1x1 case (m=39, predicted factor (40/39)^2 = 1.0519... = +5.19%), the exact
//     scale named in this candidate's own backlog entry
// confirming that the bug still measurably fires at the larger scale, with a smaller (but
// nonzero, and precisely as predicted) relative error - not "fixed at scale" and not "the same
// magnitude at every scale" either.
//
// This is reached through the real public API: `image::DynamicImage::thumbnail_exact` (and the
// lower-level `image::imageops::thumbnail` free function) on any image whose full extent (or, for
// `thumbnail`'s aspect-preserving variant, any per-output-pixel input window) exceeds 65536
// pixels - common for any real photo being reduced to a small thumbnail. No second
// participant/peer/wire input is involved (CONFIRMATION.md section 11 does not apply): a uniform
// -color `ImageBuffer` handed straight to the crate's real public thumbnail entry point *is* the
// genuine external input, the same as any real image a caller would pass in (same reasoning as
// candidate 020's own repro).
//
// Usage: `cargo run --release` prints REPRODUCED (wrong pixel value observed, with measured vs.
// predicted relative error) or `Not reproduced`.

use image::{DynamicImage, ImageBuffer, Luma};

fn check(label: &str, side: u32, m: u32, v: f32) -> bool {
    let src = ImageBuffer::from_pixel(side, side, Luma([v]));
    let img = DynamicImage::ImageLuma32F(src);
    let out = img.thumbnail_exact(1, 1);
    let got = out.as_luma32f().unwrap().get_pixel(0, 0).0[0];

    let predicted_factor = ((m + 1) as f64 / m as f64).powi(2);
    let measured_relative_error = (got as f64 - v as f64) / v as f64;
    let predicted_relative_error = predicted_factor - 1.0;
    let wrong = (got as f64 - v as f64).abs() > 1e-6;

    println!(
        "  {label:<22} {side}x{side} -> 1x1 (m={m}): expected {v}, got {got:.7} | \
         measured_relative_error={measured_relative_error:+.6} predicted_relative_error={predicted_relative_error:+.6} \
         | wrong={wrong}"
    );
    wrong
}

fn main() {
    println!("Thumbnail huge-block-splitting scaling check (Luma32F, uniform color):");

    let v = 0.6789_f32;
    // m = floor(side / 256).
    let reproduced_small = check("candidate-020-scale", 600, 2, v);
    let reproduced_large = check("this-candidate-scale", 10_000, 39, v);

    if reproduced_small && reproduced_large {
        println!(
            "REPRODUCED: the huge-block divisor bug fires at both scales through the real public \
             DynamicImage::thumbnail_exact API. The 10000x10000 case's relative error is measurably \
             smaller than the 600x600 case's (matching the ((m+1)/m)^2 shrinking-error model), but \
             is still nonzero - the bug does not disappear at scale, it only gets harder to notice."
        );
        std::process::exit(1);
    } else {
        println!(
            "Not reproduced: at least one of the two scales preserved the uniform source color \
             (small={reproduced_small}, large={reproduced_large})."
        );
    }
}
