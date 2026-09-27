// Standalone reproduction: `image::imageops::unsharpen()` (the exact public entry point named
// in the candidate) panics with an out-of-range slice index when given a zero-height image and
// a sigma large enough to push the internally-derived Gaussian kernel past the "arena" cutoff
// (`RING_QUEUE_CIRCULAR_CUTOFF == 55` in `src/imageops/filter_1d.rs`), which routes the call
// into `filter_2d_separable`'s arena path and from there into `make_columns_arenas`.
//
// Driven entirely through the crate's public API (`ImageBuffer::new` + `imageops::unsharpen`) -
// a caller building a 0-height image (e.g. from an empty crop, a zero-height decoded region, or
// simply a caller-supplied size) and blurring it with a large-radius blur is a completely
// ordinary, non-malicious use of this API, no file/codec parsing involved.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::{GrayImage, ImageBuffer};

fn run_case(width: u32, height: u32, sigma: f32, threshold: i32) -> Result<(u32, u32), String> {
    let img: GrayImage = ImageBuffer::new(width, height);
    let result = std::panic::catch_unwind(|| image::imageops::unsharpen(&img, sigma, threshold));
    match result {
        Ok(out) => Ok(out.dimensions()),
        Err(panic_payload) => Err(format!("{panic_payload:?}")),
    }
}

fn main() {
    // The exact case from the candidate: a zero-height image (not zero-width - see RESULT.md for
    // why the originally-suspected zero-width path is actually blocked by a different, already
    // documented bug, candidate 001) run through the named entry point `unsharpen()`.
    let primary = run_case(5, 0, 30.0, 0);

    match primary {
        Ok((w, h)) => {
            println!("did not reproduce - unsharpen returned a ({w}, {h}) image without panicking");
        }
        Err(msg) => {
            eprintln!("REPRODUCED - unsharpen(5x0 image, sigma=30.0, threshold=0) panicked: {msg}");
            std::process::exit(1);
        }
    }
}
