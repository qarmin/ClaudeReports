//! Standalone reproduction for: infinite loop in `OggReader::seek()` reached via the *seekable*
//! (`File`-backed) bisection fast path, not just the non-seekable packet-by-packet scan path
//! demonstrated by candidate 005.
//!
//! `symphonia-format-ogg/src/demuxer.rs`'s `do_seek`, after its byte-range bisection converges on
//! a page belonging to the target logical stream (`self.reader.is_seekable() == true`), falls
//! through into the same "consume packets until reaching the desired timestamp" loop used by the
//! non-seekable path. That loop calls `self.peek_logical_packet()`; when the target page's own
//! buffered packets run out before `target_ts` is reached, it calls `self.read_page()` to fetch
//! the *next physical page* -- which, in an interleaved multi-logical-stream Ogg file, may belong
//! to a *different* logical stream than the one being sought. The loop then peeks a packet whose
//! `track_id` doesn't match the seek target's serial, takes the `continue;` branch without
//! calling `self.discard_logical_packet()`, and spins forever peeking the same never-consumed
//! packet.
//!
//! `assets/out_long.ogg` is a real, ordinary Ogg file produced by `ffmpeg` (libvorbis), containing
//! two independent Vorbis logical bitstreams multiplexed page-by-page -- not hand-crafted bytes.
//! It is opened as a plain `File` (a real, seekable `MediaSource`), so `OggReader::do_seek` takes
//! its bisection fast path. Seeking track 0 to `t=1.07s` makes the bisection land on a page from
//! track 0 whose own packets are exhausted right at the requested timestamp, forcing the "consume
//! packets" loop to read the next physical page, which belongs to track 1 -- driving the reader
//! into the buggy loop through the plain public `FormatReader::seek` API, on the seekable path.

use std::fs::File;
use std::time::Duration;

use symphonia_core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia_core::io::MediaSourceStream;
use symphonia_core::units::Time;
use symphonia_format_ogg::OggReader;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "assets/out_long.ogg".to_string());

    let file = File::open(&path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    assert!(symphonia_core::io::MediaSource::is_seekable(&file), "source must be seekable");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut reader =
        OggReader::try_new(mss, FormatOptions::default()).expect("failed to construct OggReader");

    let tracks = reader.tracks();
    println!("found {} track(s):", tracks.len());
    for t in tracks {
        println!("  track id={:#x}", t.id);
    }

    // Seek the first track to t=1.07s. The bisection (seekable fast path) lands on a page from
    // this track whose buffered packets are exhausted before reaching target_ts, forcing a
    // physical-page crossover into the other logical stream's page while still on the seekable
    // path.
    let target_track_id = tracks[0].id;

    println!("seeking track {target_track_id:#x} to time=1.07s (seekable File source) ...");

    // Run the seek on a watchdog thread so a real infinite loop is reported instead of hanging
    // the whole process forever.
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let result = reader.seek(SeekMode::Accurate, SeekTo::Time {
            time: Time::try_from_secs_f64(1.07).unwrap(),
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
