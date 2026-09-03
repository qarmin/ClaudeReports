//! Re-test of the RGB->RGBA candidates from
//! `../../01_hayro_image_rgba_scalar/`, this time against the *actual*
//! image shape found in the user's `img.pdf` (8 small images: four 6x8,
//! two 90x120, one 180x240, one 2154x90) instead of one large synthetic
//! buffer - fixed per-call overhead (allocation, etc.) may dominate
//! differently at this scale.
//!
//! Run with: `cargo run --release -- [iterations]`

use std::time::{Duration, Instant};

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// --- Candidate implementations (same as 01_hayro_image_rgba_scalar/candidate_patches/) ---

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

// img.pdf's actual embedded image dimensions (width, height), DeviceRGB, no alpha.
const IMAGE_SIZES: &[(usize, usize)] = &[
    (180, 240),
    (90, 120),
    (90, 120),
    (6, 8),
    (6, 8),
    (6, 8),
    (6, 8),
    (2154, 90),
];

fn synthetic_images() -> Vec<Vec<u8>> {
    let mut state = 0x12345678u32;
    IMAGE_SIZES
        .iter()
        .map(|(w, h)| {
            let mut buf = vec![0u8; w * h * 3];
            for b in buf.iter_mut() {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                *b = (state >> 24) as u8;
            }
            buf
        })
        .collect()
}

fn run_bench(
    name: &str,
    images: &[Vec<u8>],
    f: impl Fn(&[u8]) -> Vec<u8>,
    iterations: usize,
    expected_hash: u64,
) {
    let mut durations = Vec::with_capacity(iterations);
    let mut last_hash = 0u64;

    for i in 0..iterations {
        let start = Instant::now();
        let mut combined_hash = 0xcbf29ce484222325u64;
        for img in images {
            let out = f(img);
            combined_hash ^= fnv1a(&out);
            std::hint::black_box(&out);
        }
        let elapsed = start.elapsed();

        if i == 0 {
            last_hash = combined_hash;
        } else if combined_hash != last_hash {
            panic!("[{name}] hash mismatch between iterations");
        }
        durations.push(elapsed);
    }

    if last_hash != expected_hash {
        panic!("[{name}] hash mismatch vs expected: got {last_hash:x}, expected {expected_hash:x}");
    }

    let warmup = (iterations / 10).max(1);
    let mut sorted: Vec<Duration> = durations[warmup..].to_vec();
    sorted.sort();
    let n = sorted.len();
    let min = sorted[0];
    let p50 = sorted[n / 2];
    let mean: Duration = sorted.iter().sum::<Duration>() / n as u32;
    let max = sorted[n - 1];

    println!(
        "[{name:20}] min={min:>9.2?} p50={p50:>9.2?} mean={mean:>9.2?} max={max:>9.2?} hash={last_hash:016x}"
    );
}

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);

    let images = synthetic_images();
    let total_pixels: usize = IMAGE_SIZES.iter().map(|(w, h)| w * h).sum();
    println!(
        "8 images matching img.pdf's shape ({total_pixels} total pixels), {iterations} iterations"
    );

    let mut expected_hash = 0xcbf29ce484222325u64;
    for img in &images {
        expected_hash ^= fnv1a(&baseline_flat_map(img));
    }
    println!("expected_hash={expected_hash:016x}");

    run_bench("baseline_flat_map", &images, baseline_flat_map, iterations, expected_hash);
    run_bench("manual_loop_zip", &images, manual_loop_zip, iterations, expected_hash);
    run_bench("map_array_collect", &images, map_array_collect, iterations, expected_hash);
    run_bench("manual_loop_u32_write", &images, manual_loop_u32_write, iterations, expected_hash);
}
