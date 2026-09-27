//! Investigates ISOMP4_TRUN_WRONG_BITMASK_FIRST_SAMPLE_DURATION.
//!
//! `TrunAtom::is_first_sample_duration_present()` (symphonia-format-isomp4/src/atoms/trun.rs)
//! tests `first_sample_flags & FIRST_SAMPLE_FLAGS_PRESENT` (bit 0x4) instead of
//! `SAMPLE_DURATION_PRESENT` (bit 0x100), unlike the correctly-written parallel
//! `is_first_sample_size_present()`. Bit 0x4 is an ordinary bit that can appear anywhere in a
//! real ISO/IEC 14496-12 sample-flags word (encoders regularly emit these words with the low
//! bits of the padding/degradation-priority sub-fields non-zero). When a `trun`'s
//! `first_sample_flags` happens to have that bit set, while the `trun`'s own
//! `SAMPLE_DURATION_PRESENT` flag is NOT set, `total_duration()` indexes
//! `self.sample_duration[0]` on a `Vec` that is guaranteed empty in that branch.
//!
//! `evil_first_sample_flags.mp4` is a real, otherwise-valid fragmented AAC-in-MP4 file produced
//! by `ffmpeg` (`-movflags frag_keyframe+empty_moov+default_base_moof`, same source as
//! candidate 017's `frag.mp4`), binary-patched only in the first fragment's `trun` atom:
//!   - `trun` flags: 0x1 (data-offset-present) -> 0x5 (+ first-sample-flags-present)
//!   - inserted `first_sample_flags = 0x02000004`, a realistic ISO/IEC 14496-12 sample-flags
//!     word (`0x02000000` is the exact `sample_depends_on = 2` / "does not depend on other
//!     samples" value already used, unmodified, by this same file's own `tfhd.default_sample_flags`
//!     field - only the low `0x4` bit was added on top of it) - with the `trun`'s own
//!     `SAMPLE_DURATION_PRESENT` (0x100) flag correctly left unset, exactly as
//!     `is_first_sample_duration_present()`'s wrong bitmask makes it misinterpret as "the first
//!     sample duration is present".
//!   - `traf`/`moof` atom sizes updated (+4 bytes) and `trun.data_offset` bumped by 4 to keep the
//!     file otherwise byte-for-byte identical and structurally valid.
//!
//! This exercises the real, public `IsoMp4Reader::try_new` + `FormatReader::next_packet` entry
//! points - exactly what any real caller (a decoder, a media player) would use to read a
//! fragmented/CMAF file - with no direct calls into `TrunAtom` at all.
use std::fs::File;
use std::panic;

use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::MediaSourceStream;
use symphonia_format_isomp4::IsoMp4Reader;

fn try_read_first_fragment_packet(path: &str) -> std::thread::Result<()> {
    panic::catch_unwind(|| {
        let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut reader = IsoMp4Reader::try_new(mss, FormatOptions::default())
            .unwrap_or_else(|e| panic!("failed to construct IsoMp4Reader from {path}: {e}"));

        println!("opened {path} successfully, track_id={}", reader.tracks()[0].id);

        // The moov atom does not itself contain any moof/trun data - reading the first packet is
        // what makes the real demuxer walk forward into the first moof/traf/trun and build a
        // MoofSegment out of it (symphonia-format-isomp4/src/demuxer.rs try_read_more_segments,
        // called from next_packet), which is exactly where TrunAtom::total_duration() is invoked.
        match reader.next_packet() {
            Ok(Some(packet)) => {
                println!("  next_packet() => Ok(Packet {{ len: {} }})", packet.data.len());
            }
            Ok(None) => println!("  next_packet() => Ok(None)"),
            Err(e) => println!("  next_packet() => Err({e:?})"),
        }
    })
}

fn main() {
    println!("=== baseline: assets/frag.mp4 (real, unmodified ffmpeg output) ===");
    match try_read_first_fragment_packet("assets/frag.mp4") {
        Ok(()) => println!("  no panic"),
        Err(_) => println!("  PANICKED"),
    }

    println!();
    println!("=== candidate: assets/evil_first_sample_flags.mp4 (bit 0x4 set in first_sample_flags) ===");

    let mut panicked = 0;
    let mut ok = 0;
    for i in 1..=3 {
        println!("-- run {i}/3 --");
        match try_read_first_fragment_packet("assets/evil_first_sample_flags.mp4") {
            Ok(()) => {
                ok += 1;
                println!("  no panic");
            }
            Err(_) => {
                panicked += 1;
                println!("  PANICKED (caught above via catch_unwind)");
            }
        }
    }

    println!();
    println!("summary: {panicked}/3 runs panicked, {ok}/3 runs completed without panic");
}
