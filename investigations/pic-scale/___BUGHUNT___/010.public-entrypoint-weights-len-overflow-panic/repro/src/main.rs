// Standalone reproduction, real public API, single participant (a caller passing raw
// width/height integers straight to `pic_scale::ImageSize`/`Scaler::plan_rgba_resampling` - no
// second participant to fake, this is the genuine external input itself).
//
// Root cause: `Scaler::plan_generic_resize` (src/plan/default_planner.rs) validates
// `destination_size` for isize/overflow safety via `check_image_size_overflow` before planning,
// but never validates `source_size` at all. `source_size.width`/`.height` (both caller-supplied
// raw `usize`, no upper bound) feed straight into `src/math/weights.rs::generate_weights` as
// `in_size`, which computes `scale = in_size / out_size` and derives `kernel_size` from it
// (saturating-cast from float, so it can reach close to `usize::MAX`). That `kernel_size` is then
// multiplied by `out_size` with a plain, unchecked `*` operator to size the weights `Vec` - which
// overflows `usize` for an `in_size`/`out_size` pair that's otherwise perfectly legal per every
// other check in the crate (comfortably within the `check_image_size_overflow` bound that *is*
// applied to `destination_size`).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (the crate
// panicked instead of returning `Result::Err`, as its own public API contract implies for
// resource/size-limit failures - see `PicScaleError::OutOfMemory`/`SourceImageIsTooLarge`, which
// exist precisely so callers get a `Result`, not a panic, for exactly this class of bad input).

use pic_scale::{ImageSize, ResamplingFunction, Scaler};

fn main() {
    let scaler = Scaler::new(ResamplingFunction::Lanczos3);

    // A source width chosen so that `scale = in_size / out_size` (out_size = 2) drives
    // `kernel_size` (derived from `round(6.0 * scale)`, saturating-cast to `usize`) high enough
    // that `kernel_size * out_size` overflows `usize` in the crate's own unchecked multiplication.
    let source_size = ImageSize::new(1usize << 62, 1);
    let destination_size = ImageSize::new(2, 1);

    println!(
        "calling Scaler::plan_rgba_resampling(source={}x{}, destination={}x{}) via the real public API",
        source_size.width, source_size.height, destination_size.width, destination_size.height
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scaler.plan_rgba_resampling(source_size, destination_size, false)
    }));

    match result {
        Ok(Ok(_plan)) => {
            println!("did not reproduce - plan built successfully (unexpected but not a bug)");
        }
        Ok(Err(e)) => {
            println!("did not reproduce - suspect code path correctly returned Err({e:?})");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked instead of returning Result::Err: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
