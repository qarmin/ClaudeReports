//! Self-contained JPEG2000 decode benchmark - no arguments, no external
//! files to hunt for. Embeds a real 1241x1752 JP2 codestream extracted from
//! a real-world PDF (`hayro-tests/pdfs/load/issue391.pdf`, its largest
//! embedded JPX image).
//!
//! Run with: `cargo run --release -- [iterations]`
//!
//! This is the tool used to measure the IDWT horizontal filter (see
//! `../RESULT.md`). Correctness is checked via a hash of the decoded
//! output every iteration, so a candidate patch that silently changes
//! output aborts the run instead of reporting a fake speedup.
//!
//! To measure a candidate patch: apply it to the real
//! `../../../../hayro-jpeg2000` checkout, then run this again (it depends
//! on that crate via a relative path).

use hayro_jpeg2000::{DecodeSettings, DecoderContext, Image};
use std::time::{Duration, Instant};

const SAMPLE_JP2: &[u8] = include_bytes!("../testdata/sample.jp2");

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let settings = DecodeSettings::default();
    let image = Image::new(SAMPLE_JP2, &settings).expect("failed to parse sample.jp2");
    let channels = image.color_space().num_channels() as usize + usize::from(image.has_alpha());
    let output_len = image.width() as usize * image.height() as usize * channels;

    println!(
        "sample.jp2: {} bytes, {}x{}, {channels} channels, {output_len} output bytes, {iterations} iterations",
        SAMPLE_JP2.len(),
        image.width(),
        image.height(),
    );

    let mut output = vec![0u8; output_len];
    let mut context = DecoderContext::default();
    let mut durations = Vec::with_capacity(iterations);
    let mut expected_hash = None;

    for i in 0..iterations {
        let start = Instant::now();
        let decoded = image.decode(&mut context).expect("decode failed");
        decoded.store_u8_into(&mut output);
        let elapsed = start.elapsed();

        let hash = fnv1a(&output);
        match expected_hash {
            None => expected_hash = Some(hash),
            Some(expected) => assert_eq!(
                hash, expected,
                "iteration {i} produced different output than iteration 0 - a candidate patch changed decode output"
            ),
        }

        durations.push(elapsed);
    }

    let warmup = (iterations / 10).max(1);
    let mut stable: Vec<Duration> = durations[warmup..].to_vec();
    stable.sort();
    let n = stable.len();
    let min = stable[0];
    let p50 = stable[n / 2];
    let mean: Duration = stable.iter().sum::<Duration>() / n as u32;
    let max = stable[n - 1];

    println!(
        "min={min:>8.3?} p50={p50:>8.3?} mean={mean:>8.3?} max={max:>8.3?} hash={:016x}",
        expected_hash.unwrap()
    );
}
