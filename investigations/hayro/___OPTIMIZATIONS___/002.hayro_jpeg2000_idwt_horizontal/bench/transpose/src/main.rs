//! Throwaway feasibility check: is a transpose + reuse-vertical-filter
//! approach to horizontal IDWT filtering even worth prototyping, given the
//! measured cost of transposing buffers of this size? Not wired into CI.
//! See ___OPTIMIZATIONS___/02_hayro_jpeg2000_idwt_horizontal/ for the write-up.

use std::time::Instant;

fn transpose_naive(src: &[f32], width: usize, height: usize, dst: &mut [f32]) {
    for y in 0..height {
        for x in 0..width {
            dst[x * height + y] = src[y * width + x];
        }
    }
}

// Cache-blocked transpose (32x32 tiles).
fn transpose_blocked(src: &[f32], width: usize, height: usize, dst: &mut [f32]) {
    const BLOCK: usize = 32;
    let mut y0 = 0;
    while y0 < height {
        let y1 = (y0 + BLOCK).min(height);
        let mut x0 = 0;
        while x0 < width {
            let x1 = (x0 + BLOCK).min(width);
            for y in y0..y1 {
                for x in x0..x1 {
                    dst[x * height + y] = src[y * width + x];
                }
            }
            x0 = x1;
        }
        y0 = y1;
    }
}

fn main() {
    // Representative decomposition size (~ largest sub-band of a 1241x1752 JPX tile).
    let width = 1241usize;
    let height = 1752usize;
    let mut state = 0x1234u32;
    let src: Vec<f32> = (0..width * height)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 8) as f32
        })
        .collect();
    let mut dst = vec![0f32; width * height];

    let iterations = 50;

    let mut best_naive = f64::MAX;
    for _ in 0..iterations {
        let t = Instant::now();
        transpose_naive(std::hint::black_box(&src), width, height, &mut dst);
        best_naive = best_naive.min(t.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(&dst);
    }

    let mut best_blocked = f64::MAX;
    for _ in 0..iterations {
        let t = Instant::now();
        transpose_blocked(std::hint::black_box(&src), width, height, &mut dst);
        best_blocked = best_blocked.min(t.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(&dst);
    }

    println!("{width}x{height} f32 transpose ({} elements):", width * height);
    println!("  naive:   {best_naive:.3} ms (round-trip ~{:.3} ms)", best_naive * 2.0);
    println!("  blocked: {best_blocked:.3} ms (round-trip ~{:.3} ms)", best_blocked * 2.0);
}
