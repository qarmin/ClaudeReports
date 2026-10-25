// Standalone bench for CompositeMethod::Over/Multiply/Screen in zune-imageprocs, reaching
// crates/zune-imageprocs/src/composite.rs::composite_region (no existing project bench/example
// reached this code path). See ___OPTIMIZATIONS___/004.COMPOSITE_FLOAT_ROUNDTRIP/RESULT.md.
//
// Usage:
//   bench_composite <width> <height> <iters> <method: over|multiply|screen>
//
// Builds two deterministic-but-varied RGBA (u8) images once, then repeatedly composites src onto
// a fresh clone of dst (composite_two_images mutates dst in place and the op is destructive on
// the image stack), hashing the resulting pixel buffer every iteration to confirm bit-stability.

use std::env;
use std::process;
use std::time::Instant;

use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::traits::OperationsTrait;
use zune_imageprocs::composite::{Composite, CompositeMethod};

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

fn make_rgba(width: usize, height: usize, seed: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(width * height * 4);
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    for _ in 0..(width * height) {
        // cheap xorshift-ish PRNG, deterministic across runs
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        let r = (state & 0xff) as u8;
        let g = ((state >> 8) & 0xff) as u8;
        let b = ((state >> 16) & 0xff) as u8;
        let a = ((state >> 24) & 0xff) as u8;
        v.push(r);
        v.push(g);
        v.push(b);
        v.push(a);
    }
    v
}

unsafe fn image_bytes(img: &Image) -> Vec<u8> {
    let mut out = Vec::new();
    for ch in img.channels_ref(false) {
        out.extend_from_slice(ch.alias());
    }
    out
}

fn main() {
    let mut args = env::args().skip(1);
    let width: usize = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("usage: bench_composite <width> <height> <iters> <method: over|multiply|screen>");
            process::exit(1);
        });
    let height: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(1080);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let method_str = args.next().unwrap_or_else(|| "over".to_string());
    let method = match method_str.as_str() {
        "over" => CompositeMethod::Over,
        "multiply" => CompositeMethod::Multiply,
        "screen" => CompositeMethod::Screen,
        other => {
            eprintln!("unknown method {other}, expected over|multiply|screen");
            process::exit(1);
        }
    };

    let dst_bytes = make_rgba(width, height, 0xDEAD_BEEF);
    let src_bytes = make_rgba(width, height, 0xC0FF_EE11);

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let dst = Image::from_u8(&dst_bytes, width, height, ColorSpace::RGBA);
        let src = Image::from_u8(&src_bytes, width, height, ColorSpace::RGBA);
        let mut images = vec![dst, src];

        let t0 = Instant::now();
        Composite::new(method, (0, 0))
            .execute_multiple(&mut images)
            .expect("composite failed");
        let us = t0.elapsed().as_micros() as u64;

        let bytes = unsafe { image_bytes(&images[0]) };
        let h = fnv1a64(std::hint::black_box(&bytes));
        match first_hash {
            None => first_hash = Some(h),
            Some(prev) => assert_eq!(h, prev, "iter {i} hash {h:016x} differs from iter 0 {prev:016x}"),
        }
        times_us.push(us);
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

    println!("method     = {method_str}");
    println!("dims       = {width}x{height}");
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
