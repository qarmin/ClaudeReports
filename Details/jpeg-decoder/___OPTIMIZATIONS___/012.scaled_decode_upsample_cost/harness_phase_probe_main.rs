// CANDIDATE012 scratch harness: --scale W H (Decoder::scale) + phase-timing breakdown
// (entropy decode / dequant+idct dispatch / upsample+color-convert), via the CANDIDATE012_*
// atomics exposed by src/lib.rs. Not part of the canonical harness - kept local per candidate
// 010's precedent for the same kind of one-off --scale probe.

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
        eprintln!("usage: bench_decode <file> [iters=30] [expected_hash_hex] [--scale W H]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let expected_hash: Option<u64> = args.next().and_then(|s| u64::from_str_radix(&s, 16).ok());

    let bytes = fs::read(&path).expect("read failed");

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let t0 = Instant::now();
        let mut dec = jpeg_decoder::Decoder::new(Cursor::new(std::hint::black_box(&bytes)));
        if let Some((w, h)) = scale_wh {
            dec.scale(w, h).expect("scale failed");
        }
        let data = dec.decode().expect("decode failed");
        let h = fnv1a64(std::hint::black_box(&data));
        let us = t0.elapsed().as_micros() as u64;

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

    let n_stable_iters = (iters - warmup) as u64;
    let entropy_ns = jpeg_decoder::_candidate012_entropy_nanos();
    let idct_ns = jpeg_decoder::_candidate012_idct_dispatch_nanos();
    let upsample_ns = jpeg_decoder::_candidate012_upsample_colorconvert_nanos();

    println!("file       = {}", path);
    println!("scale      = {:?}", scale_wh);
    println!("iters      = {} (warmup discarded = {})", iters, warmup);
    println!("hash       = {:016x}  (STABLE across all iters)", hash);
    println!("min        = {:8.3} ms", min as f64 / 1000.0);
    println!("p50        = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean       = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95        = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max        = {:8.3} ms", max as f64 / 1000.0);
    println!();
    println!(
        "phase totals over ALL {} iters (incl. warmup) - divide by {} for per-iter avg:",
        iters, iters
    );
    println!(
        "  entropy_decode_ms_total   = {:10.3}  ({:6.3} ms/iter avg)",
        entropy_ns as f64 / 1_000_000.0,
        entropy_ns as f64 / 1_000_000.0 / iters as f64
    );
    println!(
        "  idct_dispatch_ms_total    = {:10.3}  ({:6.3} ms/iter avg)",
        idct_ns as f64 / 1_000_000.0,
        idct_ns as f64 / 1_000_000.0 / iters as f64
    );
    println!(
        "  upsample_colorconv_ms_tot = {:10.3}  ({:6.3} ms/iter avg)",
        upsample_ns as f64 / 1_000_000.0,
        upsample_ns as f64 / 1_000_000.0 / iters as f64
    );
    let _ = n_stable_iters;
}
