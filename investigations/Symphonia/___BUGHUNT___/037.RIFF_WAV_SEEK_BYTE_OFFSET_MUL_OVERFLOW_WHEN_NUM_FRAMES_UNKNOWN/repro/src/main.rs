//! Standalone reproduction for: `WavReader::seek` (symphonia-format-riff/src/wave/mod.rs:295-297)
//! computes the target byte offset as
//!
//! ```text
//! let seek_pos =
//!     self.data_start_pos + (actual_ts.get() as u64 * self.packet_info.block_size.get());
//! ```
//!
//! in plain, unchecked `u64` arithmetic. The only upper bound on the requested timestamp is
//! `if let Some(num_frames) = track.num_frames` (mod.rs:279-283) - and `num_frames` stays `None`
//! for any WAVE stream whose `data` chunk length is `u32::MAX`, because `DataChunk::parse`
//! (wave/chunks.rs:611-617) deliberately maps that to `len: None` ("the file is streaming, the
//! length is not known"), which means `append_data_params` (mod.rs:163-165) - the only thing that
//! would set `num_frames` in the absence of a `fact` chunk - never runs. With no bound at all,
//! both halves of the expression can leave `u64` range:
//!
//!   - the multiply, for any `block_size >= 3` (`ffmpeg_stream.wav`, 16-bit stereo, block_size 4);
//!   - the add, for `block_size == 2` when the data chunk starts more than 1023 bytes into the
//!     file (`ffmpeg_stream_mono_big_meta.wav`, 16-bit mono with a large LIST/INFO chunk ahead of
//!     the data chunk, block_size 2, data_start_pos 1488).
//!
//! The input files are NOT hand-authored: they are produced by really running `ffmpeg`, writing
//! WAVE to a pipe (`ffmpeg ... -f wav pipe:1 > file.wav`). ffmpeg cannot know the length ahead of
//! time when it streams, so it writes `0xFFFFFFFF` for both the RIFF and the `data` chunk lengths -
//! the exact convention symphonia's own comments call out. If `ffmpeg` is on PATH the repro
//! re-generates both files at run time so nothing is taken on trust; otherwise it falls back to
//! the shipped `assets/*.wav`, which are byte-for-byte what those commands produced.
//!
//! The files are then opened through the ordinary public entry point an application uses -
//! `symphonia::default::get_probe().probe(..)` - and seeked through the public
//! `FormatReader::seek` API with `SeekTo::Timestamp` / `SeekTo::Time`.

use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use symphonia_core::formats::probe::Hint;
use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo, TrackType};
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_core::meta::MetadataOptions;
use symphonia_core::units::{Time, Timestamp};

/// A long title, so ffmpeg emits a LIST/INFO chunk big enough to push the data chunk more than
/// 1023 bytes into the file (that is what makes the *addition* overflow reachable at block_size 2).
fn long_title() -> String {
    "T".repeat(1400)
}

/// Re-generate a streaming WAVE with a real ffmpeg run, if ffmpeg is available.
fn generate_with_ffmpeg(out: &Path, channels: &str, title: Option<String>) -> bool {
    let file = match fs::File::create(out) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-v",
        "error",
        "-f",
        "lavfi",
        "-i",
        "sine=frequency=440:duration=0.2:sample_rate=48000",
        "-ac",
        channels,
        "-c:a",
        "pcm_s16le",
    ]);

    if let Some(title) = title {
        cmd.arg("-metadata").arg(format!("title={title}"));
    }

    let status = cmd
        .args(["-f", "wav", "pipe:1"])
        .stdout(Stdio::from(file))
        .stderr(Stdio::inherit())
        .status();

    matches!(status, Ok(s) if s.success())
}

fn open(path: &Path) -> Box<dyn FormatReader> {
    let file = fs::File::open(path).unwrap_or_else(|e| panic!("failed to open {path:?}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    symphonia::default::get_probe()
        .probe(&hint, mss, FormatOptions::default(), MetadataOptions::default())
        .expect("probe failed")
}

/// Open the file, seek, and (if the seek succeeded) read the packet that follows, so a seek that
/// silently landed in the wrong place is visible too. Returns `Err(())` if `seek` panicked.
fn try_seek(path: &Path, label: &str, to: SeekTo) -> Result<(), ()> {
    let mut reader = open(path);

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let seeked = reader.seek(SeekMode::Coarse, to).map_err(|e| e.to_string())?;
        let pts = match reader.next_packet() {
            Ok(Some(packet)) => format!("{}", packet.pts),
            Ok(None) => "end of stream".to_string(),
            Err(e) => format!("error: {e}"),
        };
        Ok::<_, String>((seeked, pts))
    }));

    match &result {
        Ok(Ok((seeked, pts))) => println!(
            "    {label} -> Ok(required_ts={}, actual_ts={}), next packet pts={}",
            seeked.required_ts, seeked.actual_ts, pts
        ),
        Ok(Err(e)) => println!("    {label} -> Err({e})"),
        Err(_) => println!("    {label} -> PANICKED (see stderr above)"),
    }

    result.map(|_| ()).map_err(|_| ())
}

fn report_file(path: &Path) {
    let bytes = fs::read(path).expect("read asset");
    let riff_len = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    let data_hdr = bytes.windows(4).position(|w| w == b"data").expect("no data chunk");
    let data_len = u32::from_le_bytes(bytes[data_hdr + 4..data_hdr + 8].try_into().unwrap());

    println!(
        "  size={} B, RIFF length field=0x{riff_len:08X}, data length field=0x{data_len:08X}, \
         data_start_pos={}",
        bytes.len(),
        data_hdr + 8
    );

    let reader = open(path);
    let track = reader.default_track(TrackType::Audio).expect("no audio track");
    println!(
        "  track: num_frames={:?}, time_base={:?}",
        track.num_frames,
        track.time_base.map(|tb| (tb.numer, tb.denom))
    );
}

fn main() {
    let dir: PathBuf = std::env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| "assets".into());

    let stereo = dir.join("ffmpeg_stream.wav");
    let mono = dir.join("ffmpeg_stream_mono_big_meta.wav");

    let fresh = generate_with_ffmpeg(&stereo, "2", None)
        && generate_with_ffmpeg(&mono, "1", Some(long_title()));

    if fresh {
        println!("inputs freshly produced by real `ffmpeg ... -f wav pipe:1` runs");
    }
    else {
        println!("using the shipped assets (ffmpeg not usable here)");
    }

    let mut panics = 0;

    // 5e18 * 4 == 2e19 > u64::MAX (1.844e19): the multiply itself overflows.
    // 4_611_686_018_427_388_800 is already a multiple of 1152, so actual_ts == required_ts and
    // actual_ts * 4 == 2^64 + 3584: it panics under overflow-checks, and without them it wraps to
    // 3584, landing seek_pos a mere 3584 bytes into the data chunk instead of out of range.
    let ts_cases: [(&str, i64); 4] = [
        ("ts=1000 (ordinary)", 1_000),
        ("ts=i64::MAX", i64::MAX),
        ("ts=5_000_000_000_000_000_000", 5_000_000_000_000_000_000),
        ("ts=4_611_686_018_427_388_800", 4_611_686_018_427_388_800),
    ];

    // 1.5e14 s * 48000 = 7.2e18 (fits in i64), and * 4 = 2.88e19 > u64::MAX.
    let time_cases: [(&str, i128); 2] = [
        ("time=0.1s (ordinary)", 100_000_000),
        ("time=150_000_000_000_000s", 150_000_000_000_000 * 1_000_000_000),
    ];

    for (path, note) in [
        (&stereo, "16-bit stereo -> block_size 4 (multiply overflows)"),
        (&mono, "16-bit mono + 1.4 KB title -> block_size 2, data_start_pos 1488 (add overflows)"),
    ] {
        println!("\n=== {} - {note}", path.display());
        report_file(path);

        println!("  -- SeekTo::Timestamp --");
        for (label, ts) in ts_cases {
            let to = SeekTo::Timestamp { ts: Timestamp::from(ts), track_id: 0 };
            if try_seek(path, label, to).is_err() {
                panics += 1;
            }
        }

        println!("  -- SeekTo::Time (time_base 1/48000, so ts = seconds * 48000) --");
        for (label, nanos) in time_cases {
            let time = Time::try_from_nanos_i128(nanos).expect("time in range");
            let to = SeekTo::Time { time, track_id: Some(0) };
            if try_seek(path, label, to).is_err() {
                panics += 1;
            }
        }
    }

    println!();
    if panics > 0 {
        println!("BUG REPRODUCED: {panics} seek call(s) panicked instead of returning a seek error");
        std::process::exit(1);
    }
    println!("NO PANIC: every seek returned Ok/Err without panicking");
}
