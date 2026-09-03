//! Standalone repro for candidate 009 (MKV_NESTED_ELEMENT_RECURSION_DEPTH).
//!
//! Builds a real Matroska/WebM byte stream with a chain of deeply-nested `ChapterAtom` EBML
//! master elements (`Segment > Chapters > EditionEntry > ChapterAtom > ChapterAtom > ...`) and
//! feeds it to the real public entry point, `symphonia_format_mkv::MkvReader::try_new`, exactly
//! as a real container-probing caller would for an untrusted file. This is a single-participant
//! parser input (a malformed/adversarial file handed to a parser's own public `try_new`), so no
//! second participant needs to be faked - see CONFIRMATION.md section 11's "single-process,
//! single-participant" carve-out.
//!
//! Result (see ../output.txt for the actual captured run): every nesting depth up to and
//! including 60,000 levels parses without crashing, in both debug and release builds. Depths
//! above the schema's `MkvSchema::MAX_DEPTH = 16` fail fast with a clean
//! `DecodeError("mkv (ebml): maximum ebml document depth reached")` instead of recursing further.

use std::io::Cursor;

use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_format_mkv::MkvReader;

const EBML: &[u8] = &[0x1A, 0x45, 0xDF, 0xA3];
const DOC_TYPE: &[u8] = &[0x42, 0x82];
const SEGMENT: &[u8] = &[0x18, 0x53, 0x80, 0x67];
const INFO: &[u8] = &[0x15, 0x49, 0xA9, 0x66];
const MUXING_APP: &[u8] = &[0x4D, 0x80];
const WRITING_APP: &[u8] = &[0x57, 0x41];
const TRACKS: &[u8] = &[0x16, 0x54, 0xAE, 0x6B];
const CHAPTERS: &[u8] = &[0x10, 0x43, 0xA7, 0x70];
const EDITION_ENTRY: &[u8] = &[0x45, 0xB9];
const EDITION_UID: &[u8] = &[0x45, 0xBC];
const CHAPTER_ATOM: &[u8] = &[0xB6];
const CHAPTER_UID: &[u8] = &[0x73, 0xC4];
const CHAPTER_TIME_START: &[u8] = &[0x91];

/// Encodes a size value as an 8-byte EBML VINT (unambiguous, always the same length regardless
/// of value, which keeps this test harness simple).
fn encode_size(n: u64) -> [u8; 8] {
    let mut b = [0u8; 8];
    b[0] = 0x01;
    let nb = n.to_be_bytes();
    b[1..8].copy_from_slice(&nb[1..8]);
    b
}

fn elem(id: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(id.len() + 8 + payload.len());
    v.extend_from_slice(id);
    v.extend_from_slice(&encode_size(payload.len() as u64));
    v.extend_from_slice(payload);
    v
}

/// Builds a single ChapterAtom element, nesting `n` further ChapterAtom children inside it.
///
/// Iterative (builds from the innermost element outward) so this test-harness helper itself
/// doesn't recurse `n` times in the process building the input - only the demuxer's own handling
/// of the resulting bytes is under test here. (An earlier, recursive version of this exact
/// helper function did stack-overflow its own builder process at n=60000 in a debug build - a
/// false alarm in the test harness, not in symphonia-format-mkv; see RESULT.md.)
fn nested_chapter_atom(n: u32, next_uid: &mut u64) -> Vec<u8> {
    let mut innermost = Vec::new();
    for depth in (0..=n).rev() {
        let uid = *next_uid;
        *next_uid += 1;

        let mut payload = Vec::new();
        payload.extend(elem(CHAPTER_UID, &uid.to_be_bytes()));
        payload.extend(elem(CHAPTER_TIME_START, &0u64.to_be_bytes()));
        if depth != n {
            payload.extend(innermost);
        }
        innermost = elem(CHAPTER_ATOM, &payload);
    }
    innermost
}

fn build_mkv_with_nested_chapters(nesting: u32) -> Vec<u8> {
    let mut out = Vec::new();

    // EBML header.
    out.extend(elem(EBML, &elem(DOC_TYPE, b"matroska")));

    // Segment.
    let mut segment_payload = Vec::new();

    // Info (mandatory MuxingApp/WritingApp).
    let mut info_payload = Vec::new();
    info_payload.extend(elem(MUXING_APP, b"x"));
    info_payload.extend(elem(WRITING_APP, b"x"));
    segment_payload.extend(elem(INFO, &info_payload));

    // Tracks (empty is valid).
    segment_payload.extend(elem(TRACKS, &[]));

    // Chapters -> EditionEntry -> deeply nested ChapterAtom chain.
    let mut uid_counter: u64 = 1;
    let mut edition_payload = Vec::new();
    edition_payload.extend(elem(EDITION_UID, &1u64.to_be_bytes()));
    edition_payload.extend(nested_chapter_atom(nesting, &mut uid_counter));

    let chapters_payload = elem(EDITION_ENTRY, &edition_payload);
    segment_payload.extend(elem(CHAPTERS, &chapters_payload));

    out.extend(elem(SEGMENT, &segment_payload));

    out
}

fn count_chapter_items(items: &[symphonia_core::meta::ChapterGroupItem]) -> usize {
    items
        .iter()
        .map(|item| match item {
            symphonia_core::meta::ChapterGroupItem::Group(g) => 1 + count_chapter_items(&g.items),
            symphonia_core::meta::ChapterGroupItem::Chapter(_) => 1,
        })
        .sum()
}

fn run_one(nesting: u32) {
    eprintln!("nesting={nesting}: building input...");
    let data = build_mkv_with_nested_chapters(nesting);
    eprintln!("nesting={nesting}: built {} bytes, calling MkvReader::try_new...", data.len());
    let mss =
        MediaSourceStream::new(Box::new(Cursor::new(data)), MediaSourceStreamOptions::default());
    match MkvReader::try_new(mss, FormatOptions::default()) {
        Ok(reader) => {
            let n_chapters = reader.chapters().map(|g| count_chapter_items(&g.items)).unwrap_or(0);
            println!(
                "nesting={nesting}: parsed OK (no crash), chapter tree item count={n_chapters}"
            );
        }
        Err(e) => {
            println!("nesting={nesting}: parsed with error (no crash): {e:?}");
        }
    }
}

fn main() {
    // A range of nesting depths around and well past the schema's MAX_DEPTH=16 cap.
    for nesting in [4u32, 10, 15, 16, 17, 20, 50, 500, 5000, 60000] {
        run_one(nesting);
    }
    println!("all runs completed without a crash");
}
