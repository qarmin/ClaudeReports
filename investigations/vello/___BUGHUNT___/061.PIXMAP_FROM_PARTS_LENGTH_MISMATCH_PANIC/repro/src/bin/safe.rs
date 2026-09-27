// Demonstrates the fix: `Pixmap::try_from_parts` (the new fallible counterpart added by
// patch.diff) handles the exact same malformed inputs as `vulnerable.rs` gracefully, returning
// `None` instead of panicking - giving a caller integrating an external/untrusted image decoder
// a way to handle a dimension/buffer mismatch without a hard process abort. This binary only
// compiles against a patched tree (`try_from_parts` does not exist before the fix), so it is only
// run in the justfile's `reproduce-after` step.

use vello_common::peniko::ImageAlphaType;
use vello_common::pixmap::{PixelMetadata, Pixmap};

fn run_case(name: &str, data: Vec<u8>, width: u16, height: u16) -> bool {
    let pixel_metadata = PixelMetadata::new(ImageAlphaType::AlphaPremultiplied, false);
    match Pixmap::try_from_parts(data, width, height, pixel_metadata) {
        Some(_) => {
            eprintln!("[{name}] UNEXPECTED - try_from_parts returned Some for malformed input");
            false
        }
        None => {
            println!("[{name}] handled gracefully - try_from_parts returned None");
            true
        }
    }
}

fn main() {
    let mut all_handled = true;

    all_handled &= run_case("length_mismatch", vec![0_u8; 60], 4, 4);

    let mut odd_capacity_data: Vec<u8> = Vec::with_capacity(7);
    odd_capacity_data.extend_from_slice(&[1, 2, 3, 4]);
    all_handled &= run_case("odd_capacity", odd_capacity_data, 1, 1);

    // Sanity check: well-formed input still succeeds through the fallible API.
    let good = vec![0_u8; 64];
    let pixel_metadata = PixelMetadata::new(ImageAlphaType::AlphaPremultiplied, false);
    let ok = Pixmap::try_from_parts(good, 4, 4, pixel_metadata).is_some();
    println!("[well_formed] try_from_parts returned Some: {ok}");

    if !all_handled || !ok {
        std::process::exit(1);
    }
}
