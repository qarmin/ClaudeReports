// Scratch investigation for candidate 020: checking the JUNK-padding write path's `remainder`
// arithmetic in lofty/src/id3/v2/write/chunk_file.rs's `write_to_chunk_file`
// (~lines 135-144), independent of the odd-length-chunk padding-byte ambiguity originally named
// in the IDEAS.md entry. Hypothesis: `remainder = existing_tag_len - tag_chunk_size` is used
// directly as the JUNK chunk's declared body-size field, but the JUNK chunk's on-disk footprint
// is `IFF_CHUNK_HEADER_SIZE (8) + body_size`, so the declared body size should be
// `remainder - IFF_CHUNK_HEADER_SIZE`, not `remainder` - an 8-byte overcount that would make the
// JUNK chunk declare itself extending 8 bytes past the true end of the old tag's region, eating
// into whatever real chunk data physically follows.
//
// Verification strategy: build a WAV file with an ID3v2 tag that is NOT the last chunk (append a
// real, distinguishable trailing chunk after it), shrink the tag via a real `save_to` call with
// `preferred_padding` set (forcing the JUNK path, not the end-of-file fast path already fixed in
// 015/016), then RE-PARSE the resulting file with the real `WavFile::read_from` entry point and
// check whether the trailing chunk is still intact.

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::iff::wav::WavFile;
use lofty::tag::{Accessor, TagExt};

use std::io::{Cursor, Seek, SeekFrom};

fn append_trailing_chunk_wav(bytes: &mut Vec<u8>, fourcc: &[u8; 4], body: &[u8]) {
    bytes.extend_from_slice(fourcc);
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(body);
    if !body.len().is_multiple_of(2) {
        bytes.push(0);
    }

    let new_riff_size = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&new_riff_size.to_le_bytes());
}

fn base_wav() -> Vec<u8> {
    std::fs::read("/home/rafal/Projekty/Obce/lofty-rs/lofty/tests/taglib/data/empty.wav").unwrap()
}

/// Find a chunk with the given fourcc anywhere in a RIFF/WAV byte buffer by raw scanning
/// (independent implementation, deliberately NOT reusing lofty's own chunk reader, so this is a
/// real oracle rather than testing the code against itself).
fn find_chunk_raw(bytes: &[u8], fourcc: &[u8; 4]) -> Option<(usize, u32)> {
    let mut pos = 12usize; // past "RIFF" + size + "WAVE"
    while pos + 8 <= bytes.len() {
        let cc = &bytes[pos..pos + 4];
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap());
        if cc == fourcc {
            return Some((pos, size));
        }
        let mut advance = 8usize + size as usize;
        if !size.is_multiple_of(2) {
            advance += 1;
        }
        pos += advance;
    }
    None
}

fn main() {
    let trailing_body: Vec<u8> = (0u8..=250).collect();

    let base = base_wav();
    let mut cursor = Cursor::new(base);

    let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();

    // Large initial ID3v2 tag so there's plenty of room to shrink later.
    let mut id3v2 = Id3v2Tag::default();
    id3v2.set_comment("Y".repeat(2000));
    f.set_id3v2(id3v2);
    f.save_to(&mut cursor, WriteOptions::new().preferred_padding(0))
        .unwrap();

    let mut bytes = cursor.into_inner();
    // Real, distinguishable trailing chunk after the ID3 tag, so the ID3 tag is no longer last.
    append_trailing_chunk_wav(&mut bytes, b"TEST", &trailing_body);

    let (raw_pos_before, raw_size_before) = find_chunk_raw(&bytes, b"TEST")
        .expect("TEST chunk must be present in the hand-built file before any write");
    println!(
        "before write: TEST chunk at pos={raw_pos_before}, size={raw_size_before}, file_len={}",
        bytes.len()
    );
    assert_eq!(raw_size_before as usize, trailing_body.len());
    assert_eq!(
        &bytes[raw_pos_before + 8..raw_pos_before + 8 + trailing_body.len()],
        &trailing_body[..]
    );

    // Now shrink the ID3v2 tag via the real public API, forcing the JUNK path (WAV,
    // preferred_padding set, existing tag large enough to hold new tag + an 8-byte JUNK header).
    let mut cursor2 = Cursor::new(bytes);
    let mut f2 = WavFile::read_from(&mut cursor2, ParseOptions::new()).unwrap();
    cursor2.seek(SeekFrom::Start(0)).unwrap();

    let mut small_id3v2 = Id3v2Tag::default();
    small_id3v2.set_title(String::from("T"));
    f2.set_id3v2(small_id3v2);
    f2.save_to(&mut cursor2, WriteOptions::new().preferred_padding(8))
        .unwrap();

    let after_bytes = cursor2.into_inner();

    match find_chunk_raw(&after_bytes, b"TEST") {
        Some((raw_pos_after, raw_size_after)) => {
            println!(
                "after write:  TEST chunk at pos={raw_pos_after}, size={raw_size_after}, file_len={}",
                after_bytes.len()
            );
            if raw_size_after as usize != trailing_body.len() {
                println!(
                    "MISMATCH: TEST chunk size changed from {} to {}",
                    trailing_body.len(),
                    raw_size_after
                );
            }
            let content_ok = after_bytes.len() >= raw_pos_after + 8 + trailing_body.len()
                && &after_bytes[raw_pos_after + 8..raw_pos_after + 8 + trailing_body.len()]
                    == &trailing_body[..];
            println!("TEST chunk content intact (raw scan): {content_ok}");
        },
        None => {
            println!("CORRUPTION: TEST chunk NOT FOUND by raw scan after write at all!");
        },
    }

    // Now the real oracle: can lofty itself still parse the file back correctly via its own
    // public read API, and does it still see the TEST-chunk-adjacent structure sanely (i.e. does
    // reading not error out / does the file round-trip)?
    let mut reread_cursor = Cursor::new(after_bytes.clone());
    match WavFile::read_from(&mut reread_cursor, ParseOptions::new()) {
        Ok(reparsed) => {
            println!(
                "re-parse via WavFile::read_from: OK, id3v2 present: {}",
                reparsed.id3v2().is_some()
            );
        },
        Err(e) => {
            println!("re-parse via WavFile::read_from: ERROR: {e}");
        },
    }

    // Dump JUNK chunk's own declared size vs. actual expected value, and cross-check against
    // IFF_CHUNK_HEADER_SIZE off-by-8 hypothesis directly.
    if let Some((junk_pos, junk_size)) = find_chunk_raw(&after_bytes, b"JUNK") {
        println!("JUNK chunk found at pos={junk_pos}, declared size={junk_size}");
        let junk_footprint_end = junk_pos + 8 + junk_size as usize;
        println!(
            "JUNK footprint end = {junk_footprint_end}, actual TEST-or-file-end boundary: file_len={}",
            after_bytes.len()
        );
    } else {
        println!("No JUNK chunk found after write (unexpected if JUNK path was taken)");
    }
}
