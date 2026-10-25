// Standalone bench for premultiply_u8/premultiply_u16 in zune-imageprocs, reaching
// crates/zune-imageprocs/src/premul_alpha.rs's functions directly (no existing project
// bench/example reached these free functions in isolation - the workspace criterion bench,
// benchmarks/benches/imageprocs.rs, requires libvips as a system dependency, which is not
// available in this environment, and even when available only measures the full
// PremultiplyAlpha::execute_impl operation, not this hot inner loop alone).
// See ___OPTIMIZATIONS___/005.PREMUL_ALPHA_U8_U16_SIMD/RESULT.md.
//
// Usage:
//   bench_premul_alpha <pixels> <iters> <depth: u8|u16>
//
// Simulates one RGB(A) frame: three color channels, each of length `pixels`, each multiplied
// in place against the same alpha channel (also length `pixels`) - exactly the loop
// PremultiplyAlpha::execute_impl runs per color channel. Builds the buffers once, deep-copies
// them fresh every iteration (the function mutates its input in place), and hashes the result
// every iteration to confirm bit-stability.

use std::env;
use std::process;
use std::time::Instant;

use zune_imageprocs::premul_alpha::{premultiply_u16, premultiply_u8};

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

fn make_u8(len: usize, seed: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(len);
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        v.push((state & 0xff) as u8);
    }
    v
}

fn make_u16(len: usize, seed: u32) -> Vec<u16> {
    let mut v = Vec::with_capacity(len);
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        v.push((state & 0xffff) as u16);
    }
    v
}

fn main() {
    let mut args = env::args().skip(1);
    let pixels: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: bench_premul_alpha <pixels> <iters> <depth: u8|u16>");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let depth = args.next().unwrap_or_else(|| "u8".to_string());

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    match depth.as_str() {
        "u8" => {
            let alpha = make_u8(pixels, 0xDEAD_BEEF);
            let r0 = make_u8(pixels, 0xC0FF_EE11);
            let g0 = make_u8(pixels, 0xC0FF_EE22);
            let b0 = make_u8(pixels, 0xC0FF_EE33);

            for i in 0..iters {
                let mut r = r0.clone();
                let mut g = g0.clone();
                let mut b = b0.clone();

                let t0 = Instant::now();
                premultiply_u8(std::hint::black_box(&mut r), &alpha);
                premultiply_u8(std::hint::black_box(&mut g), &alpha);
                premultiply_u8(std::hint::black_box(&mut b), &alpha);
                let us = t0.elapsed().as_micros() as u64;

                let mut bytes = Vec::with_capacity(pixels * 3);
                bytes.extend_from_slice(&r);
                bytes.extend_from_slice(&g);
                bytes.extend_from_slice(&b);
                let h = fnv1a64(&bytes);
                match first_hash {
                    None => first_hash = Some(h),
                    Some(prev) => {
                        assert_eq!(h, prev, "iter {i} hash {h:016x} differs from iter 0 {prev:016x}")
                    }
                }
                times_us.push(us);
            }
        }
        "u16" => {
            let alpha = make_u16(pixels, 0xDEAD_BEEF);
            let r0 = make_u16(pixels, 0xC0FF_EE11);
            let g0 = make_u16(pixels, 0xC0FF_EE22);
            let b0 = make_u16(pixels, 0xC0FF_EE33);

            for i in 0..iters {
                let mut r = r0.clone();
                let mut g = g0.clone();
                let mut b = b0.clone();

                let t0 = Instant::now();
                premultiply_u16(std::hint::black_box(&mut r), &alpha);
                premultiply_u16(std::hint::black_box(&mut g), &alpha);
                premultiply_u16(std::hint::black_box(&mut b), &alpha);
                let us = t0.elapsed().as_micros() as u64;

                let mut bytes = Vec::with_capacity(pixels * 3 * 2);
                for v in r.iter().chain(g.iter()).chain(b.iter()) {
                    bytes.extend_from_slice(&v.to_le_bytes());
                }
                let h = fnv1a64(&bytes);
                match first_hash {
                    None => first_hash = Some(h),
                    Some(prev) => {
                        assert_eq!(h, prev, "iter {i} hash {h:016x} differs from iter 0 {prev:016x}")
                    }
                }
                times_us.push(us);
            }
        }
        other => {
            eprintln!("unknown depth {other}, expected u8|u16");
            process::exit(1);
        }
    }

    let hash = first_hash.unwrap();
    let warmup = (iters / 10).max(1).min(iters - 1);
    let stable: Vec<u64> = {
        let mut s = times_us[warmup..].to_vec();
        s.sort_unstable();
        s
    };
    let n = stable.len();
    let min = stable[0];
    let p50 = stable[n / 2];
    let p95 = stable[(n * 95) / 100];
    let mean = stable.iter().sum::<u64>() / n as u64;
    let max = stable[n - 1];

    println!("depth      = {depth}");
    println!("pixels     = {pixels} (x3 channels)");
    println!("iters      = {iters} (warmup discarded = {warmup})");
    println!("hash       = {hash:016x}  (STABLE across all iters)");
    println!("min        = {:8.3} ms", min as f64 / 1000.0);
    println!("p50        = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean       = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95        = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max        = {:8.3} ms", max as f64 / 1000.0);
    println!();
    print!("raw_us     = [");
    for (i, t) in times_us.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{t}");
    }
    println!("]");
}
