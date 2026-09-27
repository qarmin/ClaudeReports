// Standalone reproduction for GitHub issue #571: `range end index 24585 out of range for slice
// of length 24020` panic in rawler/src/bits.rs (LEu16), hit while decoding a Panasonic RW2 (v4
// compression) raw file.
//
// This runs the *exact* code from the GitHub issue's own report: `rawler::get_decoder` ->
// `Decoder::raw_metadata` -> `Decoder::raw_image` -> `RawDevelop::develop_intermediate`, fed the
// real crash file attached to the issue (testdata/sample1_IDX_1_RAND_173729052516569860293996.rw2,
// downloaded from the issue's own attachment - not hand-authored bytes). This is a single external
// artifact handed to its real public parsing entry point end-to-end - no second participant is
// being faked here, per CONFIRMATION.md section 11.
//
// Root cause: `decode_panasonic_v4` (rawler/src/decoders/rw2/v4decompressor.rs) computes a
// per-row byte offset (`skip`/`blocks`) into the RW2 entropy-coded strip data purely from
// `width`/`row`, with no relation to how much data the file actually contains. For a
// truncated/malformed RW2, this can run `BitPumpPanasonic` past the end of the (already too
// short) buffer slice, and `BitPumpPanasonic::peek_bits` then reads 2 bytes via
// `bits::LEu16(self.buffer, idx)` with no bounds check, panicking once `idx + 2 > buffer.len()`.
// RW2 rows are decoded in parallel via rayon (`decompress_strips_fn`); which specific line panics
// first (this one, or the sibling `&buf[blocks * 0x4000..]` slice at v4decompressor.rs:12) depends
// on which row's worker thread hits its own out-of-bounds read first, so `RAYON_NUM_THREADS=1` is
// forced below to deterministically reproduce the exact `bits.rs` panic location this candidate
// is about, rather than a racy mix of the two sibling panics.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use rawler::decoders::RawDecodeParams;
use rawler::imgop::develop::RawDevelop;
use rawler::rawsource::RawSource;

fn main() {
  // Force single-threaded rayon so the specific `bits.rs` LEu16 panic (as opposed to the sibling
  // v4decompressor.rs:12 slice panic - same root cause, different line, see comment above)
  // reproduces deterministically instead of depending on parallel thread-scheduling order.
  // Safety: this is the very first thing main() does, on the single, still-single-threaded main
  // thread, strictly before rayon's global pool (or anything else) reads the environment.
  unsafe {
    std::env::set_var("RAYON_NUM_THREADS", "1");
  }

  let path = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/sample1_IDX_1_RAND_173729052516569860293996.rw2");
  let content = std::fs::read(path).expect("read real crash-report RW2 file");
  println!("loaded {} bytes from {}", content.len(), path);

  let result = std::panic::catch_unwind(|| {
    let raw_source = RawSource::new_from_slice(&content);
    let decoder = rawler::get_decoder(&raw_source).expect("get_decoder");
    let _metadata = decoder.raw_metadata(&raw_source, &RawDecodeParams::default()).expect("raw_metadata");
    let raw_image = decoder.raw_image(&raw_source, &RawDecodeParams::default(), false).expect("raw_image");
    let developer = RawDevelop::default();
    developer.develop_intermediate(&raw_image).expect("develop_intermediate")
  });

  match result {
    Ok(_developed_image) => {
      println!("did not reproduce - decode completed without panicking");
    }
    Err(panic_payload) => {
      eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
      std::process::exit(1);
    }
  }
}
