#![allow(missing_docs)]
use std::process::ExitCode;

struct NullDecoder;
impl hayro_jbig2::Decoder for NullDecoder {
    fn push_pixel(&mut self, _black: bool) {}
    fn push_pixel_chunk(&mut self, _black: bool, _chunk_count: u32) {}
    fn next_line(&mut self) {}
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.jbig2>", args[0]);
        return ExitCode::FAILURE;
    }
    let data = std::fs::read(&args[1]).unwrap();
    let image = hayro_jbig2::Image::new(&data).unwrap();
    let mut ctx = hayro_jbig2::DecoderContext::default();
    let iters: u32 = std::env::var("ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Warmup, discarded.
    for _ in 0..(iters / 10).max(1) {
        let mut d = NullDecoder;
        image.decode_with(&mut d, &mut ctx).unwrap();
    }

    let mut times_us = Vec::with_capacity(iters as usize);
    for _ in 0..iters {
        let mut d = NullDecoder;
        let start = std::time::Instant::now();
        image.decode_with(&mut d, &mut ctx).unwrap();
        times_us.push(start.elapsed().as_micros() as u64);
    }
    times_us.sort_unstable();
    let p50 = times_us[times_us.len() / 2];
    println!("p50: {:.3}ms", p50 as f64 / 1000.0);
    ExitCode::SUCCESS
}
