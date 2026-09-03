// Repro for candidate 039: MKV_CUEPOINT_DROPS_ALL_BUT_LAST_TRACKPOSITIONS.
//
// Feeds real (structurally valid) two-track Matroska files to symphonia-format-mkv's public
// `MkvReader`, through its real `FormatReader::seek` entry point, and shows that seeking a track
// to timestamp 0 returns a wildly wrong `actual_ts` whenever the CuePoint's OTHER track's
// `CueTrackPositions` happens to be the last one parsed - `CuePointElement::read` in segment.rs
// only kept the LAST `CueTrackPositions` child of a `CuePoint`, discarding the rest, and
// `seek_track_by_ts` in demuxer.rs never checked that the retained position's `track` actually
// matched the track being sought.
//
// The two clusters and all block/track data in every test file are 100% real bytes produced by a
// real ffmpeg mux (h264 video + aac audio). The only hand-authored part is each file's `Cues`
// element, which mirrors what a real multi-track-aware muxer (e.g. mkvmerge) commonly writes: one
// `CuePoint` per timestamp with one `CueTrackPositions` child per indexed track. This is a single
// external file handed to the real public parsing/seeking entry point - there is no second
// participant being faked here (see CONFIRMATION.md section 11).
//
// Two independent boundary cases are exercised (CONFIRMATION.md section 7: vary the input within
// the same bug class, not just the one that first triggered it):
//   - poisoned.mkv:  CuePoint has [video(track 1), audio(track 2)] -> audio position retained ->
//     seeking the VIDEO track (1) to ts=0 is redirected into audio's cluster (2).
//   - poisoned2.mkv: CuePoint has [audio(track 2), video(track 1)] -> video position retained ->
//     seeking the AUDIO track (2) to ts=0 is redirected into video's cluster (1).

use std::env;
use std::fs::File;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_core::units::Timestamp;
use symphonia_format_mkv::MkvReader;

fn open(path: &str) -> Box<MkvReader<'static>> {
    let file = File::open(path).expect("open input file");
    let mss = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());
    Box::new(MkvReader::try_new(mss, FormatOptions::default()).expect("MkvReader::try_new"))
}

fn seek_track(path: &str, track_id: u32, req_ts: i64) -> symphonia_core::errors::Result<Timestamp> {
    let mut reader = open(path);
    let seeked =
        reader.seek(SeekMode::Accurate, SeekTo::Timestamp { ts: Timestamp::from(req_ts), track_id })?;
    println!(
        "{path}: seek(track_id={track_id}, ts={req_ts}) -> required_ts={:?} actual_ts={:?}",
        seeked.required_ts, seeked.actual_ts
    );
    Ok(seeked.actual_ts)
}

/// Compares a "good" (correctly-cued) seek result against a "poisoned" one for the same request.
/// `max_good_delta` bounds how far from the requested timestamp the *correct* answer may
/// legitimately land (accounting for keyframe/frame spacing). The bug is confirmed if the good
/// file lands within that bound while the poisoned file either lands far outside it, or fails the
/// seek outright (it was redirected into a cluster that can't satisfy the request at all).
fn check_case(
    name: &str,
    req_ts: i64,
    max_good_delta: i64,
    good_ts: symphonia_core::errors::Result<Timestamp>,
    poisoned_ts: symphonia_core::errors::Result<Timestamp>,
) -> bool {
    let good_ok = matches!(good_ts, Ok(ts) if (ts.get() - req_ts).abs() <= max_good_delta);
    let bug_triggered = match (good_ts, poisoned_ts) {
        (Ok(good), Ok(poisoned)) => (poisoned.get() - good.get()).abs() > max_good_delta,
        (Ok(_), Err(ref e)) => {
            println!("[{name}] poisoned seek failed outright: {e}");
            true
        }
        _ => false,
    };
    println!(
        "[{name}] good lands within {max_good_delta} of ts={req_ts}: {good_ok}; poisoned diverges from good (bug reproduced): {bug_triggered}"
    );
    good_ok && bug_triggered
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: repro <good.mkv> <poisoned.mkv> <poisoned2.mkv>");
        std::process::exit(2);
    }
    let (good_path, poisoned_path, poisoned2_path) = (&args[1], &args[2], &args[3]);

    println!("== case A: video track (1) redirected via audio's CueTrackPositions ==");
    let good_video = seek_track(good_path, 1, 0);
    let poisoned_video = seek_track(poisoned_path, 1, 0);
    let case_a_ok = check_case("video-track redirected", 0, 100, good_video, poisoned_video);

    println!();
    println!("== case B: audio track (2) redirected via video's CueTrackPositions ==");
    let good_audio = seek_track(good_path, 2, 3000);
    let poisoned2_audio = seek_track(poisoned2_path, 2, 3000);
    let case_b_ok = check_case("audio-track redirected", 3000, 100, good_audio, poisoned2_audio);

    println!();
    if !case_a_ok || !case_b_ok {
        eprintln!("REPRO FAILED TO DEMONSTRATE THE BUG (or bug is fixed) in at least one case");
        std::process::exit(1);
    }

    println!("REPRO CONFIRMED (both cases): a CuePoint with multiple CueTrackPositions caused seeking one track to use another track's cluster position.");
}
