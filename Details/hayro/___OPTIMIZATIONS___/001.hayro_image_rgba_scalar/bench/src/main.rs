//! Ad-hoc benchmark harness for candidate optimizations to the RGB->RGBA
//! expansion in `src/image.rs`. Not wired into CI; used for manual perf work.
//! See ___OPTIMIZATIONS___/01_hayro_image_rgba_scalar_nullresult/ for the write-up.

use std::time::{Duration, Instant};

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// --- Candidate implementations of RGB -> RGBA (opaque alpha) expansion ---

fn baseline_flat_map(rgb: &[u8]) -> Vec<u8> {
    rgb.chunks_exact(3)
        .flat_map(|px| [px[0], px[1], px[2], 255])
        .collect::<Vec<_>>()
}

fn manual_loop_zip(rgb: &[u8]) -> Vec<u8> {
    let pixels = rgb.len() / 3;
    let mut out = vec![0u8; pixels * 4];
    for (src, dst) in rgb.chunks_exact(3).zip(out.chunks_exact_mut(4)) {
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
        dst[3] = 255;
    }
    out
}

fn manual_loop_uninit(rgb: &[u8]) -> Vec<u8> {
    let pixels = rgb.len() / 3;
    let mut out: Vec<u8> = Vec::with_capacity(pixels * 4);
    let spare = out.spare_capacity_mut();
    for (i, src) in rgb.chunks_exact(3).enumerate() {
        let base = i * 4;
        spare[base].write(src[0]);
        spare[base + 1].write(src[1]);
        spare[base + 2].write(src[2]);
        spare[base + 3].write(255);
    }
    unsafe { out.set_len(pixels * 4) };
    out
}

fn map_array_collect(rgb: &[u8]) -> Vec<u8> {
    let out: Vec<[u8; 4]> = rgb
        .chunks_exact(3)
        .map(|px| [px[0], px[1], px[2], 255])
        .collect();
    bytemuck::cast_vec(out)
}

fn manual_loop_u32_write(rgb: &[u8]) -> Vec<u8> {
    let pixels = rgb.len() / 3;
    let mut out: Vec<u32> = Vec::with_capacity(pixels);
    let spare = out.spare_capacity_mut();
    for (i, src) in rgb.chunks_exact(3).enumerate() {
        let word = u32::from_le_bytes([src[0], src[1], src[2], 255]);
        spare[i].write(word);
    }
    unsafe { out.set_len(pixels) };
    let mut bytes = Vec::with_capacity(pixels * 4);
    for w in &out {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    bytes
}

fn run_bench(
    name: &str,
    input: &[u8],
    f: impl Fn(&[u8]) -> Vec<u8>,
    iterations: usize,
    expected_hash: u64,
) {
    let mut durations = Vec::with_capacity(iterations);
    let mut last_hash = 0u64;

    for i in 0..iterations {
        let start = Instant::now();
        let out = f(input);
        let elapsed = start.elapsed();

        let h = fnv1a(&out);
        if i == 0 {
            last_hash = h;
        } else if h != last_hash {
            panic!("[{name}] hash mismatch between iterations: {h:x} vs {last_hash:x}");
        }

        durations.push(elapsed);
    }

    if last_hash != expected_hash {
        panic!(
            "[{name}] hash mismatch vs expected: got {last_hash:x}, expected {expected_hash:x}"
        );
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
        "[{name:20}] min={min:>9.2?} p50={p50:>9.2?} mean={mean:>9.2?} p95={p95:>9.2?} max={max:>9.2?} hash={last_hash:016x}"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let width = 4000usize;
    let height = 3000usize;
    let mut rgb = vec![0u8; width * height * 3];
    let mut state: u32 = 0x12345678;
    for b in rgb.iter_mut() {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        *b = (state >> 24) as u8;
    }

    let expected_hash = fnv1a(&baseline_flat_map(&rgb));
    println!("input: {width}x{height} RGB ({} bytes), iterations={iterations}", rgb.len());
    println!("expected_hash={expected_hash:016x}");

    run_bench("baseline_flat_map", &rgb, baseline_flat_map, iterations, expected_hash);
    run_bench("manual_loop_zip", &rgb, manual_loop_zip, iterations, expected_hash);
    run_bench("manual_loop_uninit", &rgb, manual_loop_uninit, iterations, expected_hash);
    run_bench("map_array_collect", &rgb, map_array_collect, iterations, expected_hash);
    run_bench("manual_loop_u32_write", &rgb, manual_loop_u32_write, iterations, expected_hash);
}
