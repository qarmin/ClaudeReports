//! Standalone microbenchmark for candidate 020: per-row-call overhead in the transform
//! functions dispatched from `create_transform_fn` that are NOT already memoized (unlike
//! `benches/expand_paletted.rs`'s paletted case, which builds an RGBA LUT once).
//!
//! Covers, per the candidate description: bit-depth expansion (`expand_gray_u8`,
//! `expand_gray_u8_with_trns`), 16-bit stripping (`transform_row_strip16`,
//! `expand_trns_and_strip_line16`), and tRNS application (`expand_trns_line`,
//! `expand_trns_line16`).
//!
//! Usage: bench_020 <mode> <iters> <width1> [width2] ...
//!   mode: graybits | graytrns | trns8 | trns16 | striptrns16 | strip16 | all

use png::benchable_apis::create_transform_fn;
use png::{BitDepth, ColorType, Info, Transformations};
use std::hint::black_box;
use std::time::Instant;

fn stats(mut times_ns: Vec<u64>) -> (u64, u64, u64, u64, u64) {
    let warmup = (times_ns.len() / 10).max(1).min(times_ns.len() - 1);
    times_ns.drain(..warmup);
    times_ns.sort_unstable();
    let n = times_ns.len();
    let min = times_ns[0];
    let p50 = times_ns[n / 2];
    let p95 = times_ns[(n * 95) / 100];
    let mean = times_ns.iter().sum::<u64>() / n as u64;
    let max = times_ns[n - 1];
    (min, p50, mean, p95, max)
}

/// Times `iters` calls to a freshly-created transform fn for one (color_type, bit_depth,
/// trns, transform-flags, width) configuration, and returns per-call nanosecond stats.
fn bench_one(
    label: &str,
    color_type: ColorType,
    bit_depth: BitDepth,
    trns: Option<Vec<u8>>,
    transform: Transformations,
    out_bytes_per_pixel: usize,
    width_px: usize,
    iters: usize,
) {
    let mut info = Info::default();
    info.color_type = color_type;
    info.bit_depth = bit_depth;
    info.trns = trns.map(Into::into);
    info.width = width_px as u32;
    info.height = 1;

    let row_bytes = {
        let bits_per_pixel = color_type.samples() * bit_depth as usize;
        (width_px * bits_per_pixel).div_ceil(8)
    };
    // Varied, deterministic bytes (not a constant fill) so a bit-exact comparison between two
    // binaries actually exercises both the trns-match and trns-no-match branches.
    let input: Vec<u8> = (0..row_bytes).map(|i| (i * 37 + 11) as u8).collect();

    let mut output = vec![0u8; width_px * out_bytes_per_pixel];

    let transform_fn = create_transform_fn(&info, transform).expect("create_transform_fn failed");

    // A single call is only ~10-30ns for narrow rows, close to `Instant::now()`'s own
    // overhead - time a batch of calls per sample instead of timing every individual call,
    // then divide by the batch size. This is the same reason criterion uses batched sampling.
    const BATCH: usize = 200;
    let mut times_ns = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        for _ in 0..BATCH {
            transform_fn(black_box(&input), black_box(&mut output), black_box(&info));
        }
        let dt = t0.elapsed().as_nanos() as u64;
        times_ns.push(dt / BATCH as u64);
        black_box(&output);
    }

    let (min, p50, mean, p95, max) = stats(times_ns);
    let hash = fnv1a64(&output);
    println!(
        "RESULT bench={label} width={width_px} iters={iters} batch={BATCH} min_ns={min} p50_ns={p50} mean_ns={mean} p95_ns={p95} max_ns={max} out_hash={hash:016x}"
    );
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x00000100000001B3);
    }
    h
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "all".to_string());
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(3_000);
    let widths: Vec<usize> = {
        let given: Vec<usize> = args.filter_map(|s| s.parse().ok()).collect();
        if given.is_empty() {
            vec![1, 4, 16, 64, 256, 1024, 4096]
        } else {
            given
        }
    };

    for &width in &widths {
        if mode == "graybits" || mode == "all" {
            // expand_gray_u8: no alpha added, output is 1 byte/pixel.
            bench_one(
                "graybits(gray4,noTrns)",
                ColorType::Grayscale,
                BitDepth::Four,
                None,
                Transformations::EXPAND,
                1,
                width,
                iters,
            );
        }
        if mode == "graytrns" || mode == "all" {
            // expand_gray_u8_with_trns: gray+alpha, output is 2 bytes/pixel.
            bench_one(
                "graytrns(gray4,trns)",
                ColorType::Grayscale,
                BitDepth::Four,
                Some(vec![5]),
                Transformations::EXPAND,
                2,
                width,
                iters,
            );
        }
        if mode == "trns8" || mode == "all" {
            // expand_trns_line: rgb+alpha, output is 4 bytes/pixel.
            bench_one(
                "trns8(rgb8,trns)",
                ColorType::Rgb,
                BitDepth::Eight,
                Some(vec![1, 2, 3]),
                Transformations::EXPAND,
                4,
                width,
                iters,
            );
        }
        if mode == "trns16" || mode == "all" {
            // expand_trns_line16: rgba, 16-bit samples, output is 8 bytes/pixel.
            bench_one(
                "trns16(rgb16,trns,noStrip)",
                ColorType::Rgb,
                BitDepth::Sixteen,
                Some(vec![0, 1, 0, 2, 0, 3]),
                Transformations::EXPAND,
                8,
                width,
                iters,
            );
        }
        if mode == "striptrns16" || mode == "all" {
            // expand_trns_and_strip_line16: rgba, stripped to 8-bit, output is 4 bytes/pixel.
            bench_one(
                "striptrns16(rgb16,trns,strip)",
                ColorType::Rgb,
                BitDepth::Sixteen,
                Some(vec![0, 1, 0, 2, 0, 3]),
                Transformations::EXPAND | Transformations::STRIP_16,
                4,
                width,
                iters,
            );
        }
        if mode == "strip16" || mode == "all" {
            // transform_row_strip16: byte-for-byte halving, rgba16 -> rgba8, 4 bytes/pixel.
            bench_one(
                "strip16(rgba16,noTrns,strip)",
                ColorType::Rgba,
                BitDepth::Sixteen,
                None,
                Transformations::STRIP_16,
                4,
                width,
                iters,
            );
        }
    }
}
