// Bench for candidate 013 (UnfilteringBuffer: pre-reserve prev row / issue #661).
//
// Generates synthetic RGBA8 PNGs at a fixed width and varying heights, with a single row
// filter forced for the whole image (Paeth/Up/Avg are the ones whose "no previous row" case is
// specially cheapened in src/filter/mod.rs::unfilter). Decodes each repeatedly via
// `Reader::next_frame`, verifying the decoded pixels hash identically every iteration, and
// reports wall-clock stats. A `multi` mode spawns several threads each looping the same decode
// to get a realistic multi-core number.
//
// Usage:
//   bench_013 single <iters> [heights...]
//   bench_013 multi <threads> <seconds> <height>

use png::{BitDepth, ColorType, Encoder, Filter};
use std::io::Cursor;
use std::time::{Duration, Instant};

fn width() -> u32 {
    std::env::var("BENCH_WIDTH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

fn make_png(width: u32, height: u32, filter: Filter) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut encoder = Encoder::new(Cursor::new(&mut buf), width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        // NOTE: `set_compression` internally overwrites the filter (see its own doc comment),
        // so it must be called BEFORE `set_filter` for the latter to stick.
        encoder.set_compression(png::Compression::Fastest);
        encoder.set_filter(filter);
        let mut writer = encoder.write_header().unwrap();
        let mut data = vec![0u8; (width * height * 4) as usize];
        for (i, px) in data.chunks_exact_mut(4).enumerate() {
            let x = (i as u32) % width;
            let y = (i as u32) / width;
            px[0] = (x.wrapping_mul(37).wrapping_add(y.wrapping_mul(11))) as u8;
            px[1] = (x ^ y) as u8;
            px[2] = x.wrapping_add(y) as u8;
            px[3] = 255;
        }
        writer.write_image_data(&data).unwrap();
    }
    buf
}

fn decode_once(bytes: &[u8], out: &mut [u8]) -> u64 {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().unwrap();
    reader.next_frame(out).unwrap();
    fnv1a64(out)
}

fn stats(times_us: &[u64]) -> (u64, u64, u64, u64, u64) {
    let warmup = (times_us.len() / 10).max(1).min(times_us.len() - 1);
    let mut s = times_us[warmup..].to_vec();
    s.sort_unstable();
    let n = s.len();
    let min = s[0];
    let p50 = s[n / 2];
    let p95 = s[(n * 95) / 100];
    let mean = s.iter().sum::<u64>() / n as u64;
    let max = s[n - 1];
    (min, p50, mean, p95, max)
}

fn run_single(iters: usize, heights: &[u32], filter: Filter) {
    for &height in heights {
        let png_bytes = make_png(width(), height, filter);
        let buf_size = (width() * height * 4) as usize;
        let mut out = vec![0u8; buf_size];

        let mut times_us = Vec::with_capacity(iters);
        let mut first_hash = None;
        for i in 0..iters {
            let t0 = Instant::now();
            let h = std::hint::black_box(decode_once(&png_bytes, &mut out));
            let us = t0.elapsed().as_micros() as u64;
            match first_hash {
                None => first_hash = Some(h),
                Some(prev) => assert_eq!(h, prev, "hash mismatch at iter {i} height {height}"),
            }
            times_us.push(us);
        }
        let (min, p50, mean, p95, max) = stats(&times_us);
        println!(
            "RESULT filter={:?} width={} height={} iters={} hash={:016x} min_us={} p50_us={} mean_us={} p95_us={} max_us={}",
            filter, width(), height, iters, first_hash.unwrap(), min, p50, mean, p95, max
        );
    }
}

fn run_multi(threads: usize, seconds: u64, height: u32, filter: Filter) {
    let png_bytes = make_png(width(), height, filter);
    let buf_size = (width() * height * 4) as usize;
    let deadline = Instant::now() + Duration::from_secs(seconds);

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let png_bytes = png_bytes.clone();
            std::thread::spawn(move || {
                let mut out = vec![0u8; buf_size];
                let mut count: u64 = 0;
                let mut hash = 0u64;
                while Instant::now() < deadline {
                    hash = std::hint::black_box(decode_once(&png_bytes, &mut out));
                    count += 1;
                }
                (count, hash)
            })
        })
        .collect();

    let mut total = 0u64;
    let mut hash0 = None;
    for h in handles {
        let (count, hash) = h.join().unwrap();
        total += count;
        match hash0 {
            None => hash0 = Some(hash),
            Some(prev) => assert_eq!(hash, prev, "hash mismatch across threads"),
        }
    }
    println!(
        "RESULT_MULTI filter={:?} width={} height={} threads={} seconds={} total_decodes={} decodes_per_sec={:.1} hash={:016x}",
        filter, width(), height, threads, seconds, total, total as f64 / seconds as f64, hash0.unwrap()
    );
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "single".to_string());

    let filter = std::env::var("BENCH_FILTER")
        .ok()
        .and_then(|s| match s.as_str() {
            "Paeth" => Some(Filter::Paeth),
            "Up" => Some(Filter::Up),
            "Avg" => Some(Filter::Avg),
            "Sub" => Some(Filter::Sub),
            "NoFilter" => Some(Filter::NoFilter),
            _ => None,
        })
        .unwrap_or(Filter::Paeth);

    match mode.as_str() {
        "single" => {
            let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(200);
            let heights: Vec<u32> = {
                let rest: Vec<u32> = args.filter_map(|s| s.parse().ok()).collect();
                if rest.is_empty() {
                    vec![2, 4, 8, 16, 32, 128, 1024, 4096]
                } else {
                    rest
                }
            };
            run_single(iters, &heights, filter);
        }
        "multi" => {
            let threads: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(8);
            let seconds: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(3);
            let height: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(16);
            run_multi(threads, seconds, height, filter);
        }
        "row_only" => {
            // Isolates exactly the mechanism issue #661 discusses: unfilter() with `previous
            // = &[]` (baseline's cheap first-row special case) vs `previous` = a real
            // zero-filled row of the same length (what candidate 013's patch always passes).
            // No decode/decompression plumbing involved, so this has a much better
            // signal-to-noise ratio than a full end-to-end decode for a 1-2 row image.
            let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(200_000);
            let row_bytes: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(4096);
            for &bpp in &[1u8, 2, 3, 4, 6, 8] {
                for &use_real_zero_prev in &[false, true] {
                    let mut current = vec![0x5au8; row_bytes];
                    let zero_prev = vec![0u8; row_bytes];
                    let mut times_us = Vec::with_capacity(iters);
                    for _ in 0..iters {
                        let mut row = current.clone();
                        let prev: &[u8] = if use_real_zero_prev { &zero_prev } else { &[] };
                        let t0 = Instant::now();
                        png::benchable_apis::unfilter(filter, bpp, std::hint::black_box(prev), std::hint::black_box(&mut row));
                        let us = t0.elapsed().as_nanos() as u64;
                        times_us.push(us);
                        current = row;
                    }
                    let (min, p50, mean, p95, max) = stats(&times_us);
                    println!(
                        "RESULT_ROW filter={:?} bpp={} row_bytes={} real_zero_prev={} iters={} min_ns={} p50_ns={} mean_ns={} p95_ns={} max_ns={}",
                        filter, bpp, row_bytes, use_real_zero_prev, iters, min, p50, mean, p95, max
                    );
                }
            }
        }
        _ => {
            eprintln!("usage: bench_013 single|multi ...");
            std::process::exit(1);
        }
    }
}
