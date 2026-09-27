//! Pure measurement candidate: what does CRC32 verification cost on the decode path?
//! `DecodeOptions::ignore_crc` already exists, so this is a clean A/B with no patch needed.
//!
//! Decodes every *.png in a directory (default: the repo's own tests/benches/) with
//! ignore_crc=false (default) and ignore_crc=true, alternating per iteration, and reports paired
//! timing plus a correctness check that decoded pixels are byte-identical either way (CRC only
//! validates, it must never affect the decoded output).

use std::hint::black_box;
use std::io::Cursor;
use std::time::Instant;
use std::{fs, path::PathBuf};

fn decode_full(bytes: &[u8], ignore_crc: bool) -> (Vec<u8>, usize, usize) {
    let mut options = png::DecodeOptions::default();
    options.set_ignore_crc(ignore_crc);
    let decoder = png::Decoder::new_with_options(Cursor::new(bytes), options);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut buf).unwrap();
    buf.truncate(info.buffer_size());
    (buf, info.width as usize, info.height as usize)
}

fn bench_file(path: &PathBuf, iters: usize) {
    let bytes = fs::read(path).unwrap();
    let name = path.file_name().unwrap().to_string_lossy().to_string();

    // Correctness: CRC checking must never change decoded pixels.
    let (baseline_pixels, w, h) = decode_full(&bytes, false);
    let (nocrc_pixels, _, _) = decode_full(&bytes, true);
    if baseline_pixels != nocrc_pixels {
        println!("RESULT file={name} verify=FAIL(pixels differ with ignore_crc)");
        return;
    }

    let warmup = iters / 10;
    let mut crc_on = Vec::with_capacity(iters);
    let mut crc_off = Vec::with_capacity(iters);

    for i in 0..iters {
        // Alternate order each iteration to cancel drift (paired-comparison discipline).
        let (t_on, t_off) = if i % 2 == 0 {
            let a = time_one(&bytes, false);
            let b = time_one(&bytes, true);
            (a, b)
        } else {
            let b = time_one(&bytes, true);
            let a = time_one(&bytes, false);
            (a, b)
        };
        if i >= warmup {
            crc_on.push(t_on);
            crc_off.push(t_off);
        }
    }

    crc_on.sort();
    crc_off.sort();
    let n = crc_on.len();
    let p50_on = crc_on[n / 2].as_secs_f64() * 1e3;
    let p50_off = crc_off[n / 2].as_secs_f64() * 1e3;
    let delta = (p50_off / p50_on - 1.0) * 100.0;
    println!(
        "RESULT file={name} img={w}x{h} n={n} crc_on_p50={p50_on:.4}ms crc_off_p50={p50_off:.4}ms delta={delta:+.1}% verify=OK"
    );
}

fn time_one(bytes: &[u8], ignore_crc: bool) -> std::time::Duration {
    let start = Instant::now();
    let (buf, _, _) = decode_full(black_box(bytes), ignore_crc);
    black_box(&buf);
    start.elapsed()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);
    let dir: PathBuf = args
        .iter()
        .position(|a| a == "--corpus")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../../../tests/benches"));

    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "png").unwrap_or(false))
        .collect();
    files.sort();

    for f in files {
        bench_file(&f, iters);
    }
}
