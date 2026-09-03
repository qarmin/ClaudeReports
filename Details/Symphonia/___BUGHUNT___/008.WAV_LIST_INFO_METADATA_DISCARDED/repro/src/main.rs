//! Standalone reproduction for: `WavReader::try_new` parses a WAVE file's own `LIST`/`INFO`
//! chunk into a local `metadata: MetadataLog`, then discards it - the `WavReader` struct's
//! `metadata` field is instead initialized from `opts.external_data.metadata.unwrap_or_default()`,
//! so `FormatReader::metadata()` never surfaces tags actually embedded in the file itself unless
//! the caller separately supplies external metadata (which no real caller does for a plain
//! `open_file` style call).
//!
//! The WAV file used here is a plain, ordinary (if minimal) WAVE file: RIFF header, `fmt ` chunk
//! (PCM, 1 channel, 8-bit), a `LIST`/`INFO` chunk containing an `INAM` ("Title") tag placed
//! *before* the `data` chunk (the layout every real WAV encoder producing INFO tags uses), and a
//! `data` chunk with a few samples. This is the genuine external input a real caller would hand
//! to the crate's own public `WavReader::try_new` / `FormatReader::metadata()` API - not
//! internal state hand-constructed to bypass that API.

use std::io::Cursor;

use symphonia_core::formats::{FormatOptions, FormatReader};
use symphonia_core::io::MediaSourceStream;
use symphonia_format_riff::WavReader;

fn le_u32(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

fn build_wav_with_info_before_data() -> Vec<u8> {
    let mut body = Vec::new();
    // "WAVE" form.
    body.extend_from_slice(b"WAVE");

    // "fmt " chunk: PCM, 1 channel, 8000 Hz, 8-bit.
    let fmt_data: Vec<u8> = {
        let mut f = Vec::new();
        f.extend_from_slice(&1u16.to_le_bytes()); // format tag: PCM
        f.extend_from_slice(&1u16.to_le_bytes()); // channels
        f.extend_from_slice(&8000u32.to_le_bytes()); // sample rate
        f.extend_from_slice(&8000u32.to_le_bytes()); // byte rate
        f.extend_from_slice(&1u16.to_le_bytes()); // block align
        f.extend_from_slice(&8u16.to_le_bytes()); // bits per sample
        f
    };
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&le_u32(fmt_data.len() as u32));
    body.extend_from_slice(&fmt_data);

    // "LIST" chunk of form "INFO", containing one "INAM" sub-chunk ("Title"), placed *before*
    // the data chunk (the ordinary, real-world layout).
    let title = b"Hello Symphonia";
    let info_body: Vec<u8> = {
        let mut b = Vec::new();
        b.extend_from_slice(b"INFO");
        b.extend_from_slice(b"INAM");
        b.extend_from_slice(&le_u32(title.len() as u32));
        b.extend_from_slice(title);
        if title.len() % 2 == 1 {
            b.push(0); // RIFF sub-chunk padding to even length
        }
        b
    };
    body.extend_from_slice(b"LIST");
    body.extend_from_slice(&le_u32(info_body.len() as u32));
    body.extend_from_slice(&info_body);

    // "data" chunk: a handful of silent 8-bit samples.
    let samples = [128u8; 16];
    body.extend_from_slice(b"data");
    body.extend_from_slice(&le_u32(samples.len() as u32));
    body.extend_from_slice(&samples);

    // Top-level RIFF header.
    let mut file = Vec::new();
    file.extend_from_slice(b"RIFF");
    file.extend_from_slice(&le_u32(body.len() as u32));
    file.extend_from_slice(&body);
    file
}

fn main() {
    let wav_bytes = build_wav_with_info_before_data();

    let mss = MediaSourceStream::new(Box::new(Cursor::new(wav_bytes)), Default::default());

    let mut reader = WavReader::try_new(mss, FormatOptions::default())
        .expect("failed to construct WavReader from a plain, valid WAV file");

    let metadata = reader.metadata();
    let current = metadata.current();

    match current {
        Some(rev) if !rev.media.tags.is_empty() => {
            println!(
                "OK: metadata() returned {} tag(s), e.g. {:?}",
                rev.media.tags.len(),
                rev.media.tags[0]
            );
        }
        Some(_) => {
            println!(
                "BUG CONFIRMED: metadata().current() returned Some(..) but with ZERO tags, even \
                 though the WAV file's own LIST/INFO chunk contained an INAM (\"Hello Symphonia\") \
                 tag."
            );
            std::process::exit(1);
        }
        None => {
            println!(
                "BUG CONFIRMED: metadata().current() returned None entirely, even though the WAV \
                 file's own LIST/INFO chunk contained an INAM (\"Hello Symphonia\") tag."
            );
            std::process::exit(1);
        }
    }
}
