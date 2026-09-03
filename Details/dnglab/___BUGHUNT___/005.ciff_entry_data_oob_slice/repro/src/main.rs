// Standalone reproduction: CIFF (Canon CRW) entry parsing panics on an out-of-range data slice.
//
// Hand-builds a genuinely minimal-but-valid CIFF/CRW file: the real "HEAPCCDR" magic (so
// rawler::get_decoder's own real format-sniffing selects the real CrwDecoder, exactly like it
// would for a real .crw file), a root directory with one entry whose declared byte size/offset
// point past the end of the file - then opens it through rawler's real, public, top-level
// `rawler::get_decoder` entry point, the same first step every rawler-based tool takes. No
// internals of the suspect function are called directly, and no second participant needs to be
// faked - this is a single external artifact (a file) parsed by its real public entry point, per
// CONFIRMATION.md section 11.
//
// (This crashes one level deeper than the trivial 0-byte input cargo-fuzz's own fuzz_ciff_parse
// target found - that target calls the low-level `CiffIFD::new` directly, bypassing the magic-byte
// gate `is_ciff`/`get_decoder` enforce, so it can't demonstrate reachability through a real file.
// See RESULT.md for exactly how this layout was derived from `CiffIFD::new`/`CiffEntry::new`.)

use rawler::get_decoder;
use rawler::rawsource::RawSource;

fn le_u32(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}
fn le_u16(v: u16) -> [u8; 2] {
    v.to_le_bytes()
}

fn build_malicious_crw() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"II"); // byte-order marker (not validated by this parser)
    buf.extend_from_slice(&le_u32(14)); // offset (2..6): root IFD starts right after this header
    buf.extend_from_slice(b"HEAPCCDR"); // magic checked by is_ciff (bytes 6..14)

    // Root IFD at offset 14. valuedata_size will be read from the last 4 bytes of the whole
    // file (end - 4) - we make that 0, so the directory (dircount + entries) starts exactly
    // at `start` (14) with no value-data region in between.
    buf.extend_from_slice(&le_u16(1)); // dircount = 1 entry, at offset 14..16

    // The one entry, 10 bytes, at offset 16..26:
    //   p (u16): tag | datalocation | typ - datalocation = 0x0000 selects the
    //     "data is offset in value_data" branch in CiffEntry::new, which is the vulnerable one.
    buf.extend_from_slice(&le_u16(0x0000)); // offset 16..18
    buf.extend_from_slice(&le_u32(0xFFFF_FFFF)); // offset 18..22: bytesize - absurdly large
    buf.extend_from_slice(&le_u32(0)); // offset 22..26: data_offset (relative to value_data=14) = 0

    // Trailing 4 bytes: read as valuedata_size via LEu32(buf, end - 4). Must be 0 for the
    // layout above (entries starting exactly at `start`) to line up.
    buf.extend_from_slice(&le_u32(0)); // offset 26..30

    buf
}

fn main() {
    let data = build_malicious_crw();
    assert_eq!(data.len(), 30);

    let out_path = std::env::temp_dir().join("bughunt_005_ciff_malicious.crw");
    std::fs::write(&out_path, &data).expect("write crw file");
    println!("wrote {} bytes to {:?}", data.len(), out_path);

    let source = RawSource::new(&out_path).expect("open raw source");
    match get_decoder(&source) {
        Ok(_) => println!("get_decoder returned Ok (bug not present)"),
        Err(e) => println!("get_decoder returned an error, no panic (bug not present): {e}"),
    }
}
