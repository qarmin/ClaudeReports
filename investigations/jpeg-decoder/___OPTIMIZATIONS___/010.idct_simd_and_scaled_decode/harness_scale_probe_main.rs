// Standalone timing harness for jpeg-decoder, shared by every candidate in this workspace
// (see ___OPTIMIZATIONS___/README.md). Not part of the jpeg-decoder crate itself.
//
// Usage:
//   bench_decode <input_file> [iters=30] [expected_hash_hex] [--info]
//
// Decodes the file `iters` times (or just reads header info with --info), hashes the output
// with FNV-1a, and aborts if any iteration's hash differs from iteration 0 (or from
// expected_hash_hex, if given). Prints min/p50/mean/p95/max over the post-warmup iterations
// plus the raw per-iteration times, so paired.sh/matrix.sh can parse a single number out of it.

use std::env;
use std::fs;
use std::io::Cursor;
use std::process;
use std::time::Instant;

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x00000100000001B3);
    }
    h
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let info_mode = if let Some(pos) = args.iter().position(|a| a == "--info") {
        args.remove(pos);
        true
    } else {
        false
    };
    // CANDIDATE010 scratch addition: --scale W H requests reduced-size (thumbnail) decoding
    // via the public Decoder::scale API, to measure src/idct.rs's scalar-only 4x4/2x2/1x1 paths.
    let scale_wh: Option<(u16, u16)> = if let Some(pos) = args.iter().position(|a| a == "--scale") {
        args.remove(pos);
        let w: u16 = args.remove(pos).parse().expect("--scale width must be u16");
        let h: u16 = args.remove(pos).parse().expect("--scale height must be u16");
        Some((w, h))
    } else {
        None
    };

    let mut args = args.into_iter();
    let path = args.next().unwrap_or_else(|| {
        eprintln!("usage: bench_decode <file> [iters=30] [expected_hash_hex] [--info] [--scale W H]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let expected_hash: Option<u64> = args.next().and_then(|s| u64::from_str_radix(&s, 16).ok());

    let bytes = fs::read(&path).expect("read failed");

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let t0 = Instant::now();
        let h = if info_mode {
            let mut dec = jpeg_decoder::Decoder::new(Cursor::new(std::hint::black_box(&bytes)));
            dec.read_info().expect("read_info failed");
            let info = dec.info().expect("no info");
            fnv1a64(&format!("{:?}", info).into_bytes())
        } else {
            let mut dec = jpeg_decoder::Decoder::new(Cursor::new(std::hint::black_box(&bytes)));
            if let Some((w, h)) = scale_wh {
                dec.scale(w, h).expect("scale failed");
            }
            let data = dec.decode().expect("decode failed");
            fnv1a64(std::hint::black_box(&data))
        };
        let us = t0.elapsed().as_micros() as u64;
        if i == iters - 1 {
            eprintln!(
                "CANDIDATE010_IDCT_NANOS_CUMULATIVE={} CANDIDATE010_IDCT_CALLS_CUMULATIVE={}",
                jpeg_decoder::_candidate010_idct_nanos(),
                jpeg_decoder::_candidate010_idct_calls()
            );
        }

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
    println!("mode       = {}", if info_mode { "info" } else { "decode" });
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
