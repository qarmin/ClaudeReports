// Standalone reproduction for the unbounded-work hang in hayro-jbig2's `Image::decode`.
//
// `hang_input.jb2` and `hang_input_symbol_dict.jb2` are real, standalone JBIG2 embedded-stream
// files (not PDFs) - taken directly from `hayro-fuzz/artifacts/fuzz_jbig2/`, a stale local
// timeout corpus that still hangs the fuzz target against current HEAD. Both are fed straight to
// hayro-jbig2's own public `Image::new`/`decode` entry point, exactly the same entry point a real
// caller (e.g. hayro's PDF JBIG2Decode filter, or any other consumer of this crate) uses for a
// malformed/adversarial JBIG2 stream. No second participant is faked here: a malformed file
// handed to a parser's own public decode entry point is the genuine external input, there is no
// "peer" being impersonated (see CONFIRMATION.md section 11).
//
// `hang_input.jb2` declares a page of just 1x1 pixels, but its one ImmediateLosslessGenericRegion
// segment declares its own region_info width/height as 34304x49920 (~1.7 billion pixels) -
// entirely independent of, and unconstrained by, the page's own tiny declared size. That region
// is fully allocated and walked through the arithmetic decoder before ever being combined with
// (and mostly discarded against) the 1x1 page.
//
// `hang_input_symbol_dict.jb2` triggers the same missing-budget class a different way: its single
// SymbolDictionary segment (22 bytes) declares 59881 new symbols; the per-symbol width field is
// delta-encoded and never reset within a height class, so it grows roughly linearly across
// thousands of symbols, making the *cumulative* pixel count decoded across the whole dictionary
// grow quadratically with the symbol count, even though each individual symbol bitmap stays
// within any single-object dimension cap.
//
// This binary treats "took more than 3 seconds" as a stand-in for "hangs" (a real invocation from
// this project's stale corpus ran well past 15s uninterrupted before this fix) - inherent to
// reproducing a hang without literally blocking forever.
//
// Exit code convention: 0 = did not reproduce (both inputs decode quickly), non-zero = reproduced
// (at least one input took unreasonably long).

use std::time::{Duration, Instant};

const HANG_THRESHOLD: Duration = Duration::from_secs(3);

struct NullDecoder;

impl hayro_jbig2::Decoder for NullDecoder {
    fn push_pixel(&mut self, _black: bool) {}
    fn push_pixel_chunk(&mut self, _black: bool, _chunk_count: u32) {}
    fn next_line(&mut self) {}
}

fn run_one(name: &str, data: &[u8]) -> Duration {
    let t0 = Instant::now();

    if let Ok(image) = hayro_jbig2::Image::new(data) {
        let mut decoder = NullDecoder;
        let _ = image.decode(&mut decoder);
    }

    let elapsed = t0.elapsed();
    println!("{name}: decode finished in {elapsed:?}");
    elapsed
}

fn main() {
    let hang_input: &[u8] = include_bytes!("../hang_input.jb2");
    let hang_input_symbol_dict: &[u8] = include_bytes!("../hang_input_symbol_dict.jb2");

    let elapsed_generic_region = run_one("hang_input.jb2 (generic region)", hang_input);
    let elapsed_symbol_dict = run_one(
        "hang_input_symbol_dict.jb2 (symbol dictionary)",
        hang_input_symbol_dict,
    );

    let reproduced = elapsed_generic_region > HANG_THRESHOLD || elapsed_symbol_dict > HANG_THRESHOLD;

    if reproduced {
        eprintln!(
            "REPRODUCED - at least one decode took longer than {HANG_THRESHOLD:?} (generic_region={elapsed_generic_region:?}, symbol_dict={elapsed_symbol_dict:?})"
        );
        std::process::exit(1);
    } else {
        println!(
            "did not reproduce - both decodes completed under {HANG_THRESHOLD:?} (generic_region={elapsed_generic_region:?}, symbol_dict={elapsed_symbol_dict:?})"
        );
    }
}
