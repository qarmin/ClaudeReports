// Standalone repro for candidate 020: `write_to_chunk_file`'s WAV `JUNK`-padding path
// (lofty/src/id3/v2/write/chunk_file.rs, ~lines 133-145) declares the replacement `JUNK` chunk's
// size `IFF_CHUNK_HEADER_SIZE` (8) bytes too large. The freed-up space after shrinking an existing
// ID3v2 tag (`remainder = existing_tag_len - tag_chunk_size`) includes the `JUNK` chunk's own
// 8-byte header, but the code writes `remainder` itself as the `JUNK` chunk's declared
// (content-only) body size instead of `remainder - IFF_CHUNK_HEADER_SIZE`. This makes the `JUNK`
// chunk's declared on-disk footprint extend 8 bytes past the true end of the old tag's region,
// silently corrupting/hiding whatever real chunk data physically follows it in the file.
//
// This drives the real, public `WavFile::read_from` / `set_id3v2` / `save_to` entry points on a
// real file byte buffer - no internal/private API, no hand-constructed `Chunks`/`chunk_file`
// state. It is a single-participant repro (a WAV file handed to lofty's own real public read/
// write API is the entire external input - see CONFIRMATION.md section 11), so no second
// participant is required or faked.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (the real trailing chunk
// gets corrupted/lost by the JUNK chunk overclaiming 8 bytes it shouldn't).

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::iff::wav::WavFile;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};

/// Builds a minimal but valid WAV file (RIFF/WAVE + a `fmt ` chunk) with no other content, purely
/// in code so this repro has no external file dependency at all.
fn minimal_wav() -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");

    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&16u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes()); // PCM
    body.extend_from_slice(&2u16.to_le_bytes()); // channels
    body.extend_from_slice(&44100u32.to_le_bytes()); // sample rate
    body.extend_from_slice(&176400u32.to_le_bytes()); // byte rate
    body.extend_from_slice(&4u16.to_le_bytes()); // block align
    body.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    body.extend_from_slice(b"data");
    body.extend_from_slice(&16u32.to_le_bytes());
    body.extend_from_slice(&[0u8; 16]);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&body);
    bytes
}

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

/// Independent raw-byte chunk scanner (deliberately NOT reusing lofty's own chunk reader), so
/// this is a real oracle rather than testing the code against itself.
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

    let mut cursor = Cursor::new(minimal_wav());
    let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();

    // Large initial ID3v2 tag so there's plenty of room to shrink later. `preferred_padding(0)`
    // here just lands the tag at end-of-file via the already-fixed (015/016) fast path - this
    // write isn't the one under test.
    let mut id3v2 = Id3v2Tag::default();
    id3v2.set_comment("Y".repeat(2000));
    f.set_id3v2(id3v2);
    f.save_to(&mut cursor, WriteOptions::new().preferred_padding(0))
        .unwrap();

    let mut bytes = cursor.into_inner();
    // Real, distinguishable trailing chunk placed after the ID3 tag, so the tag is no longer the
    // last chunk in the file - this forces every subsequent write through the `JUNK`-padding path
    // under test (or the full-rewrite fallback), never the end-of-file fast path.
    append_trailing_chunk_wav(&mut bytes, b"TEST", &trailing_body);

    let (pos_before, size_before) =
        find_chunk_raw(&bytes, b"TEST").expect("TEST chunk must be present before any write");
    assert_eq!(size_before as usize, trailing_body.len());

    // Shrink the ID3v2 tag via the real public API with `preferred_padding` set, forcing the
    // `JUNK`-padding path (existing tag large enough to hold the new, smaller tag plus an 8-byte
    // `JUNK` header).
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
        Some((pos_after, size_after)) if pos_after == pos_before && size_after == size_before => {
            let content_ok = after_bytes.len() >= pos_after + 8 + trailing_body.len()
                && after_bytes[pos_after + 8..pos_after + 8 + trailing_body.len()]
                    == trailing_body[..];
            if content_ok {
                println!(
                    "did not reproduce - TEST chunk intact at pos={pos_after}, size={size_after}"
                );
                std::process::exit(0);
            }
            eprintln!(
                "REPRODUCED - TEST chunk header intact but content corrupted (pos={pos_after}, size={size_after})"
            );
            std::process::exit(1);
        },
        Some((pos_after, size_after)) => {
            eprintln!(
                "REPRODUCED - TEST chunk moved/resized: before pos={pos_before} size={size_before}, after pos={pos_after} size={size_after}"
            );
            std::process::exit(1);
        },
        None => {
            eprintln!(
                "REPRODUCED - TEST chunk NOT FOUND after write (consumed by an oversized JUNK chunk)"
            );
            if let Some((junk_pos, junk_size)) = find_chunk_raw(&after_bytes, b"JUNK") {
                eprintln!(
                    "  JUNK chunk at pos={junk_pos}, declared size={junk_size}, footprint end={} (real TEST chunk started at {pos_before})",
                    junk_pos + 8 + junk_size as usize
                );
            }
            std::process::exit(1);
        },
    }
}
