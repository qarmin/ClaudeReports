// Repro for candidate 027: u16 multiply overflow in WaveFormatChunk::read_pcm_fmt.
//
// A WAV file whose `fmt ` chunk declares WAVE_FORMAT_PCM with a huge nChannels is written to
// disk, then opened through the real public entry point an application uses:
// `symphonia::default::get_probe().probe()` over a `std::fs::File`. The fmt-chunk fields are
// read verbatim and multiplied in u16 (`num_channels * (bits_per_sample / 8)`) before the
// channel count is ever sanity-checked, so nChannels >= 65536 / (bits/8) overflows.
//
// Expected on an unpatched tree (overflow-checks on): "attempt to multiply with overflow" panic
// inside symphonia-format-riff/src/wave/chunks.rs.
// Expected on a patched tree: every case returns Err("riff: invalid channel count"), no panic.

use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use symphonia_core::formats::probe::Hint;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_core::meta::MetadataOptions;

fn build_wav(num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut fmt = Vec::new();
    fmt.extend_from_slice(&1u16.to_le_bytes()); // wFormatTag = WAVE_FORMAT_PCM
    fmt.extend_from_slice(&num_channels.to_le_bytes()); // nChannels
    fmt.extend_from_slice(&44100u32.to_le_bytes()); // nSamplesPerSec
    fmt.extend_from_slice(&176400u32.to_le_bytes()); // nAvgBytesPerSec
    fmt.extend_from_slice(&4u16.to_le_bytes()); // nBlockAlign
    fmt.extend_from_slice(&bits_per_sample.to_le_bytes()); // wBitsPerSample
    assert_eq!(fmt.len(), 16);

    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&(fmt.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&8u32.to_le_bytes());
    body.extend_from_slice(&[0u8; 8]);

    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

/// The real path an application takes: open the file, wrap it in a MediaSourceStream, hand it to
/// the default probe.
fn open_with_probe(path: &Path) -> symphonia_core::errors::Result<()> {
    let file = fs::File::open(path).expect("open");
    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .map(|_| ())
}

fn main() {
    let dir: PathBuf = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("symphonia_027_repro"));
    fs::create_dir_all(&dir).expect("mkdir");

    // (nChannels, wBitsPerSample, overflow expected in u16 arithmetic?)
    let cases: [(u16, u16, bool); 7] = [
        (2, 16, false),
        (26, 32, false),
        (16383, 32, false),
        (16384, 32, true),  // 16384 * 4 == 65536 > u16::MAX
        (21845, 24, false),
        (21846, 24, true),  // 21846 * 3 == 65538 > u16::MAX
        (32768, 16, true),  // 32768 * 2 == 65536 > u16::MAX
    ];

    let mut panics = 0;

    for (channels, bits, overflow_expected) in cases {
        let path = dir.join(format!("ch{}_bits{}.wav", channels, bits));
        fs::write(&path, build_wav(channels, bits)).expect("write wav");

        let label = format!(
            "nChannels={:<5} wBitsPerSample={:<2} (u16 product {:>6}, overflow expected: {})",
            channels,
            bits,
            u32::from(channels) * u32::from(bits / 8),
            overflow_expected
        );

        let result = panic::catch_unwind(AssertUnwindSafe(|| open_with_probe(&path)));

        match result {
            Ok(Ok(())) => println!("{} -> probe Ok", label),
            Ok(Err(e)) => println!("{} -> probe Err: {}", label, e),
            Err(_) => {
                panics += 1;
                println!("{} -> PANICKED (see stderr above)", label);
            }
        }
    }

    println!();
    if panics > 0 {
        println!("BUG REPRODUCED: {} of {} WAV files panicked the probe", panics, cases.len());
        std::process::exit(1);
    }
    println!("NO PANIC: all {} WAV files were rejected/accepted without panicking", cases.len());
}
