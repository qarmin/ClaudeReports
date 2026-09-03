//! Bench reused verbatim from candidate 029 (itself re-running candidate 007's `Tf`-resolution-
//! cache fix, identical `patch.diff` to candidates 007/012/018/019/029) against a real,
//! non-synthetic PDF page with 218 `Tf` calls - a call frequency candidate 029 already
//! characterized as a small, borderline-resolvable win (~-1.4% to -1.8%, right at this bench's
//! noise floor). Candidate 035 does not change the bench or the patch; it re-measures this exact
//! pair under today's session's noisy host (see ../RESULT.md) to contrast the naive
//! twice-run-per-variant methodology (candidates 007/012/018) against `../tools/paired.sh`'s
//! alternating-invocation win-count when `uptime` is fluctuating at measurement time, not just
//! elevated once.
//!
//! `font_truetype_7.pdf` (real corpus file, `hayro-tests/pdfs/custom/`, already part of this
//! repo's own test suite) has 7 pages; page index 6 (the 7th page) has **218 `Tf` calls**,
//! confirmed by temporary instrumentation (an `eprintln!` in `TypedInstruction::TextFont`'s match
//! arm plus a small scratch `hayro/examples/tf_probe.rs` driver that rendered each page
//! individually and counted stderr lines between `PAGE_BOUNDARY` markers, run once, then
//! reverted before any benchmarking - same counting method as candidates 018/019, not a raw
//! byte-search, since a byte-search overcounts for this file: it turned out uncompressed so a
//! naive `grep -c ' Tf'` was directionally right but not page-precise). Those 218 calls reuse
//! only **7 distinct font resource names**: `F7` (81x), `F8` (78x), `F11` (27x), `F10` (22x),
//! `F20` (5x), `F18` (4x), `F2` (1x) - squarely in the requested 200-300 range, with the same
//! "small set of fonts reused repeatedly" shape as candidates 019/018's PDFs.
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

const PDF: &[u8] = include_bytes!("../../testdata/font_truetype_7.pdf");
const PAGE_INDEX: usize = 6;

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
    let page = pdf.pages().iter().nth(PAGE_INDEX).unwrap();

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

    println!("pdf=font_truetype_7 page={PAGE_INDEX} iterations={iterations} (warmup={warmup} discarded)");
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
