// Standalone reproduction: BMFF vendor-box parsing panics on offset+size integer overflow.
//
// This is the exact 54-byte input cargo-fuzz's own fuzz_bmff_parse target found (crash artifact
// `crash-66d729698c6852e22325a4d22c672046c97cfd97` in rawler/fuzz/artifacts/), written to a real
// file on disk and opened through rawler's real, public, top-level entry point
// `rawler::get_decoder` - the same first step every rawler-based tool takes when opening any raw
// file. No internals of the suspect function are called directly, and no second participant needs
// to be faked - single external artifact, real public entry point, per CONFIRMATION.md section 11.
//
// Unlike candidates 003/005 (X3F/CIFF, gated behind a fixed magic-byte check), this one needs no
// magic bytes at all: `get_decoder`'s own fallback chain (decoders/mod.rs) calls
// `Bmff::new(&mut rawfile.reader())` unconditionally on any file it hasn't already recognized as
// another format, and `Bmff::new` calls the exact `FileBox::parse` this bug lives in - so this
// crash is reachable from *any* unrecognized file handed to `rawler::get_decoder`, not just files
// that look like a specific raw format.

use rawler::get_decoder;
use rawler::rawsource::RawSource;

fn main() {
    // Exact fuzzer-found crashing input (54 bytes).
    let data: &[u8] = &[
        0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
        0xf3, 0xf3, 0x14, 0x9d, 0x9d, 0xff,
    ];

    let out_path = std::env::temp_dir().join("bughunt_006_bmff_overflow.bin");
    std::fs::write(&out_path, data).expect("write file");
    println!("wrote {} bytes to {:?}", data.len(), out_path);

    let source = RawSource::new(&out_path).expect("open raw source");
    match get_decoder(&source) {
        Ok(_) => println!("get_decoder returned Ok (bug not present)"),
        Err(e) => println!("get_decoder returned an error, no panic (bug not present): {e}"),
    }
}
