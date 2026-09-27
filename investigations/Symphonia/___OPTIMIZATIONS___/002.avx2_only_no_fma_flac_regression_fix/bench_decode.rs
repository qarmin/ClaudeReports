// Benchmark harness for perf-optimize sessions. Decodes a file N times, verifying a
// hash of the decoded PCM data matches every iteration (bit-exact correctness check),
// and reports min/p50/mean/p95/max wall-clock time per decode.
//
// Usage: bench_decode <path> <iterations> [expected_hash_hex]
//
// If expected_hash_hex is omitted, the hash from the first iteration is printed and used
// as the reference for the remaining iterations (still catches non-determinism within a run).

use std::env;
use std::fs::File;
use std::hash::Hasher;
use std::hint::black_box;
use std::path::Path;
use std::time::{Duration, Instant};

use symphonia::core::audio::sample::Sample;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::TrackType;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

fn decode_once(path: &Path) -> (u64, u64) {
    let file = Box::new(File::open(path).unwrap());
    let mss = MediaSourceStream::new(file, Default::default());
    let hint = Hint::new();

    let fmt_opts: FormatOptions = Default::default();
    let meta_opts: MetadataOptions = Default::default();
    let dec_opts: AudioDecoderOptions = Default::default();

    let mut format =
        symphonia::default::get_probe().probe(&hint, mss, fmt_opts, meta_opts).unwrap();

    let track = format.default_track(TrackType::Audio).unwrap();
    let track_id = track.id;

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(track.codec_params.as_ref().unwrap().audio().unwrap(), &dec_opts)
        .unwrap();

    let mut samples: Vec<f32> = Default::default();
    let mut total_sample_count: u64 = 0;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    while let Some(packet) = format.next_packet().unwrap() {
        if packet.track_id != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                samples.resize(audio_buf.samples_interleaved(), f32::MID);
                audio_buf.copy_to_slice_interleaved(&mut samples);
                total_sample_count += samples.len() as u64;
                for s in black_box(&samples) {
                    hasher.write_u32(s.to_bits());
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    (total_sample_count, hasher.finish())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: bench_decode <path> <iterations> [expected_hash_hex]");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let iterations: usize = args[2].parse().unwrap();
    let expected_hash: Option<u64> =
        args.get(3).map(|h| u64::from_str_radix(h, 16).expect("invalid hash hex"));

    let mut durations: Vec<Duration> = Vec::with_capacity(iterations);
    let mut reference_hash: Option<u64> = expected_hash;
    let mut total_samples: u64 = 0;

    for i in 0..iterations {
        let start = Instant::now();
        let (n_samples, hash) = decode_once(path);
        let elapsed = start.elapsed();
        durations.push(elapsed);
        total_samples = n_samples;

        match reference_hash {
            None => reference_hash = Some(hash),
            Some(expected) => {
                if hash != expected {
                    eprintln!(
                        "MISMATCH at iteration {i}: expected {expected:016x}, got {hash:016x}"
                    );
                    std::process::exit(2);
                }
            }
        }
    }

    durations.sort();
    let n = durations.len();
    let min = durations[0];
    let p50 = durations[n / 2];
    let p95_idx = ((n * 95) / 100).min(n - 1).max(0);
    let p95 = durations[p95_idx];
    let max = durations[n - 1];
    let sum: Duration = durations.iter().sum();
    let mean = sum / n as u32;

    println!("samples_per_decode={total_samples}");
    println!("hash={:016x}", reference_hash.unwrap());
    println!(
        "min={:?} p50={:?} mean={:?} p95={:?} max={:?}",
        min, p50, mean, p95, max
    );
}
