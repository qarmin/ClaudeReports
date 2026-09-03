//! Standalone reproduction for: `PacketBuilder::trimmed_dur` computes fresh, correct local
//! `trim_start`/`trim_end` values from the packet's PTS and the stream's end timestamp, but then
//! computes the built packet's `dur` field using the stale, still-default (`Duration::ZERO`)
//! `self.trim_start`/`self.trim_end` fields left over from the builder's earlier partial
//! `let Self { track_id, pts, buf, dts, .. } = self;` destructure, instead of the just-computed
//! local `trim_start`/`trim_end` variables. The built packet's `.dur` therefore always equals the
//! full `block_dur`, silently ignoring any encoder-delay/padding trim.
//!
//! `symphonia-bundle-mp3`'s `MpaReader::next_packet` is the only caller of `trimmed_dur` in this
//! codebase today (MP3 LAME/Xing gapless trimming). `gapless.mp3` is a real, ordinary MP3 file
//! produced by `ffmpeg`/`libmp3lame` (not hand-crafted bytes) - libmp3lame always writes a Xing
//! header with a LAME extension recording nonzero encoder delay (and, for most inputs, padding),
//! so the very first packet the demuxer produces has a negative PTS (`-delay`), which is exactly
//! the case `trimmed_dur` is supposed to trim. This repro drives the real, public
//! `symphonia_bundle_mp3::MpaReader::try_new` / `FormatReader::next_packet` entry point end-to-end
//! on that real file and inspects the returned `Packet`'s public `trim_start`/`dur`/`pts` fields -
//! no internal state is hand-constructed.

use std::fs::File;

use symphonia_bundle_mp3::MpaReader;
use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::MediaSourceStream;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "assets/gapless.mp3".to_string());

    let file = File::open(&path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader =
        MpaReader::try_new(mss, FormatOptions::default()).expect("failed to construct MpaReader");

    let first =
        reader.next_packet().expect("failed to read first packet").expect("expected a packet");

    println!(
        "packet[0] (has encoder delay): pts={} dur={} trim_start={} trim_end={}",
        first.pts.get(),
        first.dur.get(),
        first.trim_start.get(),
        first.trim_end.get()
    );

    assert!(
        first.pts.is_negative(),
        "test file's first packet must have a negative PTS (encoder delay) for this repro to be \
         meaningful - got pts={}; the MP3 file may not carry LAME encoder-delay metadata",
        first.pts.get()
    );

    let expected_trim_start = (-first.pts.get()) as u64;
    assert_eq!(
        first.trim_start.get(),
        expected_trim_start,
        "trim_start itself should already be correct (it's the freshly computed local variable) \
         -- if this fails, something else changed"
    );

    // Find a later packet that is unaffected by trimming (trim_start == 0, trim_end == 0) to
    // learn the raw, untrimmed per-frame block duration ("block_dur") independently of packet[0].
    let mut reference_dur = None;
    for _ in 0..8 {
        let pkt = match reader.next_packet().expect("failed to read packet") {
            Some(p) => p,
            None => break,
        };
        if pkt.trim_start.get() == 0 && pkt.trim_end.get() == 0 {
            reference_dur = Some(pkt.dur.get());
            break;
        }
    }
    let reference_dur = reference_dur.expect("expected an untrimmed reference packet");

    println!("reference (untrimmed) packet dur={reference_dur}");

    // Correct behaviour: packet[0].dur should be reference_dur - trim_start (since block_dur is
    // the same raw per-frame duration for every packet, and packet[0] must additionally have its
    // encoder-delay frames subtracted). The bug: PacketBuilder::trimmed_dur ignores the just-
    // computed trim_start/trim_end when calculating dur, so packet[0].dur ends up equal to the
    // full, untrimmed reference_dur instead.
    let expected_dur = reference_dur.saturating_sub(expected_trim_start);

    if first.dur.get() == expected_dur {
        println!("OK: packet[0].dur ({}) correctly excludes the encoder delay", first.dur.get());
    }
    else if first.dur.get() == reference_dur {
        println!(
            "BUG: packet[0].dur ({}) == full untrimmed block_dur ({reference_dur}), expected \
             {expected_dur} (= block_dur - trim_start) -- PacketBuilder::trimmed_dur computed dur \
             using the stale, still-default self.trim_start/self.trim_end fields instead of the \
             freshly computed local trim_start/trim_end values",
            first.dur.get()
        );
        std::process::exit(1);
    }
    else {
        println!(
            "UNEXPECTED: packet[0].dur={} (neither the correct {expected_dur} nor the buggy \
             {reference_dur})",
            first.dur.get()
        );
        std::process::exit(2);
    }
}
