// Follow-up to candidate 042's lookup-inclusive store-throughput microbenchmark. That candidate
// isolated the palette lookup's own added cost on top of a plain store, and found it explains
// only a small (0-6%), non-monotonic slice of real BMP decode time - leaving a large residual
// (47-75% of real decode time, growing at lower bit depths) still fully unexplained. This
// candidate adds the one remaining specific hypothesis from the 037/041/042 line that has never
// had its own isolated microbenchmark: the bit-field unpacking arithmetic that extracts N
// sub-pixel indices from a single input byte via shift/mask, on top of the same store+lookup
// loop, at the same four densities (1/2/4/8, matching the BMP decoder's 8/4/2/1-bit outer-loop
// shapes). Still no real bounds check anywhere (all indexing is either provably in-range or uses
// `get_unchecked` purely to guarantee branch-free codegen).
//
// The shift/mask patterns mirror the real decoder's own bit extraction exactly
// (src/codecs/bmp/decoder.rs `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`):
//   - density 1 (8-bit): the whole byte is the index, no shift/mask needed.
//   - density 2 (4-bit): `byte >> 4` and `byte & 0xf`.
//   - density 4 (2-bit): `(byte >> 6) & 0x3`, `(byte >> 4) & 0x3`, `(byte >> 2) & 0x3`, `byte & 0x3`.
//   - density 8 (1-bit): `(byte >> (7 - d)) & 0x1` for d in 0..8 (functionally identical to the
//     real decoder's `(idx & bit) != 0` bit-test form, same single-bit-per-subpixel extraction).
//
// Six variants per density:
//   - "store_natural" / "store_volatile": unchanged from candidates 041/042 - kept as the
//     same-session store-only baseline.
//   - "lookup_natural" / "lookup_volatile": unchanged from candidate 042 - store+lookup, no
//     unpacking (the counter driving the palette index is just a wrapping u8, not derived from
//     an input byte).
//   - "unpack_natural" / "unpack_volatile": new. Same store+lookup, but the palette index for
//     each sub-pixel is extracted from a shared input-byte buffer via the shift/mask patterns
//     above, instead of being a free-running counter - this is the one piece of per-pixel work
//     none of the earlier microbenchmarks in this line modeled.
//
// Two buffer-size regimes per density/variant, same as candidates 041/042:
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

/// Extract the `d`-th (0-based) sub-pixel palette index out of `byte`, for a given `density`,
/// using exactly the shift/mask patterns the real BMP decoder uses per bit depth
/// (src/codecs/bmp/decoder.rs `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`/
/// `set_1bit_pixel_run`).
#[inline(always)]
fn extract_index(byte: u8, density: usize, d: usize) -> u8 {
    match density {
        1 => byte,
        2 => {
            if d == 0 {
                byte >> 4
            } else {
                byte & 0xf
            }
        }
        4 => (byte >> (6 - 2 * d)) & 0x3,
        8 => (byte >> (7 - d)) & 0x1,
        _ => unreachable!(),
    }
}

#[inline(never)]
unsafe fn fill_unpack_natural(
    buf: *mut u32,
    total_pixels: usize,
    density: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < density {
            let idx = extract_index(byte, density, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += density;
        byte_idx += 1;
    }
    acc
}

#[inline(never)]
unsafe fn fill_unpack_volatile(
    buf: *mut u32,
    total_pixels: usize,
    density: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < density {
            let idx = extract_index(byte, density, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += density;
        byte_idx += 1;
    }
    acc
}

/// Same lookup/store as `fill_lookup_volatile`, but with `DENSITY` a compile-time const generic
/// instead of a runtime parameter - the fair baseline for `fill_unpack_volatile_specialized`
/// below, isolating the unpack arithmetic's own cost from the effect of specialization itself
/// (both get the same const-generic treatment, so the comparison between the two attributes only
/// the added shift/mask/byte-load work, not the specialization).
#[inline(never)]
unsafe fn fill_lookup_volatile_specialized<const DENSITY: usize>(
    buf: *mut u32,
    total_pixels: usize,
    palette: &[u32; 256],
) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < DENSITY {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += DENSITY;
    }
    idx as u32
}

/// Same extraction/lookup/store as `fill_unpack_volatile`, but with `DENSITY` a compile-time
/// const generic instead of a runtime parameter - matching how the real BMP decoder actually
/// dispatches (a separate, dedicated function per bit depth - `set_8bit_pixel_run`/
/// `set_4bit_pixel_run`/etc. - chosen once for the whole row/image, never re-checked per byte).
/// `fill_unpack_volatile` above instead re-derives which shift/mask pattern to use from a
/// runtime `density` value inside `extract_index`, which - per this candidate's own disassembly
/// check - LLVM hoists to one predictable branch per input byte (not per sub-pixel), but still
/// isn't quite what the real decoder does. This variant exists to bound how much of
/// `fill_unpack_volatile`'s added cost (over `fill_lookup_volatile`) is genuine
/// extraction-arithmetic cost vs. this synthetic-harness-only dispatch artifact.
#[inline(never)]
unsafe fn fill_unpack_volatile_specialized<const DENSITY: usize>(
    buf: *mut u32,
    total_pixels: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < DENSITY {
            let idx = extract_index(byte, DENSITY, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += DENSITY;
        byte_idx += 1;
    }
    acc
}

#[derive(Clone, Copy)]
enum Variant {
    StoreNatural,
    StoreVolatile,
    LookupNatural,
    LookupVolatile,
    UnpackNatural,
    UnpackVolatile,
    LookupVolatileSpecialized,
    UnpackVolatileSpecialized,
}

struct Config {
    size_name: &'static str,
    total_pixels: usize,
    density: usize,
    variant: Variant,
    variant_name: &'static str,
}

/// Reference (scalar, obviously-correct) sub-pixel index extraction, independent of
/// `extract_index`/the unpack loops - used only by `verify_unpack_correctness` below.
fn reference_extract(byte: u8, density: usize, d: usize) -> u8 {
    match density {
        1 => byte,
        2 => {
            if d == 0 {
                (byte & 0xf0) >> 4
            } else {
                byte & 0x0f
            }
        }
        4 => {
            let shift = 6 - 2 * d;
            (byte >> shift) & 0x3
        }
        8 => {
            let bit = 0x80u8 >> d;
            u8::from((byte & bit) != 0)
        }
        _ => unreachable!(),
    }
}

/// Run `fill_unpack_natural`/`fill_unpack_volatile` on a small buffer for every density and
/// compare every output pixel against a from-scratch reference extraction - not just a checksum.
/// Run unconditionally at startup (cheap - a few hundred pixels) so a wrong-output bug in the
/// timed loop can never silently produce a "fast but wrong" measurement.
fn verify_unpack_correctness() {
    let mut palette = [0u32; 256];
    for (i, p) in palette.iter_mut().enumerate() {
        *p = (i as u32).wrapping_mul(0x9E3779B1) ^ 0xDEADBEEF;
    }
    for &density in &DENSITIES {
        let n_bytes = 97; // arbitrary, not a multiple of anything, to catch off-by-one tail bugs
        let total_pixels = n_bytes * density;
        let input: Vec<u8> = (0..n_bytes).map(|i| (i as u32).wrapping_mul(2654435761) as u8).collect();
        let mut expected = vec![0u32; total_pixels];
        for (byte_idx, &byte) in input.iter().enumerate() {
            for d in 0..density {
                let idx = reference_extract(byte, density, d);
                let idx2 = extract_index(byte, density, d);
                assert_eq!(
                    idx, idx2,
                    "extract_index disagrees with reference at density={density} d={d} byte={byte:#x}"
                );
                expected[byte_idx * density + d] = palette[idx as usize];
            }
        }

        let mut buf_natural = vec![0xFFFF_FFFFu32; total_pixels];
        let mut buf_volatile = vec![0xFFFF_FFFFu32; total_pixels];
        unsafe {
            fill_unpack_natural(buf_natural.as_mut_ptr(), total_pixels, density, &input, &palette);
            fill_unpack_volatile(buf_volatile.as_mut_ptr(), total_pixels, density, &input, &palette);
        }
        assert_eq!(
            buf_natural, expected,
            "fill_unpack_natural produced wrong pixels at density={density}"
        );
        assert_eq!(
            buf_volatile, expected,
            "fill_unpack_volatile produced wrong pixels at density={density}"
        );

        let mut buf_specialized = vec![0xFFFF_FFFFu32; total_pixels];
        unsafe {
            match density {
                1 => fill_unpack_volatile_specialized::<1>(buf_specialized.as_mut_ptr(), total_pixels, &input, &palette),
                2 => fill_unpack_volatile_specialized::<2>(buf_specialized.as_mut_ptr(), total_pixels, &input, &palette),
                4 => fill_unpack_volatile_specialized::<4>(buf_specialized.as_mut_ptr(), total_pixels, &input, &palette),
                8 => fill_unpack_volatile_specialized::<8>(buf_specialized.as_mut_ptr(), total_pixels, &input, &palette),
                _ => unreachable!(),
            }
        };
        assert_eq!(
            buf_specialized, expected,
            "fill_unpack_volatile_specialized produced wrong pixels at density={density}"
        );
    }
    eprintln!("verify_unpack_correctness: OK (all 4 densities, per-pixel exact match against reference)");
}

fn main() {
    verify_unpack_correctness();

    let mut configs = Vec::new();
    for &(size_name, total_pixels) in &[("small", SMALL_PIXELS), ("large", LARGE_PIXELS)] {
        for &density in &DENSITIES {
            for &(variant, variant_name) in &[
                (Variant::StoreNatural, "store_natural"),
                (Variant::StoreVolatile, "store_volatile"),
                (Variant::LookupNatural, "lookup_natural"),
                (Variant::LookupVolatile, "lookup_volatile"),
                (Variant::UnpackNatural, "unpack_natural"),
                (Variant::UnpackVolatile, "unpack_volatile"),
                (Variant::LookupVolatileSpecialized, "lookup_volatile_specialized"),
                (Variant::UnpackVolatileSpecialized, "unpack_volatile_specialized"),
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

    // Input-byte buffers for the "unpack" variants, one per size, sized generously enough
    // (total_pixels bytes, i.e. density=1's worst case) to cover every density's byte count
    // (total_pixels / density <= total_pixels for all densities >= 1). Filled with varied,
    // non-constant bytes (not all-zero / all-0xff) so every shift/mask branch of extract_index
    // sees a realistic mix of index values, same reasoning as candidate 042's non-trivial
    // palette values.
    let mut small_input = vec![0u8; SMALL_PIXELS];
    for (i, b) in small_input.iter_mut().enumerate() {
        *b = (i as u32).wrapping_mul(2654435761) as u8;
    }
    let small_input = black_box(small_input);
    let mut large_input = vec![0u8; LARGE_PIXELS];
    for (i, b) in large_input.iter_mut().enumerate() {
        *b = (i as u32).wrapping_mul(2654435761) as u8;
    }
    let large_input = black_box(large_input);

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
            let input: &[u8] = match cfg.size_name {
                "small" => &small_input,
                "large" => &large_input,
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
                Variant::UnpackNatural => unsafe {
                    fill_unpack_natural(ptr, total_pixels, density, input, &palette)
                },
                Variant::UnpackVolatile => unsafe {
                    fill_unpack_volatile(ptr, total_pixels, density, input, &palette)
                },
                Variant::UnpackVolatileSpecialized => unsafe {
                    match density {
                        1 => fill_unpack_volatile_specialized::<1>(ptr, total_pixels, input, &palette),
                        2 => fill_unpack_volatile_specialized::<2>(ptr, total_pixels, input, &palette),
                        4 => fill_unpack_volatile_specialized::<4>(ptr, total_pixels, input, &palette),
                        8 => fill_unpack_volatile_specialized::<8>(ptr, total_pixels, input, &palette),
                        _ => unreachable!(),
                    }
                },
                Variant::LookupVolatileSpecialized => unsafe {
                    match density {
                        1 => fill_lookup_volatile_specialized::<1>(ptr, total_pixels, &palette),
                        2 => fill_lookup_volatile_specialized::<2>(ptr, total_pixels, &palette),
                        4 => fill_lookup_volatile_specialized::<4>(ptr, total_pixels, &palette),
                        8 => fill_lookup_volatile_specialized::<8>(ptr, total_pixels, &palette),
                        _ => unreachable!(),
                    }
                },
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
