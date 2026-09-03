// Standalone repro for candidate 015: growing an existing, trailing ID3v2 chunk in a WAV/AIFF
// file via lofty's real public write API (`WavFile::save_to`) underflows a `u64` subtraction in
// `lofty::id3::v2::write::chunk_file::write_to_chunk_file`, at:
//
//     updated_stream_len -= tag_chunk_size - existing_tag_len;
//
// This line is reached whenever an existing ID3v2 chunk is the very last chunk in the file (the
// common case) and the newly-written tag is larger than the one it replaces - a completely
// ordinary "add more metadata to an already-tagged file" operation. In a debug/overflow-checks
// build this panics ("attempt to subtract with overflow"); in a release build it silently wraps,
// corrupting the RIFF header's declared file size.
//
// This builds a minimal, real WAV file from scratch, writes a small ID3v2 tag to it (using
// lofty's real public API, exactly as a real caller would), then writes a much larger one -
// exercising the exact same real entry point (`WavFile::save_to`) a genuine user calls.

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::iff::wav::WavFile;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};

/// Builds a minimal, valid, real WAV file (RIFF/WAVE + `fmt ` + `data`) with a handful of silent
/// PCM samples - just enough for `WavFile::read_from` to accept it.
fn minimal_wav() -> Vec<u8> {
    let mut data_chunk = vec![0u8; 32]; // 16 silent 16-bit stereo samples
    if data_chunk.len() % 2 != 0 {
        data_chunk.push(0);
    }

    let mut fmt_chunk = Vec::new();
    fmt_chunk.extend_from_slice(&1u16.to_le_bytes()); // PCM
    fmt_chunk.extend_from_slice(&2u16.to_le_bytes()); // channels
    fmt_chunk.extend_from_slice(&44100u32.to_le_bytes()); // sample rate
    fmt_chunk.extend_from_slice(&(44100 * 2 * 2u32).to_le_bytes()); // byte rate
    fmt_chunk.extend_from_slice(&4u16.to_le_bytes()); // block align
    fmt_chunk.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&(fmt_chunk.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt_chunk);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&(data_chunk.len() as u32).to_le_bytes());
    body.extend_from_slice(&data_chunk);

    let mut file = Vec::new();
    file.extend_from_slice(b"RIFF");
    file.extend_from_slice(&(body.len() as u32).to_le_bytes());
    file.extend_from_slice(&body);
    file
}

fn main() {
    let wav = minimal_wav();
    println!("base WAV file: {} bytes", wav.len());

    let mut cursor = Cursor::new(wav);

    // Step 1: write a small ID3v2 tag as the very last chunk in the file - the ordinary case of
    // tagging a previously-untagged WAV file through the real public API.
    {
        let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        id3v2.set_title(String::from("T"));
        f.set_id3v2(id3v2);
        f.save_to(&mut cursor, WriteOptions::default()).unwrap();
    }
    println!(
        "after writing small tag:  {} bytes",
        cursor.get_ref().len()
    );

    // Step 2: write a MUCH larger ID3v2 tag over the existing (small, trailing) one - the
    // ordinary case of adding more metadata to an already-tagged file. This is where the real
    // `WavFile::save_to` entry point hits the underflowing subtraction.
    cursor.seek(SeekFrom::Start(0)).unwrap();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        id3v2.set_comment("X".repeat(200_000));
        f.set_id3v2(id3v2);

        f.save_to(&mut cursor, WriteOptions::default())
    }));

    match result {
        Ok(Ok(())) => {
            let len = cursor.get_ref().len();
            let declared =
                u32::from_le_bytes(cursor.get_ref()[4..8].try_into().unwrap());
            println!("after writing large tag:  {len} bytes");
            println!(
                "RIFF header declares total size = {} bytes; actual physical file = {} bytes",
                declared as u64 + 8,
                len
            );
            if declared as u64 + 8 != len as u64 {
                eprintln!(
                    "REPRODUCED (silent corruption) - RIFF header size does not match the \
                     actual file size"
                );
                std::process::exit(1);
            }
            println!("did not reproduce - RIFF header size matches the real file size");
        },
        Ok(Err(e)) => {
            println!("write returned an error (not the bug under test): {e}");
        },
        Err(payload) => {
            eprintln!("REPRODUCED - panicked: {payload:?}");
            std::process::exit(1);
        },
    }
}
