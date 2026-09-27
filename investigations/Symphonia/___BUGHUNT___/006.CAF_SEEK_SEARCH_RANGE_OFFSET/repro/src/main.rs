//! Standalone reproduction for: `CafReader::seek`'s `PacketInfo::VariableAudioPacket` arm
//! (symphonia-format-caf/src/demuxer.rs) computes `seek_packet_index` from
//! `packets[search_range].partition_point(...)`, which returns an index *relative to* the
//! `search_range` sub-slice, but then uses that index directly to index the *full* `packets`
//! array without adding back `search_range.start`. Whenever `current_packet_index > 0` and the
//! seek target is forward of the current position (`search_range = current_packet_index..len`),
//! this lands on the wrong, earlier packet.
//!
//! `assets/variable_packets.caf` is a real, ordinary (non-adversarial) CAF file: a valid `desc`
//! chunk describing a variable-bitrate MPEG Layer 3 stream (`bytes_per_packet == 0 &&
//! frames_per_packet == 0`, which selects `PacketInfo::VariableAudioPacket`), a `pakt` chunk with
//! 6 packets of 10 frames each, and a `data` chunk. It is read through the plain public
//! `CafReader::try_new` + `FormatReader::next_packet`/`seek` API - no hand-authored in-process
//! stand-in for the format reader's internals, and no second participant to fake here (the CAF
//! file itself is the only external input, read via the crate's real public entry point).

use std::fs::File;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_core::units::Timestamp;
use symphonia_format_caf::CafReader;

fn main() {
    // Packet table (fixed by assets/variable_packets.caf):
    //   index: 0    1    2    3    4    5
    //   start:  0   10   20   30   40   50
    //
    // Args (all optional, with defaults reproducing the original observation):
    //   argv[1] = path to the CAF file
    //   argv[2] = number of packets to read via next_packet() before seeking (moves
    //             current_packet_index forward first, same as normal playback)
    //   argv[3] = required_ts (frames) to seek to
    //   argv[4] = expected_actual_ts (frames) after a *correct* seek
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "assets/variable_packets.caf".to_string());
    let reads: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(3);
    let required_ts_frames: i64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(45);
    let expected_actual_ts_frames: i64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(40);

    let file = File::open(&path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader =
        CafReader::try_new(mss, FormatOptions::default()).expect("failed to construct CafReader");

    // Advance the internal packet cursor by `reads` packets, the same way a normal playback loop
    // would, so that `current_packet_index > 0` when the seek below runs.
    for i in 0..reads {
        let packet = reader.next_packet().expect("next_packet failed").expect("packet missing");
        println!("read packet {i}: ts={}", packet.pts);
    }

    let required_ts = Timestamp::from(required_ts_frames);
    let result = reader
        .seek(SeekMode::Accurate, SeekTo::Timestamp { ts: required_ts, track_id: 0 })
        .expect("seek failed");

    println!("seek result: required_ts={}, actual_ts={}", result.required_ts, result.actual_ts);

    let expected_actual_ts = Timestamp::from(expected_actual_ts_frames);

    if result.actual_ts == expected_actual_ts {
        println!("CORRECT: actual_ts == {expected_actual_ts} as expected");
    }
    else {
        println!(
            "BUG CONFIRMED: seeking to required_ts={required_ts} after reading {reads} packet(s) \
             landed on actual_ts={} (expected {expected_actual_ts})",
            result.actual_ts
        );
        std::process::exit(1);
    }
}
