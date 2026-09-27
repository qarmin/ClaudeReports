//! Candidate 023 (FLAC_APPLICATION_BLOCK_LENGTH_UNDERFLOW).
//!
//! Demonstrates two things:
//!
//! 1. The REAL end-to-end trigger path: a hand-crafted, malformed FLAC file with an APPLICATION
//!    metadata block whose declared length is 0..=3 bytes, fed to the real, public
//!    `symphonia_bundle_flac::FlacReader::try_new` entry point (exactly what a real decoder
//!    embedding this crate would call on an untrusted file). This is the genuine external
//!    input a real attacker/user could supply - there is no second participant to fake here.
//!
//! 2. For comparison/root-causing only (NOT the claimed bug path): calling the low-level
//!    `symphonia_metadata::embedded::flac::read_flac_application_block` function directly with a
//!    mismatched length parameter that does not correspond to how the real caller (the FLAC
//!    demuxer) ever invokes it - this shows the `usize` underflow/panic IS real in the function
//!    in isolation, but is never reachable via the real end-to-end path exercised in part 1.

use std::io::Cursor;

use symphonia_bundle_flac::FlacReader;
use symphonia_core::formats::FormatOptions;
use symphonia_core::io::{BufReader, MediaSourceStream, MediaSourceStreamOptions};
use symphonia_metadata::embedded::flac::read_flac_application_block;

fn build_flac_with_application_block(block_len: u8, body: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"fLaC");

    // Metadata block header byte: is_last=1 (0x80) | block_type=2 (APPLICATION).
    buf.push(0x80 | 2);
    // 24-bit big-endian block length.
    buf.push(0);
    buf.push(0);
    buf.push(block_len);

    buf.extend_from_slice(body);
    buf
}

fn try_real_path(block_len: u8) {
    let body = vec![0xAAu8; block_len as usize];
    let data = build_flac_with_application_block(block_len, &body);

    let mss =
        MediaSourceStream::new(Box::new(Cursor::new(data)), MediaSourceStreamOptions::default());

    print!(
        "[real path] APPLICATION block, declared block_len={block_len} (body_len={}) via FlacReader::try_new: ",
        body.len()
    );

    match FlacReader::try_new(mss, FormatOptions::default()) {
        Ok(_) => println!("Ok(..) -- UNEXPECTED, no error and no panic"),
        Err(e) => println!("Err({e}) -- clean error, no panic"),
    }
}

fn main() {
    println!("=== Part 1: real end-to-end path (FlacReader::try_new on a crafted file) ===");
    for block_len in 0u8..=4 {
        try_real_path(block_len);
    }

    println!();
    println!(
        "=== Part 2: direct, unscoped call to read_flac_application_block (NOT the real caller's usage) ==="
    );
    println!("(the real demuxer never calls this function this way - see RESULT.md)");

    let data = vec![b'A', b'B', b'C', b'D', 0xAA];
    let mut reader = BufReader::new(&data);
    // block_length=3 passed directly, without first scoping the reader to 3 bytes the way the
    // real demuxer always does. This is expected to panic (attempt to subtract with overflow).
    let _ = read_flac_application_block(&mut reader, 3);
    println!("direct call returned without panicking -- UNEXPECTED");
}
