//! Standalone repro for ___BUGHUNT___ candidate 018.
//!
//! Extends candidate 017's confirmed pic-scale-safe `compute_weights.rs::generate_weights`
//! window-misalignment bug (originally shown only for the horizontal pass on `Luma8`) to:
//!   - the VERTICAL resampling pass (a width==1 source column instead of a height==1 source row)
//!   - other pixel types sharing the same fixed-point `generate_weights` call: `Rgb8`, `Rgba8`,
//!     `Luma16`
//!
//! Real end-to-end trigger path, same as candidate 017: `image::DynamicImage::resize_exact` (the
//! crate's actual public resize API, pic-scale-safe-backed) compared against
//! `image::imageops::resize` (the crate's own separate, independent pure-Rust legacy resizer),
//! both driven purely through `image`'s public API on a real `ImageBuffer`. No hand-constructed
//! internal state.
//!
//! Note on `Luma32F` (the floating-point `generate_weights` path): NOT included here. Probing it
//! during triage showed the oracle (`imageops::resize`) itself disagrees with itself by up to a
//! full 1.0 (0..1 scale) on some large-upscale cases, which turned out to be caused by a *known,
//! already-documented, separate* bug in the oracle's own `vertical_sample` accumulator (see
//! `___BUGHUNT___/003.VERTICAL_SAMPLE_PLUS_ONE_BIAS/RESULT.md` - a `+1.0`-per-channel seed bug
//! still present, unfixed, in this repo's working tree). That bias is invisible in u8/u16 output
//! (rounds away as +-1) but dominates in `f32` output (0..1 scale), so it would confound rather
//! than confirm this candidate's finding. Excluded rather than reported as a false positive.

use image::{imageops, imageops::FilterType, DynamicImage, ImageBuffer, Luma, Rgb, Rgba};

fn ramp_u8(i: u32, n: u32) -> u8 {
    (i as f64 * 255.0 / (n - 1) as f64).round() as u8
}

struct Case {
    src: u32,
    dst: u32,
}

const CASES: [Case; 5] = [
    Case { src: 3, dst: 30 },
    Case { src: 4, dst: 40 },
    Case { src: 5, dst: 40 },
    Case { src: 10, dst: 777 },
    Case { src: 50, dst: 4000 },
];

/// Vertical pass: a single-column (width==1) ramp resized in height only.
fn run_vertical_luma8(case: &Case) -> i32 {
    let Case { src, dst } = *case;
    let src_img: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_fn(1, src, |_x, y| Luma([ramp_u8(y, src)]));
    let reference = imageops::resize(&src_img, 1, dst, FilterType::Triangle);
    let mut dyn_img = DynamicImage::ImageLuma8(src_img);
    dyn_img.resize_exact(1, dst, FilterType::Triangle);
    let subject = dyn_img.to_luma8();

    let margin = (dst / 20).max(2);
    let mut max_diff = 0i32;
    for i in margin..(dst - margin) {
        let r = reference.get_pixel(0, i).0[0] as i32;
        let s = subject.get_pixel(0, i).0[0] as i32;
        max_diff = max_diff.max((r - s).abs());
    }
    println!("[vertical Luma8]  src_h={src} dst_h={dst}: max |reference - subject| = {max_diff}");
    max_diff
}

fn run_horizontal_rgb8(case: &Case) -> i32 {
    let Case { src, dst } = *case;
    let src_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(src, 1, |x, _y| {
        let v = ramp_u8(x, src);
        Rgb([v, v, v])
    });
    let reference = imageops::resize(&src_img, dst, 1, FilterType::Triangle);
    let mut dyn_img = DynamicImage::ImageRgb8(src_img);
    dyn_img.resize_exact(dst, 1, FilterType::Triangle);
    let subject = dyn_img.to_rgb8();

    let margin = (dst / 20).max(2);
    let mut max_diff = 0i32;
    for i in margin..(dst - margin) {
        let r = reference.get_pixel(i, 0).0[0] as i32;
        let s = subject.get_pixel(i, 0).0[0] as i32;
        max_diff = max_diff.max((r - s).abs());
    }
    println!("[horizontal Rgb8] src_w={src} dst_w={dst}: max |reference - subject| = {max_diff}");
    max_diff
}

fn run_horizontal_rgba8(case: &Case) -> i32 {
    let Case { src, dst } = *case;
    let src_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(src, 1, |x, _y| {
        let v = ramp_u8(x, src);
        Rgba([v, v, v, 255])
    });
    let reference = imageops::resize(&src_img, dst, 1, FilterType::Triangle);
    let mut dyn_img = DynamicImage::ImageRgba8(src_img);
    dyn_img.resize_exact(dst, 1, FilterType::Triangle);
    let subject = dyn_img.to_rgba8();

    let margin = (dst / 20).max(2);
    let mut max_diff = 0i32;
    for i in margin..(dst - margin) {
        let r = reference.get_pixel(i, 0).0[0] as i32;
        let s = subject.get_pixel(i, 0).0[0] as i32;
        max_diff = max_diff.max((r - s).abs());
    }
    println!(
        "[horizontal Rgba8] src_w={src} dst_w={dst}: max |reference - subject| = {max_diff}"
    );
    max_diff
}

fn run_horizontal_luma16(case: &Case) -> i32 {
    let Case { src, dst } = *case;
    let src_img: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_fn(src, 1, |x, _y| {
        Luma([(x as f64 * 65535.0 / (src - 1) as f64).round() as u16])
    });
    let reference = imageops::resize(&src_img, dst, 1, FilterType::Triangle);
    let mut dyn_img = DynamicImage::ImageLuma16(src_img);
    dyn_img.resize_exact(dst, 1, FilterType::Triangle);
    let subject = dyn_img.to_luma16();

    let margin = (dst / 20).max(2);
    let mut max_abs = 0i32;
    for i in margin..(dst - margin) {
        let r = reference.get_pixel(i, 0).0[0] as i32;
        let s = subject.get_pixel(i, 0).0[0] as i32;
        max_abs = max_abs.max((r - s).abs());
    }
    // Normalize to a "gray levels out of 255" scale for a threshold comparable to the u8 cases.
    let normalized = (max_abs as f64 / 65535.0 * 255.0).round() as i32;
    println!(
        "[horizontal Luma16] src_w={src} dst_w={dst}: max |reference - subject| = {max_abs} / 65535 (~{normalized}/255)"
    );
    normalized
}

fn main() {
    // Ordinary rounding noise between two independently-implemented resamplers is +-1 (out of
    // 255, or the type's equivalent scale); anything bigger is the bug.
    let mut all_ok = true;

    println!("=== Vertical pass, Luma8 ===");
    for case in &CASES {
        if run_vertical_luma8(case) > 1 {
            all_ok = false;
        }
    }

    println!("=== Horizontal pass, Rgb8 ===");
    for case in &CASES {
        if run_horizontal_rgb8(case) > 1 {
            all_ok = false;
        }
    }

    println!("=== Horizontal pass, Rgba8 ===");
    for case in &CASES {
        if run_horizontal_rgba8(case) > 1 {
            all_ok = false;
        }
    }

    println!("=== Horizontal pass, Luma16 ===");
    for case in &CASES {
        if run_horizontal_luma16(case) > 1 {
            all_ok = false;
        }
    }

    if all_ok {
        println!("PASS: DynamicImage::resize_exact agrees with imageops::resize (Triangle filter) on every case/type/orientation tried");
    } else {
        println!("FAIL: DynamicImage::resize_exact disagrees with imageops::resize by more than rounding noise");
        std::process::exit(1);
    }
}
