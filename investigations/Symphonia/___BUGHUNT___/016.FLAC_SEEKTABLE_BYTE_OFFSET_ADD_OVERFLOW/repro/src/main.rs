//! Standalone reproduction for: `FlacReader::seek()`'s seekable binary-search bisection
//! (symphonia-bundle-flac/src/demuxer.rs) computes `start_byte_offset`/`end_byte_offset` from
//! `self.first_frame_offset + <SeekPoint>::byte_offset`, where `byte_offset` is a raw `u64` read
//! directly off the FLAC SEEKTABLE metadata block
//! (symphonia-metadata/src/embedded/flac.rs::read_flac_seektable_block) with no validation
//! against the actual stream length or against `first_frame_offset`.
//!
//! Two crafted seek tables are provided as separate assets, each triggering a checked-arithmetic
//! panic ("attempt to add with overflow") at a different line of the same function:
//!
//!   - `assets/seek_overflow_real.flac`: a single seek point with `byte_offset = u64::MAX`
//!     triggers `SeekSearchResult::Lower`, panicking at the
//!     `start_byte_offset = self.first_frame_offset + lower.byte_offset;` line (demuxer.rs:305).
//!
//!   - `assets/seek_overflow_midpoint.flac`: two seek points, each with `byte_offset` close to
//!     `u64::MAX / 2`, trigger `SeekSearchResult::Range`, both individual additions with
//!     `first_frame_offset` stay in range, but the loop's own
//!     `let mid_byte_offset = (start_byte_offset + end_byte_offset) / 2;` line (demuxer.rs:322 -
//!     the exact `clippy::manual_midpoint` pattern this candidate set out to check) then panics -
//!     this is the literal midpoint expression overflowing, driven entirely by
//!     attacker-controlled seek-table bytes rather than an implausible multi-exabyte real file.
//!
//! Both files are real, format-valid (marker + STREAMINFO + SEEKTABLE, built by post-processing a
//! genuine ffmpeg-encoded FLAC file - see `gen_flac.py`/README for how `seek_overflow_real.flac`
//! was hand-built and how `seek_overflow_midpoint.flac` was derived) FLAC files, read through the
//! plain public `FlacReader::try_new` + `FormatReader::seek` API - no in-process stand-in for the
//! demuxer's internals, and no second participant to fake (the crafted file is the only external
//! input, exactly what a real attacker/user would hand to a player).

use std::fs::File;

use symphonia_bundle_flac::FlacReader;
use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_core::units::Timestamp;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "assets/seek_overflow_midpoint.flac".to_string());
    let ts: i64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(500);

    let file = File::open(&path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader =
        FlacReader::try_new(mss, FormatOptions::default()).expect("failed to construct FlacReader");

    println!("constructed FlacReader from {path}, attempting seek(ts={ts})...");

    let result =
        reader.seek(SeekMode::Coarse, SeekTo::Timestamp { ts: Timestamp::from(ts), track_id: 0 });

    match result {
        Ok(seeked) => {
            println!(
                "seek returned Ok: required_ts={}, actual_ts={} (no panic - either the fix is \
                 applied, or overflow-checks are off and the arithmetic silently wrapped)",
                seeked.required_ts, seeked.actual_ts
            );
        }
        Err(e) => {
            println!(
                "seek returned Err (no panic: either the fix is applied and the byte range was \
                 safely clamped, producing an ordinary IO error instead of a wild seek; or, \
                 without checked arithmetic, the wrapped offset happened to still be invalid): {e}"
            );
        }
    }
}
