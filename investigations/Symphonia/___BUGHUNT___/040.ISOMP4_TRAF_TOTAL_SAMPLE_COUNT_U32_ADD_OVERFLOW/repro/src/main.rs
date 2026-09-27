//! Standalone reproduction for: `TrafAtom::read`
//! (symphonia-format-isomp4/src/atoms/traf.rs:40) accumulates every `TrunAtom::sample_count`
//! found in a track fragment into a plain `u32` with an unchecked `+=`:
//!
//! ```ignore
//! AtomType::TrackFragmentRun => {
//!     let trun = it.read_atom::<TrunAtom>()?;
//!     total_sample_count += trun.sample_count;
//!     truns.push(trun);
//! }
//! ```
//!
//! `trun.sample_count` (trun.rs:246) is a raw, unvalidated `u32` read straight from the file, and
//! `TrunAtom::read`'s own per-sample loop (trun.rs:274-297) reads zero bytes per sample when none
//! of the SAMPLE_DURATION/SIZE/FLAGS/COMPOSITION_TIME_OFFSETS_PRESENT flags are set - so a `trun`
//! claiming `sample_count = 0x80000000` costs only 16 bytes on disk. Two such `trun`s inside one
//! `traf` sum to `0x100000000`, one past `u32::MAX`, panicking with "attempt to add with overflow"
//! in an overflow-checks build.
//!
//! `traf_overflow.m4a` is a real fragmented MP4 (ftyp/moov/mvex/trex/moof/mfhd/traf/tfhd/trun x2),
//! produced by `gen_mp4.py` from a genuine `ffmpeg`-encoded fragmented AAC file with its single
//! real `trun` replaced by the two crafted ones described above - see that script's own docstring.
//! The file is read exclusively through the real, public `IsoMp4Reader::try_new` +
//! `FormatReader::next_packet` entry points, exactly what any real player/caller does to demux a
//! fragmented MP4 - there is no second participant being faked here (a malformed file is the
//! entire external input; see CONFIRMATION.md section 11's "single external artifact" exception).
//! `try_new`'s own initial atom scan does not parse the moof/traf - that only happens lazily,
//! inside `try_read_more_segments`, the first time `next_packet()` needs to look past the initial
//! `moov`-derived segment, which is the real, ordinary demuxing path for any fragmented MP4.

use std::fs::File;

use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::MediaSourceStream;
use symphonia_format_isomp4::IsoMp4Reader;

fn run(path: &str) {
    println!("=== {path} ===");
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
        .unwrap_or_else(|e| panic!("IsoMp4Reader::try_new failed unexpectedly: {e}"));

    // Draining packets is what drives the demuxer past the initial moov-derived segment and into
    // parsing the crafted moof/traf - this is where the overflow panic actually happens.
    loop {
        match reader.next_packet() {
            Ok(Some(_packet)) => continue,
            Ok(None) => break,
            Err(e) => {
                println!("  next_packet() => Err({e}) (no panic: fix likely applied)");
                return;
            }
        }
    }

    println!("  demuxed to end with no panic (unexpected for the overflow asset)");
}

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());

    if which == "all" || which == "ok" {
        // Control: the unmodified ffmpeg output. Must never panic.
        run("assets/control_ok.m4a");
    }
    if which == "all" || which == "overflow" {
        run("assets/traf_overflow.m4a");
    }
    if which == "all" || which == "overflow2" {
        // A second input in the same overflow class (CONFIRMATION.md section 7): a different
        // sample_count split (0xFFFFFFFE + 3, overflowing by 2) than the first asset's exact
        // 0x80000000 + 0x80000000 split - confirms the fix isn't narrowly tied to one value pair.
        run("assets/traf_overflow2.m4a");
    }
}
