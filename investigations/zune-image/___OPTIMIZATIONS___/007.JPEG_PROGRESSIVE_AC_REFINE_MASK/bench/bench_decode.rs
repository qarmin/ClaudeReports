// Standalone bench driver for ___OPTIMIZATIONS___ candidate 002 (JPEG Huffman/MCU entropy decode).
//
// Usage:
//   bench_decode <input_file> [iters=30] [expected_hash_hex]
//
// If expected_hash_hex is given, iteration 0 must produce that hash or the run aborts.
// Iteration N>0 must always produce the same hash as iteration 0, otherwise the run aborts.
//
// This is a harness file, not part of the source fix - it is copied into the cloned tree by
// this candidate's own justfile (see `clone:` there), not included in patch.diff, so it's
// present identically for both the baseline and patched builds.

use std::env;
use std::fs;
use std::hint::black_box;
use std::process;
use std::time::Instant;

use zune_core::bytestream::ZCursor;
use zune_jpeg::JpegDecoder;

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x00000100000001B3);
    }
    h
}

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("usage: bench_decode <file> [iters=30] [expected_hash_hex]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let expected_hash: Option<u64> = args.next().and_then(|s| u64::from_str_radix(&s, 16).ok());

    let bytes = fs::read(&path).expect("read failed");

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let t0 = Instant::now();
        let mut decoder = JpegDecoder::new(ZCursor::new(&bytes));
        let data = decoder.decode().expect("decode failed");
        let data = black_box(data);
        let us = t0.elapsed().as_micros() as u64;

        let h = fnv1a64(&data);
        match first_hash {
            None => {
                first_hash = Some(h);
                if let Some(exp) = expected_hash {
                    assert_eq!(h, exp, "iter {} hash {:016x} != expected {:016x}", i, h, exp);
                }
            }
            Some(prev) => assert_eq!(
                h, prev,
                "iter {} hash {:016x} differs from iter 0 {:016x}",
                i, h, prev
            ),
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

    println!("file       = {}", path);
    println!("iters      = {} (warmup discarded = {})", iters, warmup);
    println!("hash       = {:016x}  (STABLE across all iters)", hash);
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
        print!("{}", t);
    }
    println!("]");
}
