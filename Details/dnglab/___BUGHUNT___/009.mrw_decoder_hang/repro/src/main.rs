// Repro for candidate 009 (mrw_decoder_hang).
//
// Demonstrates that the fuzzer-found artifact does NOT hang when driven through rawler's real
// public API (rawler::decode_dummy, the exact function the fuzz_decode_metadata harness itself
// calls). It panics internally on an out-of-bounds slice index in MrwDecoder::new_mrw
// (rawler/src/bits.rs, called from rawler/src/decoders/mrw.rs), but that panic is caught by
// RawLoader::decode()'s own panic::catch_unwind wrapper and turned into an Err - in well under a
// second, every time.
//
// The reported "hang within timeout 8s" only reproduces inside the separate ASAN+libFuzzer
// -instrumented fuzz_decode_metadata binary (see RESULT.md and output.txt for that evidence,
// captured directly from cargo-fuzz - not reproduced by this crate, since that binary is fuzzing
// infrastructure, not part of rawler's own public API).

use rawler::rawsource::RawSource;
use std::time::Instant;

// The exact 10-byte crash artifact:
// rawler/fuzz/artifacts/fuzz_decode_metadata/crash-bc64cdd6e3d69fd6fa76d067616b6ecba4707c64
const CRASH_ARTIFACT: &[u8] = &[0x00, 0x4d, 0x52, 0x4d, 0x3b, 0x3b, 0x3b, 0xc0, 0x0a, 0xbf];

fn main() {
    let source = RawSource::new_from_slice(CRASH_ARTIFACT);
    let start = Instant::now();
    let result = rawler::decode_dummy(&source);
    let elapsed = start.elapsed();

    println!("decode_dummy() returned {:?} in {:?}", result.is_ok(), elapsed);
    assert!(elapsed.as_secs() < 8, "hung for >= 8s - this would be the claimed bug reproducing");
    println!("PASS: no hang - the real public API returns well within the 8s window the fuzzer used");
}
