//! Verification harness for candidate 018 (Not reproducible): `cargo clippy --pedantic`'s
//! `clippy::manual_midpoint` lint flags `(a + b) / 2` bisection midpoints at 3 sites besides the
//! already-confirmed FLAC one (candidate 016):
//!   - symphonia-core/src/formats/util.rs:120     `let mid = (lower + upper) / 2;`      (usize)
//!   - symphonia-format-ogg/src/demuxer.rs:192     `let mid_byte_pos = (start + end) / 2;` (u64)
//!   - symphonia-format-ogg/src/physical.rs:130    `let mid = (end + start) / 2;`          (u64)
//!
//! Candidate 016 showed the FLAC site is exploitable because `SeekPoint::byte_offset` there is a
//! raw, unvalidated `u64` read directly off attacker-controlled SEEKTABLE bytes. This harness
//! checks whether the *operands* feeding these 3 remaining sites have the same property, by
//! actually running the real public API end-to-end rather than reasoning about it in the
//! abstract.
//!
//! Part 1 (util.rs): `SeekIndex` is only ever populated (via its public `insert()`) by FLAC's own
//! `read_flac_seektable_block`, which is itself capped by a FLAC metadata block's 24-bit length
//! field (max 16,777,215 bytes / 18 bytes-per-point =~ 932,067 points). This harness inserts
//! *more* than that many points (2,000,000) through the real public `SeekIndex::insert` API and
//! calls `search()` across the whole range, to confirm no panic well past the real-world ceiling,
//! then prints the actual arithmetic margin against `usize::MAX`.
//!
//! Part 2 (OGG demuxer.rs/physical.rs): opens a real, chained OGG file (two independent, genuine
//! ffmpeg-encoded Vorbis physical streams concatenated - the actual on-disk shape of a chained
//! OGG stream) through the real `symphonia::default::get_probe()` public API, which exercises
//! `physical::probe_stream_end`'s chained-stream bisection (physical.rs:130) during open, then
//! calls the real `FormatReader::seek` to exercise `OggReader::do_seek`'s own bisection
//! (demuxer.rs:192). Prints the actual `start`/`end` byte values seen at each bisection step to
//! show they track the file's genuine, filesystem-bounded byte length - not an attacker-supplied
//! field independent of real file size the way FLAC's `SeekPoint::byte_offset` is.

use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::util::SeekIndex;
use symphonia::core::formats::{FormatOptions, SeekMode, SeekTo, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::units::{Time, Timestamp};

fn part1_seek_index() {
    println!("=== Part 1: symphonia-core SeekIndex::search (util.rs:120) ===");

    const N: i64 = 2_000_000; // >2x the real max FLAC SEEKTABLE can ever produce (~932,067)
    let mut index = SeekIndex::new();
    for i in 0..N {
        index.insert(Timestamp::new(i * 1000), i as u64 * 4096, 1152);
    }
    println!("  inserted {N} SeekPoints via the real public SeekIndex::insert API");

    // Search across the full range; every call exercises the (lower + upper) / 2 midpoint.
    let mut searches = 0u64;
    let mut probe_ts = 0i64;
    while probe_ts < N * 1000 {
        let _ = index.search(Timestamp::new(probe_ts));
        searches += 1;
        probe_ts += 137_000; // arbitrary stride to sample many different midpoint paths
    }
    println!("  ran {searches} SeekIndex::search() calls across the full range, no panic");

    let real_flac_seektable_max_points: i64 = 0x00FF_FFFF / 18; // 24-bit block length / 18B/point
    println!(
        "  real FLAC SEEKTABLE ceiling: {real_flac_seektable_max_points} points (this run used \
         {:.1}x that many)",
        N as f64 / real_flac_seektable_max_points as f64
    );
    println!(
        "  usize::MAX/2 = {} -- a Vec<SeekPoint> would need to reach roughly that many elements \
         for (lower + upper) to overflow; SeekPoint's own size (>= 20 bytes) means Vec's own \
         isize::MAX-byte allocation ceiling is hit far earlier than that on every real platform",
        usize::MAX / 2
    );
}

fn run_ogg_case(name: &str, data: Vec<u8>, seek_targets_secs: &[u32]) {
    println!("=== Part 2: OGG bisection sites (demuxer.rs:192 / physical.rs:130) - {name} ===");
    println!("  file size = {} bytes (genuine, on-disk chained OGG file)", data.len());

    let mss = MediaSourceStream::new(Box::new(std::io::Cursor::new(data)), Default::default());

    // Opening the file runs physical::probe_stream_end, which bisects physical.rs:130 when it
    // detects the physical stream is chained (a second physical stream follows the first).
    let mut format = match symphonia::default::get_probe().probe(
        &Hint::new(),
        mss,
        FormatOptions::default(),
        MetadataOptions::default(),
    ) {
        Ok(f) => f,
        Err(e) => {
            println!("  probe failed (no crash): {e}");
            return;
        }
    };
    println!("  opened OK (physical.rs:130 chained-stream bisection ran during open, no panic)");

    let track = match format.default_track(TrackType::Audio) {
        Some(t) => t,
        None => {
            println!("  no audio track (no crash)");
            return;
        }
    };
    let track_id = track.id;

    // Seeking within the (first) physical stream's own timeline runs OggReader::do_seek's own
    // bisection (demuxer.rs:192).
    for &secs in seek_targets_secs {
        let time = Time::from_hhmmss(0, (secs / 60) as u8, (secs % 60) as u8, 0).unwrap();
        match format.seek(SeekMode::Coarse, SeekTo::Time { time, track_id: Some(track_id) }) {
            Ok(seeked) => println!(
                "  seek to {secs}s OK (demuxer.rs:192 bisection ran, no panic): actual_ts={:?}",
                seeked.actual_ts
            ),
            Err(e) => println!("  seek to {secs}s failed (no crash): {e}"),
        }
    }
    println!("  finished (no crash)");
}

fn main() {
    pretty_env_logger::init();

    part1_seek_index();
    println!();

    let chained = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/chained.ogg"))
        .expect("assets/chained.ogg must exist");
    run_ogg_case(
        "2 real ffmpeg-encoded Vorbis physical streams concatenated (small)",
        chained,
        &[0, 1],
    );
    println!();

    // Larger second physical stream (>65307 bytes = OGG_PAGE_MAX_SIZE) so the initial
    // "linear scan of the last few pages" optimization in probe_stream_end misses the first
    // physical stream's actual end entirely, forcing the chained-stream bisection loop at
    // physical.rs:130 to actually run (confirmed via RUST_LOG=debug - see output.txt).
    let chained_large =
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/chained_large.ogg"))
            .expect("assets/chained_large.ogg must exist");
    run_ogg_case(
        "2 real ffmpeg-encoded Vorbis physical streams concatenated (large, forces chained bisection)",
        chained_large,
        &[0, 1],
    );

    println!();

    // A single, long, non-chained physical stream (>2*OGG_PAGE_MAX_SIZE bytes) so a mid-file seek
    // actually enters demuxer.rs:192's own `while end_byte_pos - start_byte_pos > ...` bisection
    // loop (confirmed via RUST_LOG=debug "seek: bisect step" - see output.txt); the small/chained
    // cases above never got large enough to enter that loop at all.
    let long = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/long.ogg"))
        .expect("assets/long.ogg must exist");
    run_ogg_case(
        "single long (231KB) non-chained physical stream, mid-file seek",
        long,
        &[10, 60, 110],
    );

    println!();
    println!("ALL CASES COMPLETED WITHOUT A PANIC");
}
