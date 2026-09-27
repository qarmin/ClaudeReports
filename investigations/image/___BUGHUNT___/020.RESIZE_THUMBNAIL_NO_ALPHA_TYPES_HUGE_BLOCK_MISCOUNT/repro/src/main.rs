// Standalone repro for: `imageops::thumbnail`/`DynamicImage::thumbnail_exact` produce a wrong
// (inflated, often saturated) pixel value - not just a rounding-off-by-one, an actual multiplicative
// distortion - when thumbnailing a source image whose sampled block exceeds 256*256 pixels in a
// dimension that is not an exact multiple of 256.
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

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};

fn main() {
    println!("Thumbnail huge-block-splitting uniform-color check (no-alpha types):");

    let mut any_wrong = false;

    // Luma8: 600x600 (not a multiple of 256, > 65536 pixels) -> 1x1.
    {
        let v = 137_u8;
        let src = ImageBuffer::from_pixel(600, 600, Luma([v]));
        let img = DynamicImage::ImageLuma8(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_luma8().unwrap().get_pixel(0, 0).0[0];
        println!("  Luma8   600x600 -> 1x1: expected {v}, got {got}");
        any_wrong |= got != v;
    }

    // Luma16.
    {
        let v = 54321_u16;
        let src = ImageBuffer::from_pixel(600, 600, Luma([v]));
        let img = DynamicImage::ImageLuma16(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_luma16().unwrap().get_pixel(0, 0).0[0];
        println!("  Luma16  600x600 -> 1x1: expected {v}, got {got}");
        any_wrong |= got != v;
    }

    // Rgb16.
    {
        let v = [1000_u16, 2000_u16, 60000_u16];
        let src = ImageBuffer::from_pixel(600, 600, Rgb(v));
        let img = DynamicImage::ImageRgb16(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_rgb16().unwrap().get_pixel(0, 0).0;
        println!("  Rgb16   600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= got != v;
    }

    // Luma32F.
    {
        let v = 0.6789_f32;
        let src = ImageBuffer::from_pixel(600, 600, Luma([v]));
        let img = DynamicImage::ImageLuma32F(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_luma32f().unwrap().get_pixel(0, 0).0[0];
        println!("  Luma32F 600x600 -> 1x1: expected {v}, got {got}");
        any_wrong |= (got - v).abs() >= 1e-4;
    }

    // Rgb32F.
    {
        let v = [0.1_f32, 0.5_f32, 0.9_f32];
        let src = ImageBuffer::from_pixel(600, 600, Rgb(v));
        let img = DynamicImage::ImageRgb32F(src);
        let out = img.thumbnail_exact(1, 1);
        let got = out.as_rgb32f().unwrap().get_pixel(0, 0).0;
        println!("  Rgb32F  600x600 -> 1x1: expected {v:?}, got {got:?}");
        any_wrong |= (got[0] - v[0]).abs() >= 1e-4 || (got[1] - v[1]).abs() >= 1e-4 || (got[2] - v[2]).abs() >= 1e-4;
    }

    // Also exercise the plain `thumbnail` (aspect-preserving) entry point, not just
    // `thumbnail_exact`, for one type, plus a non-square shape (300x1000).
    {
        let v = 91_u8;
        let src = ImageBuffer::from_pixel(300, 1000, Luma([v]));
        let img = DynamicImage::ImageLuma8(src);
        let out = img.thumbnail(1, 3);
        let dims = out.dimensions();
        let mut ok = true;
        for p in out.as_luma8().unwrap().pixels() {
            if p.0[0] != v {
                ok = false;
            }
        }
        println!("  Luma8   300x1000 -> thumbnail(1,3) = {dims:?}: all pixels == {v}? {ok}");
        any_wrong |= !ok;
    }

    if any_wrong {
        println!(
            "REPRODUCED: thumbnailing a uniform-color source through the real public \
             DynamicImage::thumbnail/thumbnail_exact API did not preserve that color - the \
             huge-block STEP_SIZE-splitting divisor in thumbnail_sample_block undercounts \
             sub-blocks whenever a dimension isn't an exact multiple of 256."
        );
        std::process::exit(1);
    } else {
        println!("Not reproduced: every thumbnail preserved the uniform source color.");
    }
}
