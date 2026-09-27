// Repro for candidate 033: u32 shift-amount overflow in `fix_wave_channel_mask`
// (symphonia-format-riff/src/wave/chunks.rs).
//
// A WAV file whose `fmt ` chunk declares WAVE_FORMAT_EXTENSIBLE (0xfffe) with the PCM sub-format
// GUID is written to disk, then opened through the real public entry point an application uses:
// `symphonia::default::get_probe().probe()` over a `std::fs::File`.
//
// `read_ext_fmt` hands the raw `nChannels` and `dwChannelMask` fields straight to
// `fix_wave_channel_mask`, which does
//
//     let shift = 32 - (!channel_mask).leading_ones();
//     channel_mask |= ((1 << channel_diff) - 1) << shift;
//
// in u32. Two independent shift amounts can reach/exceed 32:
//   (a) `shift == 32` whenever bit 31 of dwChannelMask is set (leading_ones(!mask) == 0) and the
//       mask still has fewer ones than nChannels;
//   (b) `channel_diff >= 32` whenever nChannels - popcount(mask) >= 32.
//
// Expected on an unpatched tree (overflow-checks on): "attempt to shift left with overflow" panic
// inside symphonia-format-riff/src/wave/chunks.rs.
// Expected on a patched tree: every case is either probed successfully or rejected with
// Err("wav: too many channels in mask for fmt_ext"), no panic.

use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use symphonia_core::formats::probe::Hint;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_core::meta::MetadataOptions;

/// KSDATAFORMAT_SUBTYPE_PCM, as matched by `read_ext_fmt`.
#[rustfmt::skip]
const KSDATAFORMAT_SUBTYPE_PCM: [u8; 16] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
    0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
];

/// Build a minimal WAVE_FORMAT_EXTENSIBLE file: a 40-byte `fmt ` chunk plus an 8-byte `data`
/// chunk, i.e. ~60 bytes total.
fn build_wav(num_channels: u16, channel_mask: u32) -> Vec<u8> {
    const BITS_PER_SAMPLE: u16 = 16;

    let mut fmt = Vec::new();
    fmt.extend_from_slice(&0xfffe_u16.to_le_bytes()); // wFormatTag = WAVE_FORMAT_EXTENSIBLE
    fmt.extend_from_slice(&num_channels.to_le_bytes()); // nChannels
    fmt.extend_from_slice(&44100_u32.to_le_bytes()); // nSamplesPerSec
    fmt.extend_from_slice(&176400_u32.to_le_bytes()); // nAvgBytesPerSec
    fmt.extend_from_slice(&4_u16.to_le_bytes()); // nBlockAlign
    fmt.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes()); // wBitsPerSample
    fmt.extend_from_slice(&22_u16.to_le_bytes()); // cbSize (must be exactly 22)
    fmt.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes()); // wValidBitsPerSample
    fmt.extend_from_slice(&channel_mask.to_le_bytes()); // dwChannelMask
    fmt.extend_from_slice(&KSDATAFORMAT_SUBTYPE_PCM); // SubFormat
    assert_eq!(fmt.len(), 40);

    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&(fmt.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&8_u32.to_le_bytes());
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
        .unwrap_or_else(|| std::env::temp_dir().join("symphonia_033_repro"));
    fs::create_dir_all(&dir).expect("mkdir");

    // (nChannels, dwChannelMask, overflow expected?, note)
    let cases: [(u16, u32, bool, &str); 8] = [
        (2, 0b11, false, "well-formed stereo mask, nothing to fix"),
        (2, 0, false, "mask empty, channel_diff=2 -> shift 0, fits"),
        (31, 0, false, "channel_diff=31 -> 1u32<<31, largest that fits"),
        (2, 1 << 18, false, "shift=19, channel_diff=1, fits"),
        (32, 0, true, "channel_diff=32 -> 1u32<<32"),
        (40, 0, true, "channel_diff=40 -> 1u32<<40"),
        (2, 0x8000_0000, true, "bit 31 set, channel_diff=1 -> x<<32"),
        (3, 0xc000_0000, true, "bits 30-31 set, channel_diff=1 -> x<<32"),
    ];

    let mut panics = 0;

    for (channels, mask, overflow_expected, note) in cases {
        let path = dir.join(format!("ch{channels}_mask{mask:08x}.wav"));
        fs::write(&path, build_wav(channels, mask)).expect("write wav");

        let label = format!(
            "nChannels={channels:<3} dwChannelMask=0x{mask:08x} (overflow expected: \
             {overflow_expected:<5}) [{note}]"
        );

        let result = panic::catch_unwind(AssertUnwindSafe(|| open_with_probe(&path)));

        match result {
            Ok(Ok(())) => println!("{label} -> probe Ok"),
            Ok(Err(e)) => println!("{label} -> probe Err: {e}"),
            Err(_) => {
                panics += 1;
                println!("{label} -> PANICKED (see stderr above)");
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
