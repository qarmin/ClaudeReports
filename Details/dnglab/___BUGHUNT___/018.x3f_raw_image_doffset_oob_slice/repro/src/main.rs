// Standalone reproduction: X3F decoder's raw_image() panics on an out-of-range
// caminfo.doffset+6 slice, reached only *after* X3fFile::new() has already
// successfully parsed a well-formed-enough directory (so the trivial
// truncated-file crash fixed by candidate 003 does not apply here - this is
// a distinct, one-layer-deeper bug).
//
// This crafts a genuinely malformed X3F file on disk (a well-formed directory
// with one "IMA2" entry whose declared fields are internally consistent, but
// whose data block ends 2 bytes before the file's actual end, so
// caminfo.doffset + 6 runs 2 bytes past the buffer) and hands it to
// `rawler::imgop::develop::process_raw_image` - the real, public, top-level
// function dnglab's own `Raw2Image` job (bin/dnglab/dnglab-lib/src/jobs/process_raw.rs)
// calls to convert a raw file. No internals of X3fDecoder are called directly,
// and no second participant needs to be faked - this is a single external
// artifact (a file) parsed by its real public entry point, per
// CONFIRMATION.md section 11.

use rawler::imgop::develop::{RawProcessingParams, process_raw_image};
use std::io::Cursor;

/// Builds a minimal-but-internally-consistent X3F file whose IMA2 image data
/// block ends exactly at the file's true end (before the 4-byte footer), so
/// `caminfo.doffset` (== end of that 28-byte block) is itself in range, but
/// `caminfo.doffset + 6` (used unchecked by `X3fDecoder::raw_image`) is not.
fn build_crash_x3f() -> Vec<u8> {
    let mut header = b"FOVb".to_vec();
    header.extend_from_slice(&[0u8; 4]); // 8 bytes total, padding unused by parser

    let dir_off = header.len() as u32;
    let mut dir_header = b"SECd".to_vec(); // marker bytes, unused by parser
    dir_header.extend_from_slice(&0x0002_0000u32.to_le_bytes()); // version
    dir_header.extend_from_slice(&1u32.to_le_bytes()); // entries = 1

    let dir_total_prefix = header.len() + dir_header.len() + 12; // + one 12-byte entry
    let img_off = dir_total_prefix as u32;

    // 28-byte X3fImage block: bytes [0..8) unused, [8..12)=typ, [12..16)=format,
    // [16..20)=width, [20..24)=height, [24..28)=pitch.
    let mut img_block = vec![0u8; 8];
    img_block.extend_from_slice(&2u32.to_le_bytes()); // typ = 2 (camera info)
    img_block.extend_from_slice(&0x12u32.to_le_bytes()); // format = 0x12
    img_block.extend_from_slice(&1u32.to_le_bytes()); // width
    img_block.extend_from_slice(&1u32.to_le_bytes()); // height
    img_block.extend_from_slice(&0u32.to_le_bytes()); // pitch
    assert_eq!(img_block.len(), 28);

    let mut entry0 = img_off.to_le_bytes().to_vec();
    entry0.extend_from_slice(&28u32.to_le_bytes()); // len
    entry0.extend_from_slice(b"IMA2"); // id

    let mut body = header;
    body.extend_from_slice(&dir_header);
    body.extend_from_slice(&entry0);
    body.extend_from_slice(&img_block);
    // caminfo.doffset (= img_off + 28) now equals body.len() exactly: in range
    // for X3fImage::new()'s own reads (which only need up to byte 28 of the
    // block), but buffer[caminfo.doffset + 6..] (used later, unchecked, in
    // raw_image()) will be 6 bytes past body.len(), and the 4-byte footer
    // only adds 4 more bytes - still 2 bytes short.

    let footer = dir_off.to_le_bytes();
    body.extend_from_slice(&footer);
    body
}

fn main() {
    let data = build_crash_x3f();
    let doffset = data.len() as u32 - 4 /*footer*/ ; // == img_off + 28, by construction
    println!(
        "crafted {}-byte X3F file; caminfo.doffset = {}, caminfo.doffset+6 = {} (> file len {})",
        data.len(),
        doffset,
        doffset + 6,
        data.len()
    );

    let out_path = std::env::temp_dir().join("bughunt_018_x3f_doffset_oob.x3f");
    std::fs::write(&out_path, &data).expect("write x3f file");
    println!("wrote crafted file to {:?}", out_path);

    let mut out = Cursor::new(Vec::<u8>::new());
    match process_raw_image(&out_path, &mut out, &RawProcessingParams::default()) {
        Ok(()) => println!("process_raw_image returned Ok (bug not present)"),
        Err(e) => println!("process_raw_image returned Err, no panic (bug not present): {e}"),
    }
}
