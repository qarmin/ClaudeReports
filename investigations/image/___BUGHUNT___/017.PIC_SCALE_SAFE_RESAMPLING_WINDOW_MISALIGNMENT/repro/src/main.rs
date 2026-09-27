//! Standalone repro for ___BUGHUNT___ candidate 017.
//!
//! Real end-to-end trigger path: `image::DynamicImage::resize_exact` (the crate's actual public
//! resize API, which delegates internally to the `pic-scale-safe` dependency), compared against
//! `image::imageops::resize` (the crate's own separate, independent pure-Rust legacy resize
//! implementation, used here only as a correctness oracle). No hand-constructed internal state -
//! both sides are driven purely through `image`'s public API on a real `ImageBuffer`.
//!
//! For a monotonic ramp source and a linear (partition-of-unity, symmetric) resampling kernel
//! like Triangle/Bilinear, both implementations should agree to within ordinary +-1 rounding
//! noise. A real disagreement of many gray levels, especially one that "plateaus" (repeats the
//! same output value across several consecutive output pixels instead of advancing smoothly),
//! indicates the wrong pair of source pixels is being blended for those output positions.

use image::{imageops, imageops::FilterType, DynamicImage, ImageBuffer, Luma};

struct Case {
    src_w: u32,
    dst_w: u32,
}

fn run_case(case: &Case) -> bool {
    let Case { src_w, dst_w } = *case;
    let src: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(src_w, 1, |x, _y| {
        Luma([(x as f64 * 255.0 / (src_w - 1) as f64).round() as u8])
    });

    // Oracle: the crate's own independent legacy pure-Rust implementation.
    let reference = imageops::resize(&src, dst_w, 1, FilterType::Triangle);

    // Subject: the real public entry point, `DynamicImage::resize_exact`, which is backed by
    // `pic-scale-safe` (see src/imageops/resize.rs in the `image` crate).
    let mut dyn_img = DynamicImage::ImageLuma8(src);
    dyn_img.resize_exact(dst_w, 1, FilterType::Triangle);
    let subject = dyn_img.to_luma8();

    let margin = (dst_w / 20).max(2);
    let mut max_diff = 0i32;
    let mut max_diff_i = 0u32;
    for i in margin..(dst_w - margin) {
        let r = reference.get_pixel(i, 0).0[0] as i32;
        let s = subject.get_pixel(i, 0).0[0] as i32;
        let diff = (r - s).abs();
        if diff > max_diff {
            max_diff = diff;
            max_diff_i = i;
        }
    }

    println!(
        "src_w={src_w} dst_w={dst_w}: max |reference - subject| = {max_diff} (at output index {max_diff_i})"
    );

    // Ordinary rounding noise between two independent implementations is +-1; anything bigger
    // is the bug.
    max_diff <= 1
}

fn main() {
    let cases = [
        Case { src_w: 3, dst_w: 30 },
        Case { src_w: 4, dst_w: 40 },
        Case { src_w: 5, dst_w: 40 },
        Case { src_w: 10, dst_w: 777 },
        Case { src_w: 50, dst_w: 4000 },
    ];

    let mut all_ok = true;
    for case in &cases {
        if !run_case(case) {
            all_ok = false;
        }
    }

    if all_ok {
        println!("PASS: DynamicImage::resize_exact agrees with imageops::resize (Triangle filter) on every case");
    } else {
        println!("FAIL: DynamicImage::resize_exact disagrees with imageops::resize by more than rounding noise");
        std::process::exit(1);
    }
}
