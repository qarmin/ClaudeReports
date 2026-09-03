// Standalone microbenchmark for `Brush::interpolate` (i-slint-core, internal/core/graphics/brush.rs).
//
// Isolates exactly the code path named in the candidate: an animation calling
// `Brush::interpolate` once per frame on a gradient brush. Measures wall time per call and
// allocator traffic (count + bytes), and prints a checksum of every produced Brush so two builds
// (baseline vs patched) can be compared for bit-exactness.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use i_slint_core::graphics::{Brush, Color, ConicGradientBrush, GradientStop, LinearGradientBrush, RadialGradientBrush};
use i_slint_core::properties::InterpolatedPropertyValue;

struct CountingAlloc;

static ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

fn stop(pos: f32, r: u8, g: u8, b: u8, a: u8) -> GradientStop {
    GradientStop { color: Color::from_argb_u8(a, r, g, b), position: pos }
}

fn make_linear(n_stops: usize, offset: u8) -> LinearGradientBrush {
    let stops: Vec<GradientStop> = (0..n_stops)
        .map(|i| {
            let p = i as f32 / (n_stops.max(2) - 1) as f32;
            stop(p, (i as u8).wrapping_add(offset), 128, 255u8.wrapping_sub(offset), 255)
        })
        .collect();
    LinearGradientBrush::new(45.0, stops)
}

fn make_radial(n_stops: usize, offset: u8) -> RadialGradientBrush {
    let stops: Vec<GradientStop> = (0..n_stops)
        .map(|i| {
            let p = i as f32 / (n_stops.max(2) - 1) as f32;
            stop(p, (i as u8).wrapping_add(offset), 64, 200u8.wrapping_sub(offset), 255)
        })
        .collect();
    RadialGradientBrush::new_circle(stops)
}

fn make_conic(n_stops: usize, offset: u8) -> ConicGradientBrush {
    let stops: Vec<GradientStop> = (0..n_stops)
        .map(|i| {
            let p = i as f32 / (n_stops.max(2) - 1) as f32;
            stop(p, (i as u8).wrapping_add(offset), 32, 220u8.wrapping_sub(offset), 255)
        })
        .collect();
    ConicGradientBrush::new(10.0, stops)
}

fn checksum(b: &Brush) -> u64 {
    // Order/value-sensitive but not collision-proof; good enough to catch any behavior change
    // in the interpolated output between baseline and patched builds.
    let mut h: u64 = 1469598103934665603; // FNV offset basis
    let mix = |h: &mut u64, byte: u8| {
        *h ^= byte as u64;
        *h = h.wrapping_mul(1099511628211);
    };
    match b {
        Brush::SolidColor(c) => {
            mix(&mut h, 0);
            for byte in c.as_argb_encoded().to_le_bytes() {
                mix(&mut h, byte);
            }
        }
        Brush::LinearGradient(g) => {
            mix(&mut h, 1);
            for s in g.stops() {
                for byte in s.color.as_argb_encoded().to_le_bytes() {
                    mix(&mut h, byte);
                }
                for byte in s.position.to_bits().to_le_bytes() {
                    mix(&mut h, byte);
                }
            }
        }
        Brush::RadialGradient(g) => {
            mix(&mut h, 2);
            for s in g.stops() {
                for byte in s.color.as_argb_encoded().to_le_bytes() {
                    mix(&mut h, byte);
                }
                for byte in s.position.to_bits().to_le_bytes() {
                    mix(&mut h, byte);
                }
            }
        }
        Brush::ConicGradient(g) => {
            mix(&mut h, 3);
            for s in g.stops() {
                for byte in s.color.as_argb_encoded().to_le_bytes() {
                    mix(&mut h, byte);
                }
                for byte in s.position.to_bits().to_le_bytes() {
                    mix(&mut h, byte);
                }
            }
        }
        _ => unreachable!("Brush is non-exhaustive but the bench only produces the 4 known variants"),
    }
    h
}

fn percentile(sorted: &[u128], p: f64) -> u128 {
    if sorted.is_empty() {
        return 0;
    }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.iter().position(|a| a == "--mode").map(|i| args[i + 1].clone()).unwrap_or_else(|| "linear".to_string());
    let n_stops: usize = args
        .iter()
        .position(|a| a == "--stops")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(8);
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(2_000_000);
    let batch: usize = args
        .iter()
        .position(|a| a == "--batch")
        .map(|i| args[i + 1].parse().unwrap())
        .unwrap_or(1000);

    let (from, to): (Brush, Brush) = match mode.as_str() {
        "linear" => (Brush::LinearGradient(make_linear(n_stops, 10)), Brush::LinearGradient(make_linear(n_stops, 200))),
        "radial" => (Brush::RadialGradient(make_radial(n_stops, 10)), Brush::RadialGradient(make_radial(n_stops, 200))),
        "conic" => (Brush::ConicGradient(make_conic(n_stops, 10)), Brush::ConicGradient(make_conic(n_stops, 200))),
        "solid-linear" => {
            (Brush::SolidColor(Color::from_argb_u8(255, 10, 20, 30)), Brush::LinearGradient(make_linear(n_stops, 200)))
        }
        "solid-radial" => {
            (Brush::SolidColor(Color::from_argb_u8(255, 10, 20, 30)), Brush::RadialGradient(make_radial(n_stops, 200)))
        }
        "solid-conic" => {
            (Brush::SolidColor(Color::from_argb_u8(255, 10, 20, 30)), Brush::ConicGradient(make_conic(n_stops, 200)))
        }
        other => panic!("unknown --mode {other}"),
    };

    let warmup = iters / 10;
    let mut checksum_acc: u64 = 0;
    let mut batch_durations: Vec<u128> = Vec::with_capacity(iters / batch + 1);

    let n_batches = iters / batch;
    let mut done = 0usize;
    for _ in 0..n_batches {
        let start = Instant::now();
        for j in 0..batch {
            let idx = done + j;
            let t = (idx % 1000) as f32 / 1000.0;
            let result = black_box(&from).interpolate(black_box(&to), black_box(t));
            checksum_acc ^= checksum(&result).rotate_left((idx % 61) as u32);
            black_box(&result);
        }
        let elapsed = start.elapsed().as_nanos();
        done += batch;
        if done > warmup {
            batch_durations.push(elapsed / batch as u128);
        }
    }

    batch_durations.sort_unstable();
    let min = batch_durations.first().copied().unwrap_or(0);
    let max = batch_durations.last().copied().unwrap_or(0);
    let p50 = percentile(&batch_durations, 0.50);
    let p95 = percentile(&batch_durations, 0.95);
    let mean: f64 = batch_durations.iter().sum::<u128>() as f64 / batch_durations.len().max(1) as f64;

    let alloc_count_before_reset = ALLOC_COUNT.load(Ordering::Relaxed);
    let alloc_bytes_before_reset = ALLOC_BYTES.load(Ordering::Relaxed);

    println!("mode={mode} stops={n_stops} iters={iters} batch={batch}");
    println!("checksum=0x{checksum_acc:016x}");
    println!("per_call_ns min={min} p50={p50} mean={mean:.1} p95={p95} max={max}");
    println!(
        "allocations: total_count={alloc_count_before_reset} total_bytes={alloc_bytes_before_reset} avg_count_per_call={:.4} avg_bytes_per_call={:.4}",
        alloc_count_before_reset as f64 / iters as f64,
        alloc_bytes_before_reset as f64 / iters as f64
    );
}
