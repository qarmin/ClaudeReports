use std::io::Cursor;
use symphonia_core::io::{MediaSourceStream, MediaSourceStreamOptions};
use symphonia_core::formats::FormatOptions;

fn le32(v: u32) -> [u8;4] { v.to_le_bytes() }

fn build_wav(num_channels: u16, bits_per_sample: u16) -> Vec<u8> {
    let mut fmt = Vec::new();
    fmt.extend_from_slice(&1u16.to_le_bytes());           // WAVE_FORMAT_PCM
    fmt.extend_from_slice(&num_channels.to_le_bytes());
    fmt.extend_from_slice(&44100u32.to_le_bytes());
    fmt.extend_from_slice(&176400u32.to_le_bytes());
    fmt.extend_from_slice(&4u16.to_le_bytes());           // block align
    fmt.extend_from_slice(&bits_per_sample.to_le_bytes());
    assert_eq!(fmt.len(), 16);

    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&le32(fmt.len() as u32));
    body.extend_from_slice(&fmt);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&le32(8));
    body.extend_from_slice(&[0u8; 8]);

    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&le32(body.len() as u32));
    out.extend_from_slice(&body);
    out
}

fn be32(v: u32) -> [u8;4] { v.to_be_bytes() }

fn build_aiff(num_channels: u16, sample_size: u16) -> Vec<u8> {
    // 80-bit IEEE extended for 44100.0
    let rate: [u8; 10] = [0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut comm = Vec::new();
    comm.extend_from_slice(&num_channels.to_be_bytes());
    comm.extend_from_slice(&be32(4));                     // num sample frames
    comm.extend_from_slice(&sample_size.to_be_bytes());
    comm.extend_from_slice(&rate);
    assert_eq!(comm.len(), 18);

    let mut body = Vec::new();
    body.extend_from_slice(b"AIFF");
    body.extend_from_slice(b"COMM");
    body.extend_from_slice(&be32(comm.len() as u32));
    body.extend_from_slice(&comm);
    body.extend_from_slice(b"SSND");
    body.extend_from_slice(&be32(8 + 8));
    body.extend_from_slice(&be32(0)); // offset
    body.extend_from_slice(&be32(0)); // block size
    body.extend_from_slice(&[0u8; 8]);

    let mut out = Vec::new();
    out.extend_from_slice(b"FORM");
    out.extend_from_slice(&be32(body.len() as u32));
    out.extend_from_slice(&body);
    out
}

fn mss(data: Vec<u8>) -> MediaSourceStream<'static> {
    MediaSourceStream::new(Box::new(Cursor::new(data)), MediaSourceStreamOptions::default())
}

fn main() {
    let which = std::env::args().nth(1).unwrap_or_default();
    match which.as_str() {
        "wav" => {
            let data = build_wav(16384, 32);
            println!("wav: calling WavReader::try_new with num_channels=16384 bits_per_sample=32");
            let r = symphonia_format_riff::WavReader::try_new(mss(data), FormatOptions::default());
            println!("wav result is_err={}", r.is_err());
        }
        "aiff" => {
            let data = build_aiff(16384, 32);
            println!("aiff: calling AiffReader::try_new with num_channels=16384 sample_size=32");
            let r = symphonia_format_riff::AiffReader::try_new(mss(data), FormatOptions::default());
            println!("aiff result is_err={}", r.is_err());
        }
        _ => println!("usage: repro wav|aiff"),
    }
}
