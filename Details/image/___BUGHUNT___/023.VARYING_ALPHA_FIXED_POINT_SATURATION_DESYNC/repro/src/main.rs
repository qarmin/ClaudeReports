// Standalone reproduction for candidate 023: spatially-varying-alpha resize through
// `DynamicImage::resize_exact`'s pic-scale-safe-backed fixed-point (u8/u16) path can produce
// pixels whose color is far outside the true source color range, even at fully-opaque output
// alpha, when using a filter with negative lobes (CatmullRom/Gaussian/Lanczos3).
//
// This uses only the crate's real public API (`DynamicImage::resize_exact`), exactly the way any
// caller would use it - no internal/test-only scaffolding.
//
// Invariant under test: a source image with a spatially *constant* RGB color `C` but *varying*
// alpha, resized with any linear filter through a premultiply-alpha-aware resize, should recover
// exactly `C` at every output pixel whose alpha did not collapse to (near) zero - this is
// algebraic, not "expected legitimate blending drift": color/alpha = (C*alpha)/alpha = C exactly
// in real-number arithmetic, regardless of how the resampling kernel's weighted sum overshoots,
// as long as the same weights are applied consistently to the premultiplied-color and alpha
// channels and nothing along the way clips them independently.
//
// This repro cross-checks the u16 (fixed-point) result against the crate's own f32 path on the
// exact same geometry/filter as an independent, unclamped-arithmetic oracle - the f32 path
// recovers `C` to within ~1e-6, proving the invariant *should* hold and that the u16 path's large
// deviation is a genuine implementation defect, not inherent filter ringing.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (bug present).

use image::{imageops::FilterType, DynamicImage};

const SRC_W: u32 = 2;
const SRC_H: u32 = 2;
const DST_W: u32 = 50;
const DST_H: u32 = 50;
const FILTER: FilterType = FilterType::Lanczos3;

// True constant source color, u16 domain.
const TRUE_R: u16 = 31500;
const TRUE_G: u16 = 11500;
const TRUE_B: u16 = 51000;

fn build_u16_source() -> image::ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    image::ImageBuffer::from_fn(SRC_W, SRC_H, |x, _y| {
        let alpha = ((x as u64 * 65535) / (SRC_W as u64 - 1).max(1)) as u16;
        image::Rgba([TRUE_R, TRUE_G, TRUE_B, alpha])
    })
}

fn build_f32_source() -> image::Rgba32FImage {
    let (r, g, b) = (
        TRUE_R as f32 / 65535.0,
        TRUE_G as f32 / 65535.0,
        TRUE_B as f32 / 65535.0,
    );
    image::Rgba32FImage::from_fn(SRC_W, SRC_H, |x, _y| {
        let alpha = (x as f32) / ((SRC_W as f32) - 1.0).max(1.0);
        image::Rgba([r, g, b, alpha])
    })
}

fn main() {
    // --- u16 fixed-point path (the suspect) ---
    let mut u16_img = DynamicImage::ImageRgba16(build_u16_source());
    u16_img.resize_exact(DST_W, DST_H, FILTER);
    let u16_out = u16_img.as_rgba16().unwrap();

    // --- f32 path (independent oracle, same geometry/filter, same public API) ---
    let mut f32_img = DynamicImage::ImageRgba32F(build_f32_source());
    f32_img.resize_exact(DST_W, DST_H, FILTER);
    let f32_out = f32_img.as_rgba32f().unwrap();

    let mut max_dev_u16 = 0i64;
    let mut max_dev_pixel = None;
    for (x, y, p) in u16_out.enumerate_pixels() {
        let [pr, pg, pb, pa] = p.0;
        // Only look at pixels the fixed-point path itself reports as fully opaque (alpha at the
        // representable ceiling) - there is no ambiguity/legitimate-blending excuse left for
        // those: a fully-opaque output pixel of a spatially-constant-color source must show that
        // exact source color.
        if pa != u16::MAX {
            continue;
        }
        let dev = (pr as i64 - TRUE_R as i64)
            .abs()
            .max((pg as i64 - TRUE_G as i64).abs())
            .max((pb as i64 - TRUE_B as i64).abs());
        if dev > max_dev_u16 {
            max_dev_u16 = dev;
            max_dev_pixel = Some((x, y, pr, pg, pb, pa));
        }
    }

    let mut max_dev_f32 = 0f32;
    let (r, g, b) = (
        TRUE_R as f32 / 65535.0,
        TRUE_G as f32 / 65535.0,
        TRUE_B as f32 / 65535.0,
    );
    for (_x, _y, p) in f32_out.enumerate_pixels() {
        let [pr, pg, pb, pa] = p.0;
        if pa < 0.999 {
            continue;
        }
        let dev = (pr - r).abs().max((pg - g).abs()).max((pb - b).abs());
        if dev > max_dev_f32 {
            max_dev_f32 = dev;
        }
    }

    println!("geometry: {SRC_W}x{SRC_H} -> {DST_W}x{DST_H}, filter={FILTER:?}");
    println!("true source color (u16): [{TRUE_R}, {TRUE_G}, {TRUE_B}]");
    println!(
        "u16 fixed-point path: max color deviation among fully-opaque output pixels = {max_dev_u16} / 65535 ({:.2}%), at {max_dev_pixel:?}",
        max_dev_u16 as f64 / 65535.0 * 100.0
    );
    println!(
        "f32 path (oracle, same geometry/filter): max color deviation among fully-opaque output pixels = {max_dev_f32} ({:.6}% of full range)",
        max_dev_f32 as f64 * 100.0
    );

    // A generous threshold - ordinary 16-bit rounding noise from two premultiply/unpremultiply
    // round trips is at most a handful of ULPs, nowhere near this. The f32 oracle staying near
    // machine epsilon on the exact same case proves the invariant is expected to hold.
    const THRESHOLD: i64 = 100;
    if max_dev_u16 > THRESHOLD {
        eprintln!(
            "REPRODUCED: u16 path deviates by {max_dev_u16}/65535 at a fully-opaque pixel (threshold {THRESHOLD}), while the f32 oracle on the identical case deviates by only {max_dev_f32}"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - u16 path stayed within expected rounding noise");
    }
}
