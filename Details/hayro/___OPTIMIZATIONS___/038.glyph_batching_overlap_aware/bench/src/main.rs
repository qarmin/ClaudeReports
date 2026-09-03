//! Bench for candidate 038: same as candidate 009's `vello_cpu::fill_path` glyph batching
//! (reused verbatim - identical to candidate 009's own bench), but exercising the
//! overlap-aware variant that only merges glyphs whose transformed bounding boxes are proven
//! not to overlap.
//!
//! Renders `../testdata/img.pdf` (the same real test file used throughout this workspace) N
//! times, discards the first 10% as warmup, reports min/p50/mean/p95/max of the render() time,
//! and hashes every produced pixmap (FNV-1a over the premultiplied RGBA8 bytes) - every
//! iteration and both variants (baseline/patched) must produce the same hash for the run to be
//! reported as bit-exact.
//!
//! Run with: `cargo run --release -- [iterations]`

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::{Duration, Instant};

const IMG_PDF: &[u8] = include_bytes!("../../testdata/img.pdf");

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);

    let pdf = Pdf::new(IMG_PDF.to_vec()).unwrap();
    let cache = RenderCache::new();
    let settings = InterpreterSettings::default();
    let page = pdf.pages().iter().next().unwrap();

    let mut durations = Vec::with_capacity(iterations);
    let mut hashes = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let t = Instant::now();
        let pixmap = render(
            &page,
            &cache,
            &settings,
            &RenderSettings {
                x_scale: 1.0,
                y_scale: 1.0,
                ..Default::default()
            },
        );
        let elapsed = t.elapsed();
        let bytes: Vec<u8> = pixmap
            .data()
            .iter()
            .flat_map(|px| px.to_u8_array())
            .collect();
        let hash = fnv1a(&bytes);
        std::hint::black_box(&pixmap);
        durations.push(elapsed);
        hashes.push(hash);
    }

    let warmup = (iterations / 10).max(1);
    let mut timed: Vec<Duration> = durations[warmup..].to_vec();
    timed.sort();

    let n = timed.len();
    let min = timed[0];
    let p50 = timed[n / 2];
    let p95 = timed[(n * 95 / 100).min(n - 1)];
    let max = timed[n - 1];
    let mean: Duration = timed.iter().sum::<Duration>() / n as u32;

    let first_hash = hashes[0];
    let all_same = hashes.iter().all(|h| *h == first_hash);

    println!("iterations={iterations} (warmup={warmup} discarded)");
    println!(
        "render() time: min={:.4}ms p50={:.4}ms mean={:.4}ms p95={:.4}ms max={:.4}ms",
        min.as_secs_f64() * 1e3,
        p50.as_secs_f64() * 1e3,
        mean.as_secs_f64() * 1e3,
        p95.as_secs_f64() * 1e3,
        max.as_secs_f64() * 1e3,
    );
    println!("pixmap hash (FNV-1a of RGBA8 bytes): {first_hash:#018x}");
    println!("all iterations produced identical pixmap hash: {all_same}");
}
