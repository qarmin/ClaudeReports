// Compiler:
//
// Run-time:
//   status: 0

#![feature(repr_simd, core_intrinsics)]
#![allow(internal_features)]

use std::hint::black_box;
use std::intrinsics::simd::{simd_reduce_max, simd_reduce_min};
use std::mem::transmute;

macro_rules! define_vec {
    ($name:ident, $elem:ty, $n:expr) => {
        #[repr(simd)]
        #[derive(Copy, Clone)]
        struct $name([$elem; $n]);
    };
}

define_vec!(U8x4, u8, 4);
define_vec!(I8x4, i8, 4);
define_vec!(U16x4, u16, 4);
define_vec!(I16x4, i16, 4);
define_vec!(U32x4, u32, 4);
define_vec!(I32x4, i32, 4);
define_vec!(U64x4, u64, 4);
define_vec!(I64x4, i64, 4);

macro_rules! check_direct {
    ($uvec:ident, $ivec:ident, $uty:ty, $ity:ty, $vals:expr) => {{
        let raw: [$uty; 4] = $vals;
        let u = $uvec(black_box(raw));
        let umin: $uty = unsafe { simd_reduce_min(u) };
        let umax: $uty = unsafe { simd_reduce_max(u) };
        assert_eq!(umin, *raw.iter().min().unwrap(), "unsigned reduce_min mismatch for {}", stringify!($uty));
        assert_eq!(umax, *raw.iter().max().unwrap(), "unsigned reduce_max mismatch for {}", stringify!($uty));

        // Now build the exact same bit pattern as the signed type directly (not via transmute)
        // and check the reference oracle against plain Rust comparisons.
        let raw_signed: [$ity; 4] = raw.map(|v| v as $ity);
        let s = $ivec(black_box(raw_signed));
        let smin: $ity = unsafe { simd_reduce_min(s) };
        let smax: $ity = unsafe { simd_reduce_max(s) };
        assert_eq!(smin, *raw_signed.iter().min().unwrap(), "signed reduce_min mismatch for {}", stringify!($ity));
        assert_eq!(smax, *raw_signed.iter().max().unwrap(), "signed reduce_max mismatch for {}", stringify!($ity));
    }};
}

macro_rules! check_transmuted {
    ($uvec:ident, $ivec:ident, $uty:ty, $ity:ty, $vals:expr) => {{
        // Build as the SIGNED vector type first, then transmute (byte-reinterpret) into the
        // UNSIGNED vector type before reducing - this is the shape candidate 020 flagged as not
        // yet ruled out: does the GCC-level element sign get re-derived from the destination
        // (unsigned) type, or does it stay stale from the source (signed) type?
        let raw: [$uty; 4] = $vals;
        let raw_signed: [$ity; 4] = raw.map(|v| v as $ity);
        let s = $ivec(black_box(raw_signed));
        let u: $uvec = unsafe { transmute(s) };
        let umin: $uty = unsafe { simd_reduce_min(u) };
        let umax: $uty = unsafe { simd_reduce_max(u) };
        assert_eq!(umin, *raw.iter().min().unwrap(), "transmuted unsigned reduce_min mismatch for {}", stringify!($uty));
        assert_eq!(umax, *raw.iter().max().unwrap(), "transmuted unsigned reduce_max mismatch for {}", stringify!($uty));

        // And the reverse direction: unsigned -> signed.
        let u2 = $uvec(black_box(raw));
        let s2: $ivec = unsafe { transmute(u2) };
        let smin: $ity = unsafe { simd_reduce_min(s2) };
        let smax: $ity = unsafe { simd_reduce_max(s2) };
        assert_eq!(smin, *raw_signed.iter().min().unwrap(), "transmuted signed reduce_min mismatch for {}", stringify!($ity));
        assert_eq!(smax, *raw_signed.iter().max().unwrap(), "transmuted signed reduce_max mismatch for {}", stringify!($ity));
    }};
}

fn main() {
    // High-bit-set values so signed vs unsigned interpretation genuinely differs.
    check_direct!(U8x4, I8x4, u8, i8, [200, 5, 250, 1]);
    check_direct!(U16x4, I16x4, u16, i16, [60000, 5, 65000, 1]);
    check_direct!(U32x4, I32x4, u32, i32, [4000000000, 5, 4294967290, 1]);
    check_direct!(U64x4, I64x4, u64, i64, [18000000000000000000, 5, 18446744073709551610, 1]);

    check_transmuted!(U8x4, I8x4, u8, i8, [200, 5, 250, 1]);
    check_transmuted!(U16x4, I16x4, u16, i16, [60000, 5, 65000, 1]);
    check_transmuted!(U32x4, I32x4, u32, i32, [4000000000, 5, 4294967290, 1]);
    check_transmuted!(U64x4, I64x4, u64, i64, [18000000000000000000, 5, 18446744073709551610, 1]);

    // Extreme values in the same vector (a-b overflow edge case for the Hacker's Delight
    // difference-or-zero trick, and 0/MAX for the unsigned side).
    check_direct!(U8x4, I8x4, u8, i8, [0, 255, 128, 127]);
    check_direct!(U32x4, I32x4, u32, i32, [0, 4294967295, 2147483648, 2147483647]);
    check_direct!(U64x4, I64x4, u64, i64, [0, 18446744073709551615, 9223372036854775808, 9223372036854775807]);
}
