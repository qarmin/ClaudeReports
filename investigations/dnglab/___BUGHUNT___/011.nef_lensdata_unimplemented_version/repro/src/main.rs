// Reproduction for candidate 011: rawler/src/decoders/nef/lensdata.rs's `todo!()` on an
// unrecognized NEF LensData sub-format version (GitHub issue #583).
//
// This drives the exact same real, public entry point the `dnglab` binary itself uses for
// `dnglab convert`/`dnglab extract` on a NEF file: `rawler::get_decoder` followed by
// `Decoder::raw_metadata` (see bin/dnglab/dnglab-lib/src/jobs/raw2dng.rs and
// bin/dnglab/dnglab-lib/src/jobs/process_raw.rs, both call `decoder.raw_metadata(...)`
// unconditionally as part of every convert/analyze run).
//
// The input file (data/crafted.NEF) is a real, unmodified Nikon D2H NEF file (from this
// project's own rawdb test corpus) with exactly 4 bytes changed: the plaintext ASCII version
// prefix of the Makernote's LensData tag (0x0098), changed from "0101" (a real, implemented
// version) to "0900" (a version number no arm in `from_makernote`'s match recognizes). This
// mirrors the real bug report: a valid NEF file whose LensData sub-format version isn't one of
// the ones this decoder happens to implement. Every other byte, including the whole image data
// section, is untouched real camera output.

use rawler::decoders::RawDecodeParams;
use rawler::rawsource::RawSource;
use std::path::Path;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/crafted.NEF");
    let rawsource = RawSource::new(&path).expect("failed to open crafted.NEF");

    let decoder = rawler::get_decoder(&rawsource).expect("get_decoder failed on crafted.NEF");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        decoder.raw_metadata(&rawsource, &RawDecodeParams::default())
    }));

    match result {
        Ok(Ok(_metadata)) => {
            println!("did not reproduce - raw_metadata() completed normally");
        }
        Ok(Err(e)) => {
            println!("did not reproduce - raw_metadata() returned a graceful Err: {:?}", e);
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic_payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "<non-string panic payload>".to_string());
            eprintln!("REPRODUCED - decoder.raw_metadata() panicked: {}", msg);
            std::process::exit(1);
        }
    }
}
