//! Bench for candidate 019: re-run candidate 007's `Tf`-resolution-cache fix (same
//! `patch.diff`, verbatim - identical to candidates 007/012/018) against a real, non-synthetic
//! PDF with 100-150 `Tf` calls/page reusing a small set of fonts, to narrow the frequency
//! threshold bracketed by candidate 018 to "above 34 (issue391.pdf, no change) and at or below
//! 326 (img.pdf, strong win)".
//!
//! `pdftc_900k_0007_page_4.pdf` (real corpus file, `hayro-tests/pdfs/custom/`, already part of
//! this repo's own test suite - a single extracted page from a real-world scanned/generated PDF
//! test corpus, not built for this candidate) has **147 `Tf` calls** on its one page, confirmed
//! by temporary instrumentation (an `eprintln!` in `TypedInstruction::TextFont`'s match arm, run
//! once via `render_bench`, then reverted before any benchmarking - not a byte-search, since
//! this is the same counting method candidate 018 used). Those 147 calls reuse only **7 distinct
//! font resource names** (`TT7` x56, `TT6` x34, `TT10` x28, `TT9` x21, `TT11` x5, `TT3` x2, `TT1`
//! x1) - a small set of fonts reused repeatedly, exactly the shape the backlog entry asked for.
//!
//! Renders the page N times, discards the first 10% as warmup, reports min/p50/mean/p95/max of
//! the render() time, and hashes every produced pixmap (FNV-1a over the premultiplied RGBA8
//! bytes) - every iteration and both variants (baseline/patched) must produce the same hash for
//! the run to be reported as bit-exact.
//!
//! Run with: `cargo run --release -- [iterations]` (default 300).

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};
use std::time::{Duration, Instant};

const PDF: &[u8] = include_bytes!("../../testdata/pdftc_900k_0007_page_4.pdf");

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

    let pdf = Pdf::new(PDF.to_vec()).unwrap();
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

    println!("pdf=pdftc_900k_0007_page_4 iterations={iterations} (warmup={warmup} discarded)");
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
