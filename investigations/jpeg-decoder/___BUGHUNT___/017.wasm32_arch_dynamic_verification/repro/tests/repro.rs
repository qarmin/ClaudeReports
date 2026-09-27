// Standalone reproduction: on the wasm32-unknown-unknown target, `usize` is 32 bits. The real,
// public `jpeg_decoder::Decoder::decode()` entry point, given a legal-per-spec but very large SOF
// (up to 65535x65535, no scan data needed to trigger this - it happens at frame-parse time),
// computes `block_size.width * block_size.height * dct_scale * dct_scale` as a `usize` internal
// buffer length (src/worker/immediate.rs and src/worker/rayon.rs, both share this same
// unchecked expression). On wasm32 that computation overflows or exceeds Vec's own allocation
// ceiling before any real image data is even needed - on a 64-bit host (the environment this
// crate is normally tested and fuzzed on) the identical multiplication never overflows, so this
// is invisible to the project's existing native test suite and fuzz corpus.
//
// This drives the actual affected entry point (Decoder::decode()) with a real external JPEG file
// on the actual affected target (wasm32) under a real wasm runtime (wasm-bindgen-test-runner +
// Node, the same mechanism .github/workflows/rust.yml's cross_compile_wasm job uses) - not a
// hand-built stand-in for the decoder's internal state. No second participant is being
// impersonated: a JPEG file handed to a decoder's public decode entry point is the genuine
// external input a real caller/attacker would supply (CONFIRMATION.md section 11's exception for
// a single external artifact with no "other side").
//
// Only meaningful on wasm32; a no-op elsewhere.
#![cfg(target_arch = "wasm32")]

extern crate jpeg_decoder as jpeg;

macro_rules! overflow_case {
    ($name:ident, $file:expr) => {
        #[wasm_bindgen_test::wasm_bindgen_test]
        fn $name() {
            const JPG: &[u8] = include_bytes!(concat!("../inputs/", $file));
            let mut decoder = jpeg::Decoder::new(JPG);
            // Before the fix: this call panics (either "attempt to multiply with overflow" in a
            // debug build, or "capacity overflow" out of Vec::resize once the product exceeds
            // isize::MAX even without wrapping) instead of returning a Result at all - so merely
            // reaching this line without panicking/trapping already demonstrates the fix.
            // Whether decode() itself then reports Ok or Err is irrelevant to this bug (these
            // fixtures have no real scan data - a clean Err is the fully-fixed, expected outcome).
            let _ = decoder.decode();
        }
    };
}

// The original crashing fixture, verbatim from the project's own
// tests/crashtest/images/max_size.jpg (part of the existing malformed-input corpus the project's
// native crashtest() test already runs - just never previously run on wasm32 with real dynamic
// execution). 65535x65535, 1 component, default (unscaled) dct_scale=8: block_size resolves to
// 8192x8192, and 8192*8192*8*8 == 2^32 exactly, wrapping a 32-bit usize to 0.
overflow_case!(max_size, "max_size.jpg");

// Same file with only the width field patched from 65535 to 65529 (a byte-for-byte copy
// otherwise, including the same DHT/SOS/entropy bytes) - block_size becomes 8192x8192 still (both
// 65529 and 65535 divide down to the same 8192-block width), demonstrating the bug isn't a
// one-exact-file fluke: any width/height in [65529, 65535] with 1x1 sampling hits the identical
// wraparound.
overflow_case!(rect_variant, "overflow-variant-rect-65529x65535.jpg");

// Same again with width patched to 65524: block_size becomes 8191x8192, so
// 8191*8192*8*8 == 4294443008 - this fits in a 32-bit usize (no wraparound at all, checked_mul
// alone would say "fine") but still exceeds isize::MAX (2147483647), which is Vec<u8>'s actual
// allocation ceiling on a 32-bit target - `Vec::resize` panics with "capacity overflow" instead.
// This is a second, related-but-distinct overflow surface within the same bug class (see
// RESULT.md's "vary the input" note) - a fix that only guards against usize wraparound (checked_mul
// returning None) does not catch this one.
overflow_case!(capacity_variant, "overflow-variant-capacity-65535x65524.jpg");
