// Temporary correctness-check harness for ___OPTIMIZATIONS___ candidate work on
// pic-scale-safe's runtime AVX2/AVX-512 multiversioning (candidates 048/049/052/053, and this
// one). Not part of any shipped patch - deleted before the working tree is reverted.
//
// Hashes `DynamicImage::resize_exact` output (DefaultHasher over `to_rgb8().into_raw()`) across
// several filters and destination sizes, on both a large procedurally-generated source (same one
// `benches/imageops.rs`'s `bench_resize_dynamic` uses) and a small one, so a baseline vs. patched
// build's hash lists can be diffed for bit-exactness.

use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgb};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_resize(src: &DynamicImage, w: u32, h: u32, filter: FilterType, label: &str) {
    let mut resized = src.clone();
    resized.resize_exact(w, h, filter);
    let mut hasher = DefaultHasher::new();
    resized.to_rgb8().into_raw().hash(&mut hasher);
    println!("{label} {w}x{h} {filter:?}: {:016x}", hasher.finish());
}

fn main() {
    let src = ImageBuffer::from_fn(4032, 3024, |x, y| {
        Rgb([x as u8, (y / 8) as u8, ((x * 13 + y) % 256) as u8])
    });
    let src_dyn = DynamicImage::from(src);

    let dst_sizes = [
        (400u32, 300u32),
        (64, 48),
        (200, 150),
        (32, 24),
        (800, 600),
        (128, 96),
        (16, 12),
        (1000, 750),
    ];
    let filters = [
        FilterType::Lanczos3,
        FilterType::Triangle,
        FilterType::CatmullRom,
        FilterType::Gaussian,
        FilterType::Nearest,
    ];

    for (w, h) in dst_sizes {
        for filter in filters {
            hash_resize(&src_dyn, w, h, filter, "large");
        }
    }

    let small_src = ImageBuffer::from_fn(400, 300, |x, y| {
        Rgb([x as u8, (y / 8) as u8, ((x * 13 + y) % 256) as u8])
    });
    let small_src_dyn = DynamicImage::from(small_src);

    let small_dst_sizes = [(32u32, 24u32), (16, 12), (200, 150)];
    for (w, h) in small_dst_sizes {
        for filter in filters {
            hash_resize(&small_src_dyn, w, h, filter, "small");
        }
    }
}
