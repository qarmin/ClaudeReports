//! The same end-to-end trigger as `repro`'s Part B, but with no `catch_unwind` at all, so the
//! panic aborts the process the way it would in a real application and `RUST_BACKTRACE=1`
//! produces a real stack trace through the actual call chain.
//!
//! Run with `RUST_BACKTRACE=1 cargo run --bin uncaught`.

use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

fn chunk(ctype: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 8 + payload.len());
    out.extend_from_slice(ctype);
    out.extend_from_slice(&(payload.len() as i64).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn build_caf() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"caff");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());

    let mut desc = Vec::new();
    desc.extend_from_slice(&44100.0f64.to_be_bytes());
    desc.extend_from_slice(b"ima4");
    desc.extend_from_slice(&0u32.to_be_bytes());
    desc.extend_from_slice(&34u32.to_be_bytes());
    desc.extend_from_slice(&64u32.to_be_bytes());
    desc.extend_from_slice(&1u32.to_be_bytes());
    desc.extend_from_slice(&16u32.to_be_bytes());
    out.extend_from_slice(&chunk(b"desc", &desc));

    let mut chan = Vec::new();
    chan.extend_from_slice(&0u32.to_be_bytes()); // mChannelLayoutTag = UseChannelDescriptions
    chan.extend_from_slice(&0u32.to_be_bytes()); // mChannelBitmap
    chan.extend_from_slice(&0u32.to_be_bytes()); // mNumberChannelDescriptions = 0
    out.extend_from_slice(&chunk(b"chan", &chan));

    let mut data_payload = Vec::new();
    data_payload.extend_from_slice(&0u32.to_be_bytes());
    data_payload.extend_from_slice(&[0u8; 34]);
    out.extend_from_slice(&chunk(b"data", &data_payload));

    out
}

fn main() {
    let data = build_caf();
    println!("crafted CAF file: {} bytes", data.len());

    let mss = MediaSourceStream::new(Box::new(std::io::Cursor::new(data)), Default::default());

    let format = symphonia::default::get_probe()
        .probe(&Hint::new(), mss, FormatOptions::default(), MetadataOptions::default())
        .expect("probe should recognize the crafted CAF file");

    let track = format.default_track(TrackType::Audio).expect("should have an audio track");
    let codec_params = track.codec_params.as_ref().unwrap().audio().unwrap();

    let _decoder = symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &Default::default())
        .expect("should find a registered ADPCM IMA QT decoder");

    println!("no panic");
}
