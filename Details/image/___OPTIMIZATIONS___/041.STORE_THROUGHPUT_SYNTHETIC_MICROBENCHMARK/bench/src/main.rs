// Synthetic, controlled microbenchmark isolating pure output-store throughput: no palette
// lookup, no bounds check at all - just N consecutive 4-byte (RGBA-pixel-sized, u32) stores per
// outer-loop iteration (N = 1/2/4/8, matching the BMP decoder's 8/4/2/1-bit outer-loop
// densities - each outer iteration processes one input byte, which packs 8/N pixel indices, so
// 8-bit writes 1 pixel/iteration, 4-bit writes 2, 2-bit writes 4, 1-bit writes 8, all for a
// fixed total pixel count, matching candidates 008/011/025/037's fixed-array bounds-check-
// elimination fix), so the total amount of store work is identical across densities and only
// the outer-loop grouping shape (how many stores happen per iteration before the loop
// counter/branch fires again) differs.
//
// Two variants per density:
//   - "natural": plain `ptr::write`, LLVM is free to optimize/vectorize/merge stores however it
//     likes (an upper bound on achievable store throughput on this machine).
//   - "volatile": `ptr::write_volatile`, forces one genuine, unmergeable store instruction per
//     pixel - closer to what the real BMP loop's un-vectorizable, data-dependent-index writes
//     actually compile to (a data-dependent palette lookup defeats auto-vectorization the same
//     way volatile does, just for a different reason - neither loop can be reduced to a
//     memset-style pattern by the optimizer).
//
// Two buffer-size regimes per density/variant:
//   - "small" (153,560 pixels ~= Core_8_Bit.bmp/Core_4_Bit.bmp's 435x353 = 153,555px scale from
//     candidates 008/011/025/037, rounded up to a multiple of 8) - fits comfortably in L2/L3.
//   - "large" (12,192,768 pixels = 4032x3024, this workspace's usual "realistic photo" scale) -
//     4 bytes/pixel = ~48.8 MB, exceeds typical L3, memory-bandwidth-bound.
//
// All 16 (density x variant x size) configs are measured in true round-robin (every config once
// per round, ROUNDS rounds) rather than back-to-back per config, per this workspace's own
// documented noise lesson (IDEAS.md, candidate 009's session: back-to-back repeats of one
// variant can get caught inside a machine-wide slow/fast "mode" that looks like a per-variant
// effect but isn't).

use std::hint::black_box;
use std::time::Instant;

const SMALL_PIXELS: usize = 153_560; // ~= Core_8_Bit.bmp/Core_4_Bit.bmp (435x353), rounded up to a multiple of 8
const LARGE_PIXELS: usize = 4032 * 3024; // this workspace's usual "realistic photo" scale
const DENSITIES: [usize; 4] = [1, 2, 4, 8];
const ROUNDS: usize = 30;

#[inline(never)]
unsafe fn fill_natural(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
    let mut val: u32 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            val = val.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    val
}

#[inline(never)]
unsafe fn fill_volatile(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
    let mut val: u32 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            val = val.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    val
}

#[derive(Clone, Copy)]
enum Variant {
    Natural,
    Volatile,
}

struct Config {
    size_name: &'static str,
    total_pixels: usize,
    density: usize,
    variant: Variant,
    variant_name: &'static str,
}

fn main() {
    let mut configs = Vec::new();
    for &(size_name, total_pixels) in &[("small", SMALL_PIXELS), ("large", LARGE_PIXELS)] {
        for &density in &DENSITIES {
            for &(variant, variant_name) in &[
                (Variant::Natural, "natural"),
                (Variant::Volatile, "volatile"),
            ] {
                assert_eq!(
                    total_pixels % density,
                    0,
                    "pixel count must divide evenly by density"
                );
                configs.push(Config {
                    size_name,
                    total_pixels,
                    density,
                    variant,
                    variant_name,
                });
            }
        }
    }

    // One persistent buffer per distinct size, reused (and re-touched every round) across all
    // configs of that size so paging/first-touch cost isn't part of the timed region.
    let mut small_buf = vec![0u32; SMALL_PIXELS];
    let mut large_buf = vec![0u32; LARGE_PIXELS];

    // times_ns[config_index][round]
    let mut times_ns: Vec<Vec<u64>> = vec![Vec::with_capacity(ROUNDS); configs.len()];
    let mut checksum: u64 = 0;

    for _round in 0..ROUNDS {
        for (ci, cfg) in configs.iter().enumerate() {
            let buf: &mut [u32] = match cfg.size_name {
                "small" => &mut small_buf,
                "large" => &mut large_buf,
                _ => unreachable!(),
            };
            let ptr = black_box(buf.as_mut_ptr());
            let total_pixels = black_box(cfg.total_pixels);
            let density = black_box(cfg.density);

            let t0 = Instant::now();
            let ret = match cfg.variant {
                Variant::Natural => unsafe { fill_natural(ptr, total_pixels, density) },
                Variant::Volatile => unsafe { fill_volatile(ptr, total_pixels, density) },
            };
            let ns = t0.elapsed().as_nanos() as u64;

            checksum = checksum.wrapping_add(black_box(ret) as u64);
            times_ns[ci].push(ns);
        }
    }

    println!("checksum(ignore) = {checksum:#x}");
    println!("rounds = {ROUNDS} (warmup discarded = {})", (ROUNDS / 10).max(1));
    println!();
    println!(
        "{:<7} {:>8} {:>9} {:>10} {:>10} {:>10} {:>10} {:>12} {:>10}",
        "size", "density", "variant", "min_ns", "p50_ns", "mean_ns", "max_ns", "GB/s(p50)", "ns/px(p50)"
    );

    for (ci, cfg) in configs.iter().enumerate() {
        let warmup = (ROUNDS / 10).max(1).min(ROUNDS - 1);
        let mut stable = times_ns[ci][warmup..].to_vec();
        stable.sort_unstable();
        let n = stable.len();
        let min = stable[0];
        let p50 = stable[n / 2];
        let mean = stable.iter().sum::<u64>() / n as u64;
        let max = stable[n - 1];
        let total_bytes = cfg.total_pixels * 4;
        let gbps = (total_bytes as f64) / (p50 as f64); // bytes/ns = GB/s
        let ns_per_px = (p50 as f64) / (cfg.total_pixels as f64);

        println!(
            "{:<7} {:>8} {:>9} {:>10} {:>10} {:>10} {:>10} {:>12.3} {:>10.4}",
            cfg.size_name, cfg.density, cfg.variant_name, min, p50, mean, max, gbps, ns_per_px
        );
    }

    println!();
    print!("raw_ns = {{");
    for (ci, cfg) in configs.iter().enumerate() {
        print!(
            "\"{}x{}x{}\": {:?}, ",
            cfg.size_name, cfg.density, cfg.variant_name, times_ns[ci]
        );
    }
    println!("}}");
}
