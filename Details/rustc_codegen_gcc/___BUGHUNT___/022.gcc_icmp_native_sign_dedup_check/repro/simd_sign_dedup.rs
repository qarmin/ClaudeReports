#![feature(portable_simd)]
// Compiler:
//
// Run-time:
//   status: 0

// Escalation for candidate 022: gcc_icmp's native comparison path explicitly skips the
// sign-correcting cast when `a_type.is_vector()` (src/int.rs), unlike the scalar path which
// always re-derives sign from the requested IntPredicate. This checks whether a SIMD-vector
// analog of candidate 018's transmute-based sign-corruption trick can make an unsigned/signed
// vector comparison observe the wrong result (i.e. whether the `is_vector()` skip is actually
// reachable with a wrongly-signed vector GCC type via any normal codegen path).

use std::hint::black_box;
use std::mem::transmute;
use std::simd::cmp::SimdPartialOrd;
use std::simd::{i8x16, u8x16};

fn main() {
    // All bytes 0xFF: semantically -1 (signed) or 255 (unsigned) per lane.
    let signed_all_ff: i8x16 = i8x16::splat(black_box(-1i8));
    let unsigned_via_transmute: u8x16 = unsafe { transmute(signed_all_ff) };

    // Every lane should compare as the maximum unsigned value (255 > 0), not as -1.
    let zero = u8x16::splat(0);
    let gt = unsigned_via_transmute.simd_gt(zero);
    assert!(gt.all(), "transmuted all-0xFF u8x16 should compare > 0 in every lane (got {:?})", gt);

    let cmp_lt = unsigned_via_transmute.simd_lt(u8x16::splat(1));
    assert!(!cmp_lt.any(), "transmuted all-0xFF u8x16 should NOT compare < 1 in any lane");

    // Mirror: transmute unsigned 0xFF into signed, must compare as -1 (< 0).
    let unsigned_all_ff: u8x16 = u8x16::splat(black_box(0xFFu8));
    let signed_via_transmute: i8x16 = unsafe { transmute(unsigned_all_ff) };
    let lt0 = signed_via_transmute.simd_lt(i8x16::splat(0));
    assert!(lt0.all(), "transmuted 0xFF i8x16 should compare < 0 in every lane (got {:?})", lt0);

    // Ordinary (non-transmuted) sanity checks.
    let a = u8x16::splat(200);
    let b = u8x16::splat(100);
    assert!(a.simd_gt(b).all());
    let sa = i8x16::splat(-50);
    let sb = i8x16::splat(50);
    assert!(sa.simd_lt(sb).all());

    println!("all simd checks passed");
}
