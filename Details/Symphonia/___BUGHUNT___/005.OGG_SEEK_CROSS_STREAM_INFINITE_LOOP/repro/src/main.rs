//! Standalone reproduction for: infinite loop in `OggReader::seek()` when the packet skip loop
//! encounters a packet belonging to a different logical bitstream than the seek target.
//!
//! `symphonia-format-ogg/src/demuxer.rs`'s `do_seek` "consume packets until reaching the desired
//! timestamp" loop calls `self.peek_logical_packet()`. When the peeked packet's `track_id`
//! doesn't match the seek target's serial, it does `continue;` without calling
//! `self.discard_logical_packet()`. Peeking does not advance the read position, so the same
//! non-matching packet is peeked again on the next iteration, forever.
//!
//! `assets/out_long.ogg` is a real, ordinary Ogg file produced by `ffmpeg` (libvorbis), containing
//! two independent Vorbis logical bitstreams multiplexed page-by-page - not hand-crafted bytes.
//! It is wrapped in `ReadOnlySource`, the crate's own public wrapper for a non-seekable source
//! (the same wrapper a real caller uses for a network/pipe input, see e.g. `symphonia-play`),
//! which makes `OggReader::do_seek` skip its byte-range-bisection fast path (available only for
//! seekable sources) and fall through to its packet-by-packet scan from the current read
//! position instead. That scan must cross pages belonging to the *other* logical stream before
//! reaching the target track's packet at the requested timestamp, driving the reader straight
//! into the buggy loop through the plain public `FormatReader::seek` API.

use std::fs::File;
use std::time::Duration;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::{MediaSourceStream, ReadOnlySource};
use symphonia_core::units::Time;
use symphonia_format_ogg::OggReader;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let path = std::env::args().nth(1).unwrap_or_else(|| "assets/out_long.ogg".to_string());

    let file = File::open(&path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let source = ReadOnlySource::new(file);
    assert!(!symphonia_core::io::MediaSource::is_seekable(&source), "source must be non-seekable");
    let mss = MediaSourceStream::new(Box::new(source), Default::default());

    let mut reader =
        OggReader::try_new(mss, FormatOptions::default()).expect("failed to construct OggReader");

    let tracks = reader.tracks();
    println!("found {} track(s):", tracks.len());
    for t in tracks {
        println!("  track id={:#x}", t.id);
    }

    // Seek the second track (serial) to ~1.0s. Because pages/packets from both logical streams
    // are interleaved, reaching this timestamp on this track requires skipping over at least one
    // packet belonging to the other logical stream first.
    let target_track_id = tracks[1].id;

    println!("seeking track {target_track_id:#x} to time=1.0s ...");

    // Run the seek on a watchdog thread so a real infinite loop is reported instead of hanging
    // the whole process forever.
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let result =
            reader.seek(SeekMode::Accurate, SeekTo::Time {
                time: Time::try_from_secs_f64(1.0).unwrap(),
                track_id: Some(target_track_id),
            });
        let _ = tx.send(result.is_ok());
    });

    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(ok) => {
            println!("seek returned: ok={ok}");
            handle.join().unwrap();
        }
        Err(_) => {
            println!("TIMEOUT: seek() did not return within 10s -- infinite loop confirmed");
            std::process::exit(1);
        }
    }
}
