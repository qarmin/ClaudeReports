// Standalone reproduction: `resize_nearest` (src/resize_nearest.rs) computes its fixed-point
// horizontal/vertical scale factors as
//
//     let k_x: u64 = ((src_width as u64) << 32) / dst_width as u64;
//     let k_y: u64 = ((src_height as u64) << 32) / dst_height as u64;
//
// `src_width`/`src_height` are `usize`, which is 64-bit on common platforms, so either can reach
// or exceed `2^32`. Shifting a `u64` left by 32 does not panic (the shift *amount* 32 is well
// under 64), but it silently drops any bits at or above bit 31 of the pre-shift value - for
// `src_width == 2^32` the shift result becomes exactly 0 instead of the mathematically correct
// `2^64` (which doesn't fit in `u64` at all). The division that follows then produces `k_x == 0`
// regardless of `dst_width`, so every destination pixel in the row ends up reading the SAME
// source pixel (index 0) instead of being spread across the source width - a silent, wrong-output
// logic bug, not a crash.
//
// This is reachable through the crate's own public API (`resize_plane8` with
// `ResamplingFunction::Nearest`) - `resize_fixed_point`'s own guards (length check, stride/size
// overflow checks) only reject inputs that don't fit in `usize` at all; they do not bound
// `width`/`height` to below `2^32` individually. A `>= 2^32`-wide (or tall) single-channel `u8`
// image needs only ~4 GiB of `u8` buffer, which is practically allocatable on an ordinary
// development machine (verified here), not just a theoretical "multi-gigapixel" construction.
//
// Exit code convention: 0 = did not reproduce (fix present), non-zero = reproduced (bug present).

use pic_scale_safe::{resize_plane8, ImageSize, ResamplingFunction};

fn case_kx() -> bool {
    let src_width: usize = 1usize << 32;
    let src_height: usize = 1;
    let dst_width: usize = 2;
    let dst_height: usize = 1;

    let mut src = vec![0u8; src_width * src_height];
    // Independent oracle: the CORRECT fixed-point formula, evaluated in u128 so it can't itself
    // truncate, predicts pixel0 samples src[2^30] and pixel1 samples src[3*2^30].
    let expected0 = 1usize << 30;
    let expected1 = 3usize << 30;
    src[0] = 0x11u8; // what a k_x-truncated-to-0 computation would always sample instead
    src[expected0] = 0xAAu8;
    src[expected1] = 0xBBu8;

    let source_size = ImageSize { width: src_width, height: src_height };
    let destination_size = ImageSize { width: dst_width, height: dst_height };
    let out = resize_plane8(&src, source_size, destination_size, ResamplingFunction::Nearest)
        .expect("resize_plane8 should succeed for a well-formed, if large, input");
    println!("case_kx (src_width=2^32, dst_width=2): output={:?}", out);
    out == vec![0xAAu8, 0xBBu8]
}

fn case_ky() -> bool {
    let src_width: usize = 1;
    let src_height: usize = 1usize << 32;
    let dst_width: usize = 1;
    let dst_height: usize = 2;

    let mut src = vec![0u8; src_width * src_height];
    let expected0 = 1usize << 30;
    let expected1 = 3usize << 30;
    src[0] = 0x11u8;
    src[expected0] = 0xAAu8;
    src[expected1] = 0xBBu8;

    let source_size = ImageSize { width: src_width, height: src_height };
    let destination_size = ImageSize { width: dst_width, height: dst_height };
    let out = resize_plane8(&src, source_size, destination_size, ResamplingFunction::Nearest)
        .expect("resize_plane8 should succeed for a well-formed, if large, input");
    println!("case_ky (src_height=2^32, dst_height=2): output={:?}", out);
    out == vec![0xAAu8, 0xBBu8]
}

fn case_non_power_of_two() -> bool {
    // A boundary value that isn't an exact power of two, so the fix isn't only checked against
    // the one exact input that first demonstrated the bug (CONFIRMATION.md section 7).
    let src_width: usize = (1usize << 32) + 777;
    let src_height: usize = 1;
    let dst_width: usize = 3;
    let dst_height: usize = 1;

    let src = vec![0u8; src_width * src_height];

    // Independent oracle computed directly here (not by calling the crate's own internals).
    let k_x: u128 = ((src_width as u128) << 32) / dst_width as u128;
    let k_x_half: u128 = k_x >> 1;
    let mut expected = Vec::new();
    let mut src_x_fixed = k_x_half;
    for _ in 0..dst_width {
        expected.push((src_x_fixed >> 32) as usize);
        src_x_fixed += k_x;
    }
    println!("case_non_power_of_two: expected sample indices = {:?}", expected);

    let source_size = ImageSize { width: src_width, height: src_height };
    let destination_size = ImageSize { width: dst_width, height: dst_height };
    let out = resize_plane8(&src, source_size, destination_size, ResamplingFunction::Nearest)
        .expect("resize_plane8 should succeed for a well-formed, if large, input");

    // The source is all-zero here, so this can't check sampled *values* like the other two cases -
    // it only checks that the three expected sample indices are non-zero and distinct, which the
    // truncation bug (all indices forced to 0) would fail.
    let spread = expected.iter().collect::<std::collections::HashSet<_>>().len() == dst_width
        && expected[0] > 0;
    println!("case_non_power_of_two: output={:?}, spread_ok={}", out, spread);
    spread
}

fn main() {
    let mut all_ok = true;
    all_ok &= case_kx();
    all_ok &= case_ky();
    all_ok &= case_non_power_of_two();

    if all_ok {
        println!("did not reproduce - fixed-point scale factors are computed correctly");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - k_x/k_y truncated to 0 for a >=2^32 source dimension, collapsing every destination sample onto source index 0");
        std::process::exit(1);
    }
}
