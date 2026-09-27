// Standalone repro for: `imageops::thumbnail`/`DynamicImage::thumbnail_exact` produce a wrong
// (inflated, often saturated) pixel value - not just a rounding-off-by-one, an actual multiplicative
// distortion - when thumbnailing a source image whose sampled block exceeds 256*256 pixels in a
// dimension that is not an exact multiple of 256.
//
// This is the same root cause as candidate 020 (`thumbnail_sample_block` in
// `src/imageops/sample.rs`, huge-block STEP_SIZE-splitting divisor), re-checked here specifically
// for the alpha-bearing pixel types (`LumaA8`, `Rgba8`, `Rgba16`, `Rgba32F`) that candidate 020
// itself did not empirically exercise. The buggy divisor computation has no type-specific
// branching (it operates purely on `u64` pixel-count/coordinate arithmetic before any per-channel
// summation), so it was expected to reproduce identically for these types - this repro confirms
// that expectation empirically rather than leaving it as an inference.
//
// Root cause (src/imageops/sample.rs, `thumbnail_sample_block`): when a sampled block has more
// than 256*256 pixels, the function splits it into STEP_SIZE=256 sub-blocks, sums each sub-block's
// own average via `sum.add_pixel(k)`, then divides that sum by
// `(right - left) / STEP_SIZE * (top - bottom) / STEP_SIZE` (integer/floor division) to get the
// final average. `step_by(STEP_SIZE)` iterates one extra, smaller trailing sub-block per dimension
// whenever the span isn't an exact multiple of 256, so the number of `sum.add_pixel` calls actually
// made is `ceil(span/STEP_SIZE)` per dimension, not `floor(span/STEP_SIZE)`. The floor-based
// divisor is then too small, inflating the result.
//
// This is reached through the real public API: `image::DynamicImage::thumbnail`/`thumbnail_exact`
// (and the lower-level `image::imageops::thumbnail` free function) on any image whose full extent
// (or, for `thumbnail`'s non-exact aspect-preserving variant, any *per-output-pixel input window*)
// exceeds 65536 pixels - common for any real photo being reduced to a small thumbnail. No second
// participant/peer/wire input is involved (CONFIRMATION.md section 11 does not apply): a uniform
// -color `ImageBuffer` handed straight to the crate's real public thumbnail entry point *is* the
// genuine external input, the same as any real image a caller would pass in.
//
// Usage: `cargo run --release` prints REPRODUCED (wrong pixel value observed) or `Not reproduced`.

use image::{DynamicImage, GenericImageView, ImageBuffer, LumaA, Rgba};

fn main() {
    println!("Thumbnail huge-block-splitting uniform-color check (alpha-bearing types):");

    let mut any_wrong = false;

    // LumaA8: 600x600 (not a multiple of 256, > 65536 pixels) -> 1x1.
    {
        let v = [137_u8, 200_u8];
        let src = ImageBuffer::from_pixel(600, 600, LumaA(v));
        let img = DynamicImage::ImageLumaA8(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_luma_alpha8().unwrap().get_pixel(0, 0).0;
        println!("  LumaA8  600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= got != v;
    }

    // Rgba8.
    {
        let v = [10_u8, 20_u8, 250_u8, 255_u8];
        let src = ImageBuffer::from_pixel(600, 600, Rgba(v));
        let img = DynamicImage::ImageRgba8(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_rgba8().unwrap().get_pixel(0, 0).0;
        println!("  Rgba8   600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= got != v;
    }

    // Rgba16.
    {
        let v = [1000_u16, 2000_u16, 60000_u16, 65535_u16];
        let src = ImageBuffer::from_pixel(600, 600, Rgba(v));
        let img = DynamicImage::ImageRgba16(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_rgba16().unwrap().get_pixel(0, 0).0;
        println!("  Rgba16  600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= got != v;
    }

    // Rgba32F.
    {
        let v = [0.1_f32, 0.5_f32, 0.9_f32, 0.3_f32];
        let src = ImageBuffer::from_pixel(600, 600, Rgba(v));
        let img = DynamicImage::ImageRgba32F(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_rgba32f().unwrap().get_pixel(0, 0).0;
        println!("  Rgba32F 600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= (got[0] - v[0]).abs() >= 1e-4
            || (got[1] - v[1]).abs() >= 1e-4
            || (got[2] - v[2]).abs() >= 1e-4
            || (got[3] - v[3]).abs() >= 1e-4;
    }

    // Also exercise the plain `thumbnail` (aspect-preserving) entry point, not just
    // `thumbnail_exact`, for one type, plus a non-square shape (300x1000).
    {
        let v = [91_u8, 5_u8, 6_u8, 255_u8];
        let src = ImageBuffer::from_pixel(300, 1000, Rgba(v));
        let img = DynamicImage::ImageRgba8(src);
        let out = img.thumbnail(1, 3);
        let dims = out.dimensions();
        let mut ok = true;
        for p in out.as_rgba8().unwrap().pixels() {
            if p.0 != v {
                ok = false;
            }
        }
        println!("  Rgba8   300x1000 -> thumbnail(1,3) = {dims:?}: all pixels == {v:?}? {ok}");
        any_wrong |= !ok;
    }

    if any_wrong {
        println!(
            "REPRODUCED: thumbnailing a uniform-color source through the real public \
             DynamicImage::thumbnail/thumbnail_exact API did not preserve that color for one or \
             more alpha-bearing pixel types - the huge-block STEP_SIZE-splitting divisor in \
             thumbnail_sample_block undercounts sub-blocks whenever a dimension isn't an exact \
             multiple of 256."
        );
        std::process::exit(1);
    } else {
        println!("Not reproduced: every thumbnail preserved the uniform source color.");
    }
}
