// Isolated bench for candidate 043: pre-sizing the output `Vec` passed to
// `flate2::read::ZlibDecoder::read_to_end` when decoding a PDF FlateDecode stream.
//
// This does not link against hayro-syntax (its `flate::decode` is `pub(crate)`, not reachable
// from an external bench crate) - instead it reproduces the exact two functions under test
// (`zlib_stream`/baseline vs `zlib_stream`/patched) against the same pinned flate2/zlib-rs
// version hayro-syntax uses, and decodes real FlateDecode-compressed stream payloads extracted
// from this workspace's own test PDFs (../testdata/*.zlib - from img.pdf and issue391.pdf).
//
// Usage: flate_decode_bench <baseline|patched> [iters=50]

use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
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

fn zlib_stream_baseline(data: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).ok().map(|_| result)
}

fn zlib_stream_patched(data: &[u8]) -> Option<Vec<u8>> {
    const OUTPUT_SIZE_GUESS_FACTOR: usize = 3;
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::with_capacity(data.len() * OUTPUT_SIZE_GUESS_FACTOR);
    decoder.read_to_end(&mut result).ok().map(|_| result)
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| {
        eprintln!("usage: flate_decode_bench <baseline|patched> [iters=50]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(50);

    let testdata_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../testdata");
    let mut streams: Vec<Vec<u8>> = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(&testdata_dir)
        .expect("read testdata dir failed")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("zlib"))
        .collect();
    entries.sort();
    for path in &entries {
        streams.push(fs::read(path).expect("read stream failed"));
    }
    assert!(!streams.is_empty(), "no .zlib testdata files found");

    let decode_fn: fn(&[u8]) -> Option<Vec<u8>> = match mode.as_str() {
        "baseline" => zlib_stream_baseline,
        "patched" => zlib_stream_patched,
        other => {
            eprintln!("unknown mode: {other} (expected baseline|patched)");
            process::exit(1);
        }
    };

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut first_hash: Option<u64> = None;

    for i in 0..iters {
        let t0 = Instant::now();
        let mut combined_hash: u64 = 0xcbf29ce484222325;
        for stream in &streams {
            let decoded = std::hint::black_box(decode_fn(std::hint::black_box(stream)))
                .expect("decode failed");
            // Fold each stream's hash into a running combined hash (order-stable, cheap).
            let h = fnv1a64(&decoded);
            combined_hash ^= h;
            combined_hash = combined_hash.wrapping_mul(0x00000100000001B3);
        }
        let us = t0.elapsed().as_micros() as u64;

        match first_hash {
            None => first_hash = Some(combined_hash),
            Some(prev) => assert_eq!(
                combined_hash, prev,
                "iter {i} hash {combined_hash:016x} differs from iter 0 {prev:016x}"
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

    println!("mode       = {mode}");
    println!("streams    = {} files", streams.len());
    println!("iters      = {iters} (warmup discarded = {warmup})");
    println!("hash       = {hash:016x}  (STABLE across all iters)");
    println!("min        = {:8.3} ms", min as f64 / 1000.0);
    println!("p50        = {:8.3} ms", p50 as f64 / 1000.0);
    println!("mean       = {:8.3} ms", mean as f64 / 1000.0);
    println!("p95        = {:8.3} ms", p95 as f64 / 1000.0);
    println!("max        = {:8.3} ms", max as f64 / 1000.0);
}
