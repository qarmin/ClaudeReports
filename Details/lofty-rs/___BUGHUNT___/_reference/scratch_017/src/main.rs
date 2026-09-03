// Scratch investigation for candidate 017: boundary-value sweep of the WAV `JUNK`-padding
// path and the shared full-file-rewrite fallback in `write_to_chunk_file`
// (lofty/src/id3/v2/write/chunk_file.rs, ~lines 116-186), which were only manually traced (not
// fuzzed/boundary-tested) when candidates 015/016 fixed the sibling "overwrite tag at end of
// file" fast-path wrong-sign subtraction.
//
// Strategy: build a real WAV/AIFF file, tag it once via the real public API (lands the ID3 chunk
// as the file's *last* chunk, taking the already-covered fast path), then append one more real,
// valid trailing chunk by hand so the ID3 chunk is no longer last. This forces every subsequent
// `save_to` call through either the JUNK-padding path (WAV, `preferred_padding` set) or the
// full-file-rewrite fallback (WAV with `preferred_padding` unset, or AIFF always) - the two sites
// under investigation - via the real `WavFile`/`AiffFile::save_to` entry point.

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::iff::aiff::AiffFile;
use lofty::iff::wav::WavFile;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};

fn append_trailing_chunk_wav(bytes: &mut Vec<u8>, fourcc: &[u8; 4], body_len: usize) {
    let mut body = vec![0u8; body_len];
    // keep it a valid-looking payload, doesn't matter what's in it
    for (i, b) in body.iter_mut().enumerate() {
        *b = (i % 251) as u8;
    }
    bytes.extend_from_slice(fourcc);
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&body);
    if !body.len().is_multiple_of(2) {
        bytes.push(0);
    }

    let new_riff_size = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&new_riff_size.to_le_bytes());
}

fn append_trailing_chunk_aiff(bytes: &mut Vec<u8>, fourcc: &[u8; 4], body_len: usize) {
    let mut body = vec![0u8; body_len];
    for (i, b) in body.iter_mut().enumerate() {
        *b = (i % 251) as u8;
    }
    bytes.extend_from_slice(fourcc);
    bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&body);
    if !body.len().is_multiple_of(2) {
        bytes.push(0);
    }

    let new_form_size = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&new_form_size.to_be_bytes());
}

/// Hand-crafts a WAV file with a bare, zero-length `ID3 ` chunk (header only, no body - the
/// tightest possible `existing_tag_len == IFF_CHUNK_HEADER_SIZE` boundary) as a non-terminal
/// chunk, followed by a real trailing chunk. This is raw on-disk bytes fed straight to the real
/// `WavFile::read_from` entry point, exactly how a real caller would encounter a
/// maximally-adversarial file - not internal state handed directly to `write_to_chunk_file`.
fn wav_with_zero_length_id3_chunk() -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");

    // fmt chunk: minimal 16-byte PCM format
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&16u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes()); // PCM
    body.extend_from_slice(&2u16.to_le_bytes()); // channels
    body.extend_from_slice(&44100u32.to_le_bytes()); // sample rate
    body.extend_from_slice(&176400u32.to_le_bytes()); // byte rate
    body.extend_from_slice(&4u16.to_le_bytes()); // block align
    body.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // Bare, zero-length ID3 chunk: header only, no body at all.
    body.extend_from_slice(b"ID3 ");
    body.extend_from_slice(&0u32.to_le_bytes());

    // data chunk after it, so the ID3 chunk is not the last chunk.
    body.extend_from_slice(b"data");
    body.extend_from_slice(&16u32.to_le_bytes());
    body.extend_from_slice(&[0u8; 16]);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&body);
    bytes
}

fn base_wav() -> Vec<u8> {
    std::fs::read("/home/rafal/Projekty/Obce/lofty-rs/lofty/tests/taglib/data/empty.wav").unwrap()
}

fn base_aiff() -> Vec<u8> {
    std::fs::read("/home/rafal/Projekty/Obce/lofty-rs/lofty/tests/taglib/data/empty.aiff").unwrap()
}

/// Builds a WAV file with a small ID3v2 tag that is *not* the last chunk (a real trailing `JUNK`
/// chunk follows it), so `write_to_chunk_file`'s fast path (already covered by 015/016) is never
/// taken again on subsequent writes.
fn wav_with_non_terminal_id3(initial_comment_len: usize) -> Vec<u8> {
    let base = base_wav();
    let mut cursor = Cursor::new(base);

    let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();

    let mut id3v2 = Id3v2Tag::default();
    if initial_comment_len > 0 {
        id3v2.set_comment("Y".repeat(initial_comment_len));
    } else {
        id3v2.set_title(String::from("T"));
    }
    f.set_id3v2(id3v2);
    f.save_to(&mut cursor, WriteOptions::new().preferred_padding(0))
        .unwrap();

    let mut bytes = cursor.into_inner();
    append_trailing_chunk_wav(&mut bytes, b"JUNK", 16);
    bytes
}

fn aiff_with_non_terminal_id3(initial_comment_len: usize) -> Vec<u8> {
    let base = base_aiff();
    let mut cursor = Cursor::new(base);

    let mut f = AiffFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();

    let mut id3v2 = Id3v2Tag::default();
    if initial_comment_len > 0 {
        id3v2.set_comment("Y".repeat(initial_comment_len));
    } else {
        id3v2.set_title(String::from("T"));
    }
    f.set_id3v2(id3v2);
    f.save_to(&mut cursor, WriteOptions::new().preferred_padding(0))
        .unwrap();

    let mut bytes = cursor.into_inner();
    append_trailing_chunk_aiff(&mut bytes, b"ANNO", 16);
    bytes
}

fn try_write_wav(base: &[u8], comment_len: usize, preferred_padding: u32) -> Result<(), String> {
    let mut cursor = Cursor::new(base.to_vec());
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        if comment_len > 0 {
            id3v2.set_comment("X".repeat(comment_len));
        } else {
            id3v2.set_title(String::from("T"));
        }
        f.set_id3v2(id3v2);
        f.save_to(
            &mut cursor,
            WriteOptions::new().preferred_padding(preferred_padding),
        )
    }));

    match result {
        Ok(Ok(())) => {
            let bytes = cursor.into_inner();
            let declared = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            if declared as u64 + 8 != bytes.len() as u64 {
                return Err(format!(
                    "silent corruption: RIFF declares {} bytes, actual file is {} bytes",
                    declared as u64 + 8,
                    bytes.len()
                ));
            }
            Ok(())
        },
        Ok(Err(e)) => Err(format!("returned error (not the bug under test): {e}")),
        Err(payload) => Err(format!("PANIC: {payload:?}")),
    }
}

fn try_write_aiff(base: &[u8], comment_len: usize) -> Result<(), String> {
    let mut cursor = Cursor::new(base.to_vec());
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut f = AiffFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        if comment_len > 0 {
            id3v2.set_comment("X".repeat(comment_len));
        } else {
            id3v2.set_title(String::from("T"));
        }
        f.set_id3v2(id3v2);
        f.save_to(&mut cursor, WriteOptions::new().preferred_padding(0))
    }));

    match result {
        Ok(Ok(())) => {
            let bytes = cursor.into_inner();
            let declared = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
            if declared as u64 + 8 != bytes.len() as u64 {
                return Err(format!(
                    "silent corruption: FORM declares {} bytes, actual file is {} bytes",
                    declared as u64 + 8,
                    bytes.len()
                ));
            }
            Ok(())
        },
        Ok(Err(e)) => Err(format!("returned error (not the bug under test): {e}")),
        Err(payload) => Err(format!("PANIC: {payload:?}")),
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let mut failures = Vec::new();
    let mut ok_count = 0usize;
    let mut junk_path_hits = 0usize;
    let mut rewrite_path_hits = 0usize;

    // Sweep `existing_tag_len` itself (via the *initial* tag's size) across several magnitudes,
    // then for each one sweep the *new* tag's size on both sides of the
    // `existing_tag_len - IFF_CHUNK_HEADER_SIZE` boundary - this is the only way to actually
    // exercise the "large enough to overwrite in place" JUNK-padding branch at all, since it
    // requires the new tag to be smaller than the existing one by at least 8 bytes.
    for initial_comment_len in [0usize, 50, 500, 2000, 8000] {
        let wav_base = wav_with_non_terminal_id3(initial_comment_len);

        for comment_len in (0..=4000).step_by(1) {
            match try_write_wav(&wav_base, comment_len, 8) {
                Ok(()) => ok_count += 1,
                Err(e) => failures.push(format!(
                    "WAV JUNK-path initial_comment_len={initial_comment_len} preferred_padding=8, comment_len={comment_len}: {e}"
                )),
            }
        }

        for comment_len in (0..=4000).step_by(1) {
            match try_write_wav(&wav_base, comment_len, 0) {
                Ok(()) => ok_count += 1,
                Err(e) => failures.push(format!(
                    "WAV full-rewrite initial_comment_len={initial_comment_len} preferred_padding=None, comment_len={comment_len}: {e}"
                )),
            }
        }

        let aiff_base = aiff_with_non_terminal_id3(initial_comment_len);
        for comment_len in (0..=4000).step_by(1) {
            match try_write_aiff(&aiff_base, comment_len) {
                Ok(()) => ok_count += 1,
                Err(e) => failures.push(format!(
                    "AIFF full-rewrite initial_comment_len={initial_comment_len}, comment_len={comment_len}: {e}"
                )),
            }
        }
    }

    // Tightest possible boundary: existing_tag_len == IFF_CHUNK_HEADER_SIZE (8) exactly, via a
    // hand-crafted, maximally-adversarial on-disk WAV file (not lofty-produced state).
    let zero_len_base = wav_with_zero_length_id3_chunk();
    for comment_len in (0..=200).step_by(1) {
        match try_write_wav(&zero_len_base, comment_len, 8) {
            Ok(()) => ok_count += 1,
            Err(e) => failures.push(format!(
                "WAV zero-length-ID3-chunk preferred_padding=8, comment_len={comment_len}: {e}"
            )),
        }
        match try_write_wav(&zero_len_base, comment_len, 0) {
            Ok(()) => ok_count += 1,
            Err(e) => failures.push(format!(
                "WAV zero-length-ID3-chunk preferred_padding=None, comment_len={comment_len}: {e}"
            )),
        }
    }

    println!("ok: {ok_count}, failures: {}", failures.len());
    for f in failures.iter().take(50) {
        println!("FAIL: {f}");
    }
    println!("(junk_path_hits/rewrite_path_hits counters unused - see stderr log grep instead)");
    let _ = (junk_path_hits, rewrite_path_hits);

    if !failures.is_empty() {
        std::process::exit(1);
    }
}
