//! Checks whether symphonia-format-mkv and symphonia-format-isomp4's `FormatReader::seek()` have
//! the same "peek the next packet/frame, and on a track/stream mismatch skip it with `continue`
//! but never discard/consume it" shape confirmed in candidate 005 for symphonia-format-ogg, which
//! caused an unconditional infinite loop (the same non-matching item is peeked again forever).
//!
//! For each of a real two-track WebM (`assets/two_track.webm`, produced by `ffmpeg`/`libvorbis`)
//! and a real two-track MP4 (`assets/two_track.mp4`, produced by `ffmpeg`/AAC), this seeks every
//! track to a spread of timestamps covering the full duration, on both a seekable (`File`) source
//! and a non-seekable (`ReadOnlySource`) source (the latter is what forced candidate 005's ogg bug
//! to manifest, since it bypasses any byte-offset bisection fast path). Each individual seek call
//! runs on a watchdog thread with a 10s timeout so a real infinite loop is reported as a timeout
//! instead of hanging the whole process forever.
//!
//! If every seek across both formats, both source-seekability modes, and the full timestamp
//! spread returns within the timeout, this exits 0 and prints PASS - meaning no evidence of the
//! candidate-005-style missing-discard bug was found in either format's seek implementation.
//! If any single seek call times out, this exits 1 and prints which (format, source mode, track,
//! timestamp) combination hung.

use std::fs::File;
use std::sync::mpsc;
use std::time::Duration;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia_core::units::Time;

const TIMESTAMPS_SECS: [f64; 6] = [0.1, 1.5, 3.0, 4.9, 6.5, 7.9];
const WATCHDOG_TIMEOUT: Duration = Duration::from_secs(10);

/// Run `f` on a watchdog thread; returns `Some(result)` if it completed within the timeout, or
/// `None` if it timed out (in which case the spawned thread is leaked - the whole point is to
/// detect a real infinite loop without the detector itself hanging).
fn with_watchdog<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(WATCHDOG_TIMEOUT).ok()
}

fn check_mkv(path: &str, non_seekable: bool) -> bool {
    use symphonia_format_mkv::MkvReader;

    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = if non_seekable {
        let source = ReadOnlySource::new(file);
        assert!(!MediaSource::is_seekable(&source));
        MediaSourceStream::new(Box::new(source), Default::default())
    }
    else {
        MediaSourceStream::new(Box::new(file), Default::default())
    };

    let mut reader =
        MkvReader::try_new(mss, FormatOptions::default()).expect("failed to construct MkvReader");
    let track_ids: Vec<u32> = reader.tracks().iter().map(|t| t.id).collect();
    println!("  [mkv, non_seekable={non_seekable}] found {} track(s): {:?}", track_ids.len(), track_ids);

    let mut all_ok = true;
    for &tid in &track_ids {
        for &secs in &TIMESTAMPS_SECS {
            let label = format!("mkv non_seekable={non_seekable} track={tid:#x} time={secs}s");
            print!("    seeking {label} ... ");
            let mut reader_opt = Some(reader);
            let result = with_watchdog(move || {
                let mut r = reader_opt.take().unwrap();
                let ok = r
                    .seek(SeekMode::Accurate, SeekTo::Time {
                        time: Time::try_from_secs_f64(secs).unwrap(),
                        track_id: Some(tid),
                    })
                    .is_ok();
                (r, ok)
            });
            match result {
                Some((r, ok)) => {
                    reader = r;
                    println!("returned (ok={ok})");
                }
                None => {
                    println!("TIMEOUT after {WATCHDOG_TIMEOUT:?} -- possible infinite loop");
                    all_ok = false;
                    // Can't recover the reader (it's stuck on the leaked thread); stop this run.
                    return all_ok;
                }
            }
        }
    }
    all_ok
}

fn check_mp4(path: &str, non_seekable: bool) -> bool {
    use symphonia_format_isomp4::IsoMp4Reader;

    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = if non_seekable {
        let source = ReadOnlySource::new(file);
        assert!(!MediaSource::is_seekable(&source));
        MediaSourceStream::new(Box::new(source), Default::default())
    }
    else {
        MediaSourceStream::new(Box::new(file), Default::default())
    };

    let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
        .expect("failed to construct IsoMp4Reader");
    let track_ids: Vec<u32> = reader.tracks().iter().map(|t| t.id).collect();
    println!("  [mp4, non_seekable={non_seekable}] found {} track(s): {:?}", track_ids.len(), track_ids);

    let mut all_ok = true;
    for &tid in &track_ids {
        for &secs in &TIMESTAMPS_SECS {
            let label = format!("mp4 non_seekable={non_seekable} track={tid:#x} time={secs}s");
            print!("    seeking {label} ... ");
            let mut reader_opt = Some(reader);
            let result = with_watchdog(move || {
                let mut r = reader_opt.take().unwrap();
                let ok = r
                    .seek(SeekMode::Accurate, SeekTo::Time {
                        time: Time::try_from_secs_f64(secs).unwrap(),
                        track_id: Some(tid),
                    })
                    .is_ok();
                (r, ok)
            });
            match result {
                Some((r, ok)) => {
                    reader = r;
                    println!("returned (ok={ok})");
                }
                None => {
                    println!("TIMEOUT after {WATCHDOG_TIMEOUT:?} -- possible infinite loop");
                    all_ok = false;
                    return all_ok;
                }
            }
        }
    }
    all_ok
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("checking symphonia-format-mkv seek() ...");
    let mkv_seekable_ok = check_mkv("assets/two_track.webm", false);
    let mkv_nonseekable_ok = check_mkv("assets/two_track.webm", true);

    println!("checking symphonia-format-isomp4 seek() ...");
    let mp4_seekable_ok = check_mp4("assets/two_track.mp4", false);
    let mp4_nonseekable_ok = check_mp4("assets/two_track.mp4", true);

    let all_ok = mkv_seekable_ok && mkv_nonseekable_ok && mp4_seekable_ok && mp4_nonseekable_ok;

    if all_ok {
        println!("PASS: no hang observed in any (format, source-seekability, track, timestamp) combination");
    }
    else {
        println!("FAIL: at least one seek() call timed out -- see above for which combination");
        std::process::exit(1);
    }
}
