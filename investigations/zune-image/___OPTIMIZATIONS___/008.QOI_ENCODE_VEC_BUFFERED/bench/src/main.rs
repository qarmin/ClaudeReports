// Standalone bench for zune-qoi's QoiEncoder::encode, adapted from perf-optimize's
// templates/bench_template.rs.
//
// Usage:
//   qoi_encode_bench <qoi_file> [iters=30] [expected_hash_hex]
//
// Decodes the input .qoi file once (via zune-qoi's own decoder) to get raw pixels/dimensions,
// then times re-encoding those pixels back to QOI, `iters` times. Every iteration's encoded
// output is hashed (fnv1a64) and must match iteration 0's hash - this is a bit-exact comparison,
// appropriate here since the candidate patch does not intend to change encoder output at all.

use std::env;
use std::fs;
use std::hint::black_box;
use std::process;
use std::time::Instant;

use zune_qoi::zune_core::bit_depth::BitDepth;
use zune_qoi::zune_core::bytestream::ZCursor;
use zune_qoi::zune_core::colorspace::ColorSpace;
use zune_qoi::zune_core::options::EncoderOptions;

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
        eprintln!("usage: qoi_encode_bench <qoi_file> [iters=30] [expected_hash_hex]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
    let expected_hash: Option<u64> = args.next().and_then(|s| u64::from_str_radix(&s, 16).ok());

    let compressed = fs::read(&path).expect("read failed");

    // decode once, outside the timed loop, to get raw pixels + dimensions
    let mut decoder = zune_qoi::QoiDecoder::new(ZCursor::new(&compressed));
    let pixels = decoder.decode().expect("decode failed");
    let (width, height) = decoder.dimensions().unwrap();
    let channels = decoder.colorspace().unwrap().num_components();
    let colorspace = if channels == 4 {
        ColorSpace::RGBA
    } else {
        ColorSpace::RGB
    };

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let options = EncoderOptions::new(width, height, colorspace, BitDepth::Eight);
        let mut encoder = zune_qoi::QoiEncoder::new(&pixels, options);
        let mut sink: Vec<u8> = Vec::new();

        let t0 = Instant::now();
        encoder.encode(&mut sink).expect("encode failed");
        let us = t0.elapsed().as_micros() as u64;

        black_box(&sink);

        let h = fnv1a64(&sink);
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
            )
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
    println!("pixels     = {}x{} ({} channels)", width, height, channels);
    println!("iters      = {} (warmup discarded = {})", iters, warmup);
    println!("hash       = {:016x}  (STABLE across all iters)", hash);
    println!("min        = {:8.3} ms", min as f64 / 1000.0);
    println!("p50        = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean       = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95        = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max        = {:8.3} ms", max as f64 / 1000.0);
}
