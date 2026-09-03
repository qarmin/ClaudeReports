use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::iff::aiff::AiffFile;
use std::io::Cursor;

fn build_aiff() -> Vec<u8> {
    let mut chunks = Vec::new();

    // valid "COMM" chunk (18 bytes, PCM, no compression): channels=2, sample_frames=0x10bc2,
    // sample_size=16, sample_rate=44100.0 as an 80-bit IEEE-754 extended float (bytes lifted
    // from the project's own tests/files/assets/minimal/full_test.aiff).
    chunks.extend_from_slice(b"COMM");
    chunks.extend_from_slice(&18u32.to_be_bytes());
    chunks.extend_from_slice(&[
        0x00, 0x02, 0x00, 0x01, 0x0b, 0xc2, 0x00, 0x10, 0x40, 0x0e, 0xbb, 0x80, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ]);

    // valid "SSND" chunk: 8-byte offset/blockSize header + 4 bytes of sample data
    chunks.extend_from_slice(b"SSND");
    chunks.extend_from_slice(&12u32.to_be_bytes());
    chunks.extend_from_slice(&[0u8; 8]);
    chunks.extend_from_slice(&[0u8; 4]);

    // trailing "ID3 " chunk: declares 8 bytes of content, but 0 bytes physically follow.
    // This is exactly the off-by-IFF_CHUNK_HEADER_SIZE boundary: at this point
    // remaining_size (pre-decrement) == 8 (just this header), so declared size == 8 slips
    // through the buggy `size > remaining_size` check, then id3_chunk()'s content() read
    // hits real physical EOF.
    chunks.extend_from_slice(b"ID3 ");
    chunks.extend_from_slice(&8u32.to_be_bytes());

    let mut data = Vec::new();
    data.extend_from_slice(b"FORM");
    data.extend_from_slice(&((4 + chunks.len()) as u32).to_be_bytes());
    data.extend_from_slice(b"AIFF");
    data.extend_from_slice(&chunks);

    data
}

fn main() {
    let data = build_aiff();
    println!("input file: {} bytes total", data.len());
    println!(
        "layout: FORM/AIFF header(12) + valid 'COMM' chunk(26) + valid 'SSND' chunk(20) + truncated 'ID3 ' chunk header only(8, declares 8 content bytes, 0 physically present)"
    );

    let mut cursor = Cursor::new(data);
    let result = AiffFile::read_from(&mut cursor, ParseOptions::new());

    match &result {
        Ok(f) => println!(
            "RESULT: Ok(AiffFile {{ properties: {:?} }}) -- BestAttempt gracefully degraded past the malformed trailing chunk",
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
