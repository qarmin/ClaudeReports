// Follow-up to candidate 041's pure-store-throughput microbenchmark. That candidate isolated
// only the store cost of the BMP decode per-pixel loop (no palette lookup, no bounds check),
// leaving palette lookup + bit-field unpacking + the bounds check itself lumped together as one
// residual against the real whole-decode time. This variant adds a cheap array-index lookup
// (still branch-free - a fixed 256-entry palette array indexed by a wrapping monotonic u8
// counter, using `get_unchecked` so there is still no bounds check anywhere in the loop) on top
// of the same store, at the same four densities (1/2/4/8, matching the BMP decoder's 8/4/2/1-bit
// outer-loop shapes), to isolate the lookup's own added cost from the store's.
//
// Four variants per density:
//   - "store_natural" / "store_volatile": identical to candidate 041's `fill_natural`/
//     `fill_volatile` (no lookup) - kept here, unmodified in behavior, as the same-session
//     baseline this candidate's lookup-inclusive numbers are compared against (see
//     MEASUREMENT.md's paired-comparison discipline - avoids re-using a different session's
//     numbers for the "no lookup" side of the comparison).
//   - "lookup_natural" / "lookup_volatile": same store, but the value written is read from
//     `palette[idx as usize]` first (`idx` a wrapping `u8`, `palette` a `[u32; 256]` - the index
//     never needs range-clamping since `u8::wrapping_add` already stays in `0..256`, so
//     `get_unchecked` is used purely to guarantee no bounds-check branch is emitted, not because
//     the index could otherwise go out of range).
//
// Two buffer-size regimes per density/variant, same as candidate 041:
//   - "small" (153,560 pixels ~= Core_8_Bit.bmp/Core_4_Bit.bmp's 435x353 scale) - fits in L2/L3.
//   - "large" (12,192,768 pixels = 4032x3024, ~48.8 MB) - exceeds typical L3.
//
// All (density x variant x size) configs are measured in true round-robin (every config once per
// round, ROUNDS rounds) per this workspace's own documented noise lesson (IDEAS.md, candidate
// 009's session).

use std::hint::black_box;
use std::time::Instant;

const SMALL_PIXELS: usize = 153_560; // ~= Core_8_Bit.bmp/Core_4_Bit.bmp (435x353), rounded up to a multiple of 8
const LARGE_PIXELS: usize = 4032 * 3024; // this workspace's usual "realistic photo" scale
const DENSITIES: [usize; 4] = [1, 2, 4, 8];
const ROUNDS: usize = 30;

#[inline(never)]
unsafe fn fill_store_natural(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
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
unsafe fn fill_store_volatile(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
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

#[inline(never)]
unsafe fn fill_lookup_natural(buf: *mut u32, total_pixels: usize, density: usize, palette: &[u32; 256]) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    idx as u32
}

#[inline(never)]
unsafe fn fill_lookup_volatile(buf: *mut u32, total_pixels: usize, density: usize, palette: &[u32; 256]) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    idx as u32
}

#[derive(Clone, Copy)]
enum Variant {
    StoreNatural,
    StoreVolatile,
    LookupNatural,
    LookupVolatile,
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
                (Variant::StoreNatural, "store_natural"),
                (Variant::StoreVolatile, "store_volatile"),
                (Variant::LookupNatural, "lookup_natural"),
                (Variant::LookupVolatile, "lookup_volatile"),
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

    // Fixed 256-entry palette, non-trivial values so the compiler can't fold the lookup into a
    // constant - still fits comfortably in L1 (1 KiB), so no cache-miss cost is intended here.
    let mut palette = [0u32; 256];
    for (i, p) in palette.iter_mut().enumerate() {
        *p = (i as u32).wrapping_mul(0x9E3779B1) ^ 0xDEADBEEF;
    }
    let palette = black_box(palette);

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
                Variant::StoreNatural => unsafe { fill_store_natural(ptr, total_pixels, density) },
                Variant::StoreVolatile => unsafe { fill_store_volatile(ptr, total_pixels, density) },
                Variant::LookupNatural => unsafe { fill_lookup_natural(ptr, total_pixels, density, &palette) },
                Variant::LookupVolatile => unsafe { fill_lookup_volatile(ptr, total_pixels, density, &palette) },
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
        "{:<7} {:>8} {:>15} {:>10} {:>10} {:>10} {:>10} {:>12} {:>10}",
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
            "{:<7} {:>8} {:>15} {:>10} {:>10} {:>10} {:>10} {:>12.3} {:>10.4}",
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
