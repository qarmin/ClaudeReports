// Standalone reproduction: `resize_fixed_point`/`resize_floating_point` (src/resize_fixed_point.rs,
// src/resize_floating_point.rs) validate the caller-supplied `src.len()` against
// `source_size.width * CHANNELS * source_size.height` using plain (non-overflow-checked) usize
// multiplication - right before code just below that goes to the trouble of using
// `overflowing_mul` for the exact same shape of computation to detect stride/size overflow.
//
// A crafted large width/height overflows that first plain multiplication and panics (debug build,
// overflow-checks on) before the overflow guards further down are ever reached, defeating their
// purpose entirely.
//
// This must be run as a DEBUG build (`cargo run`, NOT `--release`) to observe the panic - in
// release the same multiplication silently wraps, and the (accidental) result happens to still be
// rejected by the length-mismatch check or the later overflow guards for the specific inputs
// tried here, so the release build masks the bug rather than fixing it.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

fn main() {
    use pic_scale_safe::{resize_rgb_f32, resize_rgba8, ImageSize, ResamplingFunction};

    let mut reproduced = false;

    // Case 1: fixed-point path (resize_rgba8, CHANNELS=4). width << CHANNELS already overflows in
    // the first multiplication step (width*CHANNELS = 2^64, itself out of range).
    {
        let src: Vec<u8> = vec![0u8; 16];
        let source_size = ImageSize {
            width: 1usize << 62,
            height: 4,
        };
        let destination_size = ImageSize {
            width: 2,
            height: 2,
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            resize_rgba8(&src, source_size, destination_size, ResamplingFunction::Bilinear)
        }));
        match result {
            Err(_) => {
                println!("case1 (resize_rgba8, width=2^62,height=4): PANICKED (overflow in length check)");
                reproduced = true;
            }
            Ok(r) => println!("case1 (resize_rgba8, width=2^62,height=4): did not panic, got {r:?}"),
        }
    }

    // Case 2: floating-point path (resize_rgb_f32, CHANNELS=3), a different boundary - width*CHANNELS
    // alone fits, only the final *height multiplication overflows.
    {
        let src: Vec<f32> = vec![0.0f32; 12];
        let source_size = ImageSize {
            width: (1usize << 61) + 1,
            height: 3,
        };
        let destination_size = ImageSize {
            width: 2,
            height: 2,
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            resize_rgb_f32(&src, source_size, destination_size, ResamplingFunction::Bilinear)
        }));
        match result {
            Err(_) => {
                println!("case2 (resize_rgb_f32, width=2^61+1,height=3): PANICKED (overflow in length check)");
                reproduced = true;
            }
            Ok(r) => println!("case2 (resize_rgb_f32, width=2^61+1,height=3): did not panic, got {r:?}"),
        }
    }

    // Case 3: width = usize::MAX outright.
    {
        let src: Vec<u8> = vec![0u8; 16];
        let source_size = ImageSize {
            width: usize::MAX,
            height: 1,
        };
        let destination_size = ImageSize {
            width: 2,
            height: 2,
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            resize_rgba8(&src, source_size, destination_size, ResamplingFunction::Bilinear)
        }));
        match result {
            Err(_) => {
                println!("case3 (resize_rgba8, width=usize::MAX,height=1): PANICKED (overflow in length check)");
                reproduced = true;
            }
            Ok(r) => println!("case3 (resize_rgba8, width=usize::MAX,height=1): did not panic, got {r:?}"),
        }
    }

    // Sanity check: an ordinary well-formed input must still work and must be unaffected by any fix.
    {
        let src: Vec<u8> = vec![0u8; 16 * 16 * 4];
        let source_size = ImageSize {
            width: 16,
            height: 16,
        };
        let destination_size = ImageSize { width: 8, height: 8 };
        let result = resize_rgba8(&src, source_size, destination_size, ResamplingFunction::Bilinear);
        match result {
            Ok(v) if v.len() == 8 * 8 * 4 => println!("sanity check (16x16 -> 8x8 rgba8): ok"),
            other => {
                println!("sanity check (16x16 -> 8x8 rgba8): UNEXPECTED result {other:?}");
                reproduced = true;
            }
        }
    }

    if reproduced {
        eprintln!("REPRODUCED - plain-multiplication length check overflows/panics before the overflow guards below it are reached");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all boundary inputs returned a graceful Err instead of panicking, and the well-formed input still resizes correctly");
    }
}
