// Standalone perf-measurement harness used for this candidate's clean (no-criterion-overhead)
// perf stat comparison. Built as a temporary examples/_perfcheck_055.rs against each source
// variant in turn (unpatched baseline / fast path with #[inline(always)] / fast path without the
// annotation / fast path using chunks_exact_mut instead of pixels_from_channels_mut), the
// resulting binary copied out, then the source reverted before building the next variant. Not
// part of patch.diff - this file never lived in the real source tree at the same time as any
// other real-source change.
use std::hint::black_box;
use std::time::Instant;

use image::{imageops, ImageBuffer, Rgb};

fn main() {
    let src: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(1920, 1080, |x, y| {
        Rgb([x as u8, (y / 8) as u8, ((x * 13 + y) % 256) as u8])
    });

    let iters: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50);

    let start = Instant::now();
    for _ in 0..iters {
        let mut img = src.clone();
        imageops::contrast_in_place(&mut img, 5.0);
        black_box(&img);
    }
    let elapsed = start.elapsed();
    println!(
        "{} iters in {:?} ({:?}/iter)",
        iters,
        elapsed,
        elapsed / iters as u32
    );
}
