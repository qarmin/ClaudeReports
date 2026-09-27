//! Ad-hoc benchmark harness for candidate optimizations to
//! `Pixmap::take_unpremultiplied`, which sits on the render -> PNG/JPEG
//! save path used by consumers such as hayro. Not wired into CI.
//! See ___OPTIMIZATIONS___/01_take_unpremultiplied_lut/ for the write-up.

use std::time::{Duration, Instant};
use vello_common::peniko::color::{PremulRgba8, Rgba8};

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn rgba8_bytes(v: &[Rgba8]) -> &[u8] {
    bytemuck::cast_slice(v)
}

// --- Baseline: original Pixmap::take_unpremultiplied (float division) ---

fn baseline(buf: &[PremulRgba8]) -> Vec<Rgba8> {
    buf.iter()
        .map(|&PremulRgba8 { r, g, b, a }| {
            let alpha = 255.0 / f32::from(a);
            if a != 0 {
                let unpremultiply = |component: u8| (f32::from(component) * alpha + 0.5) as u8;
                Rgba8 {
                    r: unpremultiply(r),
                    g: unpremultiply(g),
                    b: unpremultiply(b),
                    a,
                }
            } else {
                Rgba8 { r, g, b, a }
            }
        })
        .collect()
}

// --- Candidate: currently applied in src/pixmap.rs (256-entry 16.16 fixed-point LUT) ---

fn build_recip_lut() -> [u32; 256] {
    let mut lut = [0u32; 256];
    for a in 1..256usize {
        lut[a] = ((255u64 * 65536 * 2 + a as u64) / (2 * a as u64)) as u32;
    }
    lut
}

fn lut_scalar(buf: &[PremulRgba8], lut: &[u32; 256]) -> Vec<Rgba8> {
    buf.iter()
        .map(|&PremulRgba8 { r, g, b, a }| {
            if a == 0 {
                return Rgba8 { r, g, b, a };
            }
            let recip = lut[a as usize] as u64;
            let unpremultiply = |component: u8| (((component as u64 * recip) + 0x8000) >> 16) as u8;
            Rgba8 {
                r: unpremultiply(r),
                g: unpremultiply(g),
                b: unpremultiply(b),
                a,
            }
        })
        .collect()
}

fn max_abs_diff(a: &[Rgba8], b: &[Rgba8]) -> i32 {
    let mut max_diff = 0i32;
    for (x, y) in a.iter().zip(b) {
        for (cx, cy) in [(x.r, y.r), (x.g, y.g), (x.b, y.b), (x.a, y.a)] {
            max_diff = max_diff.max((cx as i32 - cy as i32).abs());
        }
    }
    max_diff
}

fn run_bench(
    name: &str,
    input: &[PremulRgba8],
    f: impl Fn(&[PremulRgba8]) -> Vec<Rgba8>,
    iterations: usize,
) -> Vec<Rgba8> {
    let mut durations = Vec::with_capacity(iterations);
    let mut last: Vec<Rgba8> = Vec::new();
    let mut last_hash = 0u64;

    for i in 0..iterations {
        let start = Instant::now();
        let out = f(input);
        let elapsed = start.elapsed();

        let h = fnv1a(rgba8_bytes(&out));
        if i == 0 {
            last_hash = h;
        } else if h != last_hash {
            panic!("[{name}] hash mismatch between iterations");
        }

        last = out;
        durations.push(elapsed);
    }

    let warmup = (iterations / 10).max(1);
    let mut sorted: Vec<Duration> = durations[warmup..].to_vec();
    sorted.sort();
    let n = sorted.len();
    let min = sorted[0];
    let p50 = sorted[n / 2];
    let p95 = sorted[(n * 95 / 100).min(n - 1)];
    let max = sorted[n - 1];
    let mean: Duration = sorted.iter().sum::<Duration>() / n as u32;

    println!(
        "[{name:14}] min={min:>9.2?} p50={p50:>9.2?} mean={mean:>9.2?} p95={p95:>9.2?} max={max:>9.2?} hash={last_hash:016x}"
    );

    last
}

fn exhaustive_correctness_check(lut: &[u32; 256]) {
    // Compare baseline vs LUT candidate over all VALID premultiplied
    // (component, alpha) pairs, i.e. component <= alpha (the Pixmap invariant).
    let mut max_diff = 0i32;
    let mut mismatches = 0u64;
    let mut total = 0u64;
    for a in 0..=255u8 {
        for component in 0..=a {
            total += 1;
            let px = [PremulRgba8 { r: component, g: component, b: component, a }];
            let base = baseline(&px)[0];
            let cand = lut_scalar(&px, lut)[0];
            let diff = (base.r as i32 - cand.r as i32).abs();
            if diff != 0 {
                mismatches += 1;
            }
            max_diff = max_diff.max(diff);
        }
    }
    println!(
        "exhaustive check (valid premultiplied domain): {mismatches} / {total} pairs differ, max abs diff = {max_diff}"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(30);

    let lut = build_recip_lut();
    exhaustive_correctness_check(&lut);

    let width = 4000usize;
    let height = 3000usize;
    let mut buf = vec![PremulRgba8 { r: 0, g: 0, b: 0, a: 0 }; width * height];
    let mut state: u32 = 0x9e3779b9;
    for px in buf.iter_mut() {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let a = (state >> 24) as u8;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let r = ((state >> 16) as u8 as u32 * a as u32 / 255) as u8;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let g = ((state >> 16) as u8 as u32 * a as u32 / 255) as u8;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let b = ((state >> 16) as u8 as u32 * a as u32 / 255) as u8;
        *px = PremulRgba8 { r, g, b, a };
    }

    println!("input: {width}x{height} pixels, iterations={iterations}");

    let base_out = run_bench("baseline", &buf, baseline, iterations);
    let lut_out = run_bench("lut_scalar", &buf, |b| lut_scalar(b, &lut), iterations);

    println!(
        "max abs diff baseline vs lut_scalar on random buffer: {}",
        max_abs_diff(&base_out, &lut_out)
    );
}
