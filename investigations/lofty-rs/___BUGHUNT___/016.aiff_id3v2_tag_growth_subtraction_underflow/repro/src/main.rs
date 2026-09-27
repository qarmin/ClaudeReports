// Standalone repro for candidate 016: the AIFF (BigEndian) instantiation of the same
// `write_to_chunk_file` bug confirmed for WAV (LittleEndian) in candidate 015.
//
// Growing an existing, trailing ID3v2 chunk in an AIFF file via lofty's real public write API
// (`AiffFile::save_to`) underflows a `u64` subtraction in
// `lofty::id3::v2::write::chunk_file::write_to_chunk_file`, at:
//
//     updated_stream_len -= tag_chunk_size - existing_tag_len;
//
// This is the exact same shared, format-agnostic function candidate 015 found buggy for WAV -
// `lofty/src/id3/v2/write/mod.rs` dispatches to it with `BigEndian` for AIFF and `LittleEndian`
// for WAV, but the buggy line itself does not depend on endianness at all. This repro drives the
// real AIFF-specific entry point (`AiffFile::read_from` -> `set_id3v2` -> `save_to`) to confirm
// the bug independently reproduces through the AIFF path, not just the WAV one.
//
// This builds a minimal, real AIFF file from scratch (FORM/AIFF + COMM + SSND), writes a small
// ID3v2 tag to it (using lofty's real public API, exactly as a real caller would), then writes a
// much larger one - exercising the exact same real entry point (`AiffFile::save_to`) a genuine
// user calls.

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::iff::aiff::AiffFile;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};

/// Builds a minimal, valid, real AIFF file (FORM/AIFF + `COMM` + `SSND`) - just enough for
/// `AiffFile::read_from` to accept it.
fn minimal_aiff() -> Vec<u8> {
    // COMM: channels(u16 BE)=2, sample_frames(u32 BE)=16, sample_size(u16 BE)=16,
    // sample_rate as an 80-bit IEEE extended float (10 bytes BE) = 44100.0, the standard
    // well-known byte sequence for that value in AIFF COMM chunks.
    let mut comm_chunk = Vec::new();
    comm_chunk.extend_from_slice(&2u16.to_be_bytes()); // channels
    comm_chunk.extend_from_slice(&16u32.to_be_bytes()); // sample frames
    comm_chunk.extend_from_slice(&16u16.to_be_bytes()); // sample size
    comm_chunk.extend_from_slice(&[0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // 44100 Hz

    let mut ssnd_chunk = Vec::new();
    ssnd_chunk.extend_from_slice(&0u32.to_be_bytes()); // offset
    ssnd_chunk.extend_from_slice(&0u32.to_be_bytes()); // block size
    ssnd_chunk.extend_from_slice(&[0u8; 32]); // 16 silent 16-bit stereo samples

    let mut body = Vec::new();
    body.extend_from_slice(b"AIFF");
    body.extend_from_slice(b"COMM");
    body.extend_from_slice(&(comm_chunk.len() as u32).to_be_bytes());
    body.extend_from_slice(&comm_chunk);
    body.extend_from_slice(b"SSND");
    body.extend_from_slice(&(ssnd_chunk.len() as u32).to_be_bytes());
    body.extend_from_slice(&ssnd_chunk);

    let mut file = Vec::new();
    file.extend_from_slice(b"FORM");
    file.extend_from_slice(&(body.len() as u32).to_be_bytes());
    file.extend_from_slice(&body);
    file
}

fn main() {
    let aiff = minimal_aiff();
    println!("base AIFF file: {} bytes", aiff.len());

    let mut cursor = Cursor::new(aiff);

    // Step 1: write a small ID3v2 tag as the very last chunk in the file - the ordinary case of
    // tagging a previously-untagged AIFF file through the real public API.
    {
        let mut f = AiffFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
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
    // `AiffFile::save_to` entry point hits the underflowing subtraction.
    cursor.seek(SeekFrom::Start(0)).unwrap();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut f = AiffFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        id3v2.set_comment("X".repeat(200_000));
        f.set_id3v2(id3v2);

        f.save_to(&mut cursor, WriteOptions::default())
    }));

    match result {
        Ok(Ok(())) => {
            let len = cursor.get_ref().len();
            let declared = u32::from_be_bytes(cursor.get_ref()[4..8].try_into().unwrap());
            println!("after writing large tag:  {len} bytes");
            println!(
                "FORM header declares total size = {} bytes; actual physical file = {} bytes",
                declared as u64 + 8,
                len
            );
            if declared as u64 + 8 != len as u64 {
                eprintln!(
                    "REPRODUCED (silent corruption) - FORM header size does not match the \
                     actual file size"
                );
                std::process::exit(1);
            }
            println!("did not reproduce - FORM header size matches the real file size");
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
