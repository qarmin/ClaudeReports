// Standalone reproduction: `generate_weights` (src/compute_weights.rs) computes each destination
// pixel's source-window `FilterBounds { start, size }` using the crate's generic filter-weight
// type `T` (concretely `f32` for every real public entry point). The window's clamps
// (`.min(in_size.as_())`, both for `center_x`/`end`) convert the integer `in_size` (source
// width/height) to `T` first, compare in float space, then cast the clamped float value back to
// `usize`.
//
// `usize -> f32 -> usize` is lossy for any value above `2^24` (16,777,216) - f32 only has a
// 24-bit mantissa. For an `in_size` whose nearest representable f32 rounds UP (about half of all
// values above 2^24 do, e.g. 16_777_219 -> 16_777_220.0), the clamp bound used inside
// `generate_weights` is that rounded-UP value, not the true `in_size`. This lets the computed
// `end` (and therefore `bounds.size = end - start`) exceed the true, integer `in_size` by however
// much the rounding overshot (1 in the case exercised here).
//
// Downstream, `fixed_point_horizontal.rs`/`floating_point_horizontal.rs` slice the source data as
// `&src[px..(px + bounds_size * CHANNELS)]` using that inflated `bounds_size`. Since the row's
// actual data is only `in_size * CHANNELS` elements long, this slice's end index lands one past
// the end of the row - a slice-index-out-of-bounds panic, at the exact site originally suspected
// (candidate BOUNDS_SIZE_CHANNELS_OVERFLOW) for a *different* reason: not because
// `bounds_size * CHANNELS` overflows `usize` arithmetically (it doesn't - the value involved is
// only ~16.7 million), but because `bounds_size` itself is already wrong (too large) by the time
// it reaches that multiplication.
//
// Reachable through the crate's public API with a perfectly ordinary ~16 MB single-row u8 image -
// no multi-gigabyte allocation needed, unlike several of this session's other size-overflow
// candidates.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use pic_scale_safe::{resize_plane8, resize_plane_f32, ImageSize, ResamplingFunction};

fn main() {
    let mut reproduced = false;

    // Case 1: fixed-point path (resize_plane8 -> resize_fixed_point -> convolve_row_fixed_point
    // -> convolve_row_handler_fixed_point, src/fixed_point_horizontal.rs).
    {
        let width = 16_777_219usize; // (width as f32) as usize == 16_777_220 (rounds UP by 1)
        let src = vec![0u8; width];
        let src_size = ImageSize::new(width, 1);
        let dst_size = ImageSize::new(1, 1);
        let result = std::panic::catch_unwind(|| {
            resize_plane8(&src, src_size, dst_size, ResamplingFunction::Bilinear)
        });
        match result {
            Err(_) => {
                println!("case1 (resize_plane8, width=16_777_219 -> 1): PANICKED (slice OOB)");
                reproduced = true;
            }
            Ok(v) => println!("case1 (resize_plane8, width=16_777_219 -> 1): did not panic, {v:?}"),
        }
    }

    // Case 2: floating-point path (resize_plane_f32 -> resize_floating_point ->
    // convolve_row_fixed_point-equivalent -> convolve_row_handler_floating_point,
    // src/floating_point_horizontal.rs) - same generate_weights root cause, separate call site.
    {
        let width = 16_777_219usize;
        let src = vec![0f32; width];
        let src_size = ImageSize::new(width, 1);
        let dst_size = ImageSize::new(1, 1);
        let result = std::panic::catch_unwind(|| {
            resize_plane_f32(&src, src_size, dst_size, ResamplingFunction::Bilinear)
        });
        match result {
            Err(_) => {
                println!("case2 (resize_plane_f32, width=16_777_219 -> 1): PANICKED (slice OOB)");
                reproduced = true;
            }
            Ok(v) => println!("case2 (resize_plane_f32, width=16_777_219 -> 1): did not panic, {v:?}"),
        }
    }

    // Sanity check: a width just below the 2^24 precision boundary (exact usize<->f32 round-trip)
    // must still resize correctly and must be unaffected by any fix.
    {
        let width = 1_000_000usize;
        let src = vec![7u8; width];
        let src_size = ImageSize::new(width, 1);
        let dst_size = ImageSize::new(10, 1);
        match resize_plane8(&src, src_size, dst_size, ResamplingFunction::Bilinear) {
            Ok(v) if v.len() == 10 => {
                println!("sanity (resize_plane8, width=1_000_000 -> 10): ok, got 10 output pixels")
            }
            other => {
                println!("sanity (resize_plane8, width=1_000_000 -> 10): UNEXPECTED {other:?}");
                reproduced = true;
            }
        }
    }

    if reproduced {
        eprintln!(
            "REPRODUCED - generate_weights' float-domain in_size clamp lets FilterBounds.size exceed the true source width for widths > 2^24, causing an out-of-bounds slice panic in fixed_point_horizontal.rs/floating_point_horizontal.rs"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - both >2^24-width cases resized without panicking, and the sanity case still produced the correct output length");
    }
}
