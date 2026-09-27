use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::iff::wav::WavFile;
use std::io::Cursor;

fn build_wav() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"RIFF");
    data.extend_from_slice(&0u32.to_le_bytes()); // riff size, unchecked by verify_wav

    data.extend_from_slice(b"WAVE");

    // valid "fmt " chunk: 16 bytes of PCM format data
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes()); // format_tag = PCM
    data.extend_from_slice(&1u16.to_le_bytes()); // channels
    data.extend_from_slice(&44100u32.to_le_bytes()); // sample_rate
    data.extend_from_slice(&88200u32.to_le_bytes()); // byte_rate
    data.extend_from_slice(&2u16.to_le_bytes()); // block_align
    data.extend_from_slice(&16u16.to_le_bytes()); // bits_per_sample

    // valid "data" chunk with 4 real content bytes
    data.extend_from_slice(b"data");
    data.extend_from_slice(&4u32.to_le_bytes());
    data.extend_from_slice(&[0u8; 4]);

    // trailing "ID3 " chunk: declares 8 bytes of content, but 0 bytes physically follow.
    // This is exactly the off-by-IFF_CHUNK_HEADER_SIZE boundary: at this point
    // remaining_size (pre-decrement) == 8 (just this header), so declared size == 8 slips
    // through the buggy `size > remaining_size` check, then id3_chunk()'s content() read
    // hits real physical EOF.
    data.extend_from_slice(b"ID3 ");
    data.extend_from_slice(&8u32.to_le_bytes());

    data
}

fn main() {
    let data = build_wav();
    println!("input file: {} bytes total", data.len());
    println!(
        "layout: RIFF header(12) + valid 'fmt ' chunk(24) + valid 'data' chunk(12) + truncated 'ID3 ' chunk header only(8, declares 8 content bytes, 0 physically present)"
    );

    let mut cursor = Cursor::new(data);
    let result = WavFile::read_from(&mut cursor, ParseOptions::new());

    match &result {
        Ok(f) => println!(
            "RESULT: Ok(WavFile {{ properties: {:?} }}) -- BestAttempt gracefully degraded past the malformed trailing chunk",
            f.properties()
        ),
        Err(e) => println!(
            "RESULT: Err({e:?}) -- BestAttempt failed to gracefully degrade, hard I/O error propagated instead"
        ),
    }

    match result {
        Err(_) => std::process::exit(1),
        Ok(_) => std::process::exit(0),
    }
}
