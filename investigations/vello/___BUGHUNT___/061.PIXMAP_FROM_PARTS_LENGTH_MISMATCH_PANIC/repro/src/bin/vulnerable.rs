// Demonstrates the bug: `Pixmap::from_parts` (vello_common's real, public constructor - the same
// entry point any application feeding an externally-decoded image buffer into vello_cpu would
// call directly) panics (hard process abort, not a `Result`/`Option`) when the caller-supplied
// `data: Vec<u8>` doesn't match `width * height * 4`, or when the vector's capacity isn't a
// multiple of 4. Both are realistic patterns for a `Vec<u8>` produced by an external image
// decoder whose reported dimensions came from somewhere other than the buffer's own actual
// length. This binary compiles and panics identically both before and after the fix in
// patch.diff - the fix does not (and by design should not) change `from_parts`'s own documented,
// backward-compatible panicking contract; see src/bin/safe.rs for the fix's actual deliverable
// (a fallible alternative entry point).

use vello_common::peniko::ImageAlphaType;
use vello_common::pixmap::{PixelMetadata, Pixmap};

fn run_case(name: &str, data: Vec<u8>, width: u16, height: u16) -> bool {
    let pixel_metadata = PixelMetadata::new(ImageAlphaType::AlphaPremultiplied, false);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Pixmap::from_parts(data, width, height, pixel_metadata)
    }));
    match result {
        Ok(_) => {
            println!("[{name}] did not panic");
            false
        }
        Err(payload) => {
            eprintln!("[{name}] REPRODUCED - panicked: {payload:?}");
            true
        }
    }
}

fn main() {
    let mut reproduced = false;

    // Case 1: a decoder reports width=4, height=4 (implying 64 bytes) but the actual decoded
    // buffer is only 60 bytes long (e.g. a truncated/off-by-one-row decode). Hits the
    // `assert_eq!` in `Pixmap::from_parts`.
    reproduced |= run_case("length_mismatch", vec![0_u8; 60], 4, 4);

    // Case 2: the vector's `len()` matches `width * height * 4` exactly, but its `capacity()` is
    // not a multiple of 4 (a realistic pattern for a `Vec` built incrementally, e.g. via
    // `Vec::with_capacity` sized in odd chunks by a decoder/re-encoder, then never
    // `shrink_to_fit`-ed). Hits the `bytemuck::try_cast_vec(...).expect(...)` in
    // `Pixmap::from_parts`.
    let mut odd_capacity_data: Vec<u8> = Vec::with_capacity(7);
    odd_capacity_data.extend_from_slice(&[1, 2, 3, 4]);
    reproduced |= run_case("odd_capacity", odd_capacity_data, 1, 1);

    if reproduced {
        std::process::exit(1);
    }
}
