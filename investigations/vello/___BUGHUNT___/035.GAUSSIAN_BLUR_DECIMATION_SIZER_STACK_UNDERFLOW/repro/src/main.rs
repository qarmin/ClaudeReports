//! Standalone reproduction for candidate 035.
//!
//! Calls only `vello_common`'s real, public `DecimationSizer` API (the actual entry point any
//! downstream crate depending on `vello_common` would use) - no internal/private items, no
//! hand-faked state standing in for a caller. There is no second participant to fake here: the
//! "external input" this bug is about is simply a caller invoking a public method in an order the
//! type system does not prevent, and this repro does exactly that, directly.

use vello_common::filter::gaussian_blur::DecimationSizer;

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_else(|| "no-downscale".to_string());

    match variant.as_str() {
        // Variant 1: call upscale() before any matching downscale() at all.
        "no-downscale" => {
            let mut sizer = DecimationSizer::new(10, 10);
            println!("Calling upscale() with no prior downscale()...");
            let (w, h) = sizer.upscale();
            println!("upscale() unexpectedly returned ({w}, {h}) without panicking");
        }
        // Variant 2: one balanced downscale/upscale pair, then one extra upscale() beyond it -
        // same underlying stack-underflow bug, different call sequence (CONFIRMATION.md section 7).
        "extra-upscale" => {
            let mut sizer = DecimationSizer::new(7, 7);
            let _ = sizer.downscale();
            let _ = sizer.upscale();
            println!("Calling one more upscale() beyond the single matching downscale()...");
            let (w, h) = sizer.upscale();
            println!("upscale() unexpectedly returned ({w}, {h}) without panicking");
        }
        other => {
            eprintln!("unknown variant: {other}");
            std::process::exit(2);
        }
    }
}
