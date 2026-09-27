//! Investigates ISOMP4_TRUN_TS_SAMPLE_DEFAULT_DUR_DIV_ZERO.
//!
//! `evil.mp4` is a real, otherwise-valid fragmented AAC-in-MP4 file produced by `ffmpeg`
//! (`-movflags frag_keyframe+empty_moov+default_base_moof`), with every `tfhd` atom's
//! `default_sample_duration` field binary-patched from its real value (1024, or 832 for the
//! last, shorter fragment) to 0. None of its `trun` atoms carry per-sample durations (`trun`
//! flags are `0x1`, data-offset-present only), so every sample's duration in every fragment is
//! supposed to come from `tfhd`'s (now zeroed) `default_sample_duration`.
//!
//! This exercises the real, public `IsoMp4Reader::try_new` and `FormatReader::seek` entry
//! points - exactly what any real caller of this crate would use - to determine whether a
//! default_sample_duration of 0 makes `TrunAtom::ts_sample`'s
//! `ts_delta.checked_div(u64::from(default_dur)).unwrap_or(0)` produce a *wrong* (but successful)
//! sample resolution when reached through the real seek path, as opposed to the caller
//! (`MoofSegment::ts_sample` in stream.rs) never selecting such a zero-duration run in the first
//! place (in which case seeking anywhere past sample 0 of the file fails with an honest
//! `SeekErrorKind::OutOfRange`/`DecodeError`, rather than resolving to a wrong sample silently).
use std::fs::File;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_core::units::Time;
use symphonia_format_isomp4::IsoMp4Reader;

fn try_seek_all(path: &str, times_secs: &[f64]) {
    println!("=== {path} ===");
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
        .unwrap_or_else(|e| panic!("failed to construct IsoMp4Reader from {path}: {e}"));

    let track_id = reader.tracks()[0].id;
    println!("opened {path} successfully, track_id={track_id}");

    for &t in times_secs {
        // Re-open for every seek attempt: IsoMp4Reader's segment list is stateful (it only reads
        // more moof segments forward), so re-using one reader across many seeks to different
        // points doesn't cleanly test each timestamp in isolation.
        let file = File::open(path).unwrap();
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default()).unwrap();

        let result = reader.seek(SeekMode::Accurate, SeekTo::Time {
            time: Time::try_from_secs_f64(t).unwrap(),
            track_id: Some(track_id),
        });

        match &result {
            Ok(seeked) => {
                println!(
                    "  seek(t={t:.3}s) => Ok(SeekedTo {{ track_id: {}, actual_ts: {}, required_ts: {} }})",
                    seeked.track_id, seeked.actual_ts, seeked.required_ts
                );
            }
            Err(e) => {
                println!("  seek(t={t:.3}s) => Err({e:?})");
            }
        }
    }
}

fn main() {
    let times = [0.0, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    // Baseline: the real, unpatched fragmented MP4 - seeking should succeed everywhere within
    // the track's duration.
    try_seek_all("assets/frag.mp4", &times);

    println!();

    // The candidate: every tfhd's default_sample_duration zeroed.
    try_seek_all("assets/evil.mp4", &times);
}
