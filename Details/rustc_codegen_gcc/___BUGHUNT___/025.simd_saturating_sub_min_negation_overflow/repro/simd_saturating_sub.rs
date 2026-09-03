// Compiler:
//
// Run-time:
//   status: 0

// Repro for candidate 025 (simd_saturating_sub_min_negation_overflow), found while looking for
// more instances of this workspace's dominant "GCC-declared-signedness-vs-Rust-semantic-
// signedness" bug family, but landing on an adjacent one instead: an overflow-in-negation bug
// in the *signed saturating subtract* lowering for `core::simd` (`src/intrinsic/simd.rs`,
// `simd_saturating_sub`'s `(signed=true, is_add=false)` branch).
//
// That branch computes `lhs.saturating_sub(rhs)` by negating `rhs` and reusing the (correct)
// saturating-add overflow-detection algorithm on `lhs + (-rhs)`. When `rhs` is the element type's
// minimum value (e.g. `i8::MIN`), negating it overflows and two's-complement-wraps back to the
// same minimum value instead of its true (unrepresentable) mathematical negation - silently
// corrupting the sum the overflow check is based on, so the saturating-add algorithm faithfully
// detects overflow of the WRONG addition and returns a wrong, non-saturated value instead of the
// correct saturated one.
//
// This is exactly the same negation-of-MIN root cause already suspected (but not yet confirmed
// against this project's own Rust-level code, only demonstrated for the x86-specific
// `_mm_subs_epi8`/`_mm_subs_epi16` builtin path) in this workspace's `IDEAS.md` - this candidate
// confirms the same bug class lives independently in the generic, target-independent
// `core::simd` intrinsic lowering too, not just the x86 SSE2 builtin dispatch.

#![feature(portable_simd)]

use std::hint::black_box;
use std::simd::prelude::{SimdInt, SimdUint};
use std::simd::{i8x4, i16x4, u8x4};

fn main() {
    // The headline case: subtracting the minimum value from values that should saturate to MAX.
    let lhs = black_box(i8x4::from_array([127, 0, -128, -1]));
    let rhs = black_box(i8x4::from_array([-128, -128, -128, 1]));
    let got = lhs.saturating_sub(rhs).to_array();
    let expected = [
        127i8.saturating_sub(-128),
        0i8.saturating_sub(-128),
        (-128i8).saturating_sub(-128),
        (-1i8).saturating_sub(1),
    ];
    assert_eq!(got, expected, "i8 saturating_sub mismatch (rhs includes MIN)");

    // Ordinary (non-MIN-operand) cases, to confirm the fix does not regress the already-correct
    // path (this exact shape, minus the MIN operand, worked before the fix too).
    let lhs2 = black_box(i8x4::from_array([10, -10, 5, -5]));
    let rhs2 = black_box(i8x4::from_array([3, -3, -3, 3]));
    let got2 = lhs2.saturating_sub(rhs2).to_array();
    let expected2 = [
        10i8.saturating_sub(3),
        (-10i8).saturating_sub(-3),
        5i8.saturating_sub(-3),
        (-5i8).saturating_sub(3),
    ];
    assert_eq!(got2, expected2, "i8 saturating_sub mismatch (no MIN operand)");

    // Signed saturating_add is unaffected (different code path, not exercised by this bug) -
    // checked here as a sanity control, including its own MIN-operand boundary.
    let got_add = lhs.saturating_add(rhs).to_array();
    let expected_add = [
        127i8.saturating_add(-128),
        0i8.saturating_add(-128),
        (-128i8).saturating_add(-128),
        (-1i8).saturating_add(1),
    ];
    assert_eq!(got_add, expected_add, "i8 saturating_add mismatch");

    // Wider element width (i16), to confirm the bug (and the fix) aren't width-specific.
    let lhs16 = black_box(i16x4::from_array([i16::MAX, 0, i16::MIN, -1]));
    let rhs16 = black_box(i16x4::from_array([i16::MIN, i16::MIN, 1, i16::MIN]));
    let got16 = lhs16.saturating_sub(rhs16).to_array();
    let expected16 = [
        i16::MAX.saturating_sub(i16::MIN),
        0i16.saturating_sub(i16::MIN),
        i16::MIN.saturating_sub(1),
        (-1i16).saturating_sub(i16::MIN),
    ];
    assert_eq!(got16, expected16, "i16 saturating_sub mismatch");

    // Unsigned saturating_sub is a separate, unaffected code path - checked as a control.
    let ulhs = black_box(u8x4::from_array([255, 0, 10, 5]));
    let urhs = black_box(u8x4::from_array([0, 255, 20, 5]));
    let ugot = ulhs.saturating_sub(urhs).to_array();
    let uexpected = [
        255u8.saturating_sub(0),
        0u8.saturating_sub(255),
        10u8.saturating_sub(20),
        5u8.saturating_sub(5),
    ];
    assert_eq!(ugot, uexpected, "u8 saturating_sub mismatch");
}
