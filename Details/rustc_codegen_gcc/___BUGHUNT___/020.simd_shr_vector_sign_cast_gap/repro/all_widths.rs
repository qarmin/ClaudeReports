#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::simd_shr;

macro_rules! define_pair {
    ($signed_ty:ident, $unsigned_ty:ident, $elem_s:ty, $elem_u:ty, $n:literal) => {
        #[repr(simd)]
        #[derive(Clone, Copy)]
        struct $signed_ty([$elem_s; $n]);

        #[repr(simd)]
        #[derive(Clone, Copy)]
        struct $unsigned_ty([$elem_u; $n]);
    };
}

define_pair!(I8x16, U8x16, i8, u8, 16);
define_pair!(I16x8, U16x8, i16, u16, 8);
define_pair!(I32x4, U32x4, i32, u32, 4);
define_pair!(I64x2, U64x2, i64, u64, 2);

// A logical shift (Uint element type -> lshr) must fill with zero regardless of the
// bit pattern's high bit. An arithmetic shift (Int element type -> ashr) must fill
// with a copy of the sign bit. The two macros below construct a vector whose
// underlying GCC-level element type could plausibly retain the *wrong* sign after a
// transmute (mirroring `tests/run/ptr_to_int_div.rs`'s scalar pointer-to-int case),
// then check the shift result against the sign the Rust-level type actually demands.

macro_rules! check_unsigned_shr {
    ($signed_ty:ident, $unsigned_ty:ident, $elem_s:ty, $elem_u:ty, $n:literal, $neg_val:literal, $shift_val:literal) => {{
        let signed = $signed_ty([$neg_val; $n]);
        // Reinterpret bits: signed (negative) value becomes a large unsigned value
        // with the same bit pattern - Rust-level type is now unsigned => simd_shr
        // must dispatch to a LOGICAL shift (zero-fill), even though the raw bit
        // pattern has its top bit set.
        let unsigned_a: $unsigned_ty = unsafe { std::mem::transmute(signed) };
        let shift = $unsigned_ty([$shift_val; $n]);

        let shr_result = unsafe { simd_shr(unsigned_a, shift) };
        let shr_arr: [$elem_u; $n] = unsafe { std::mem::transmute(shr_result) };
        let expected_shr = [(($neg_val as $elem_s) as $elem_u) >> $shift_val; $n];
        assert_eq!(
            shr_arr,
            expected_shr,
            "simd_shr wrong for {} (unsigned vector from transmuted signed value) - got {:?}, expected logical shift {:?}",
            stringify!($unsigned_ty),
            shr_arr,
            expected_shr,
        );

        println!("{}: shr={:?} OK", stringify!($unsigned_ty), shr_arr);
    }};
}

macro_rules! check_signed_shr {
    ($signed_ty:ident, $unsigned_ty:ident, $elem_s:ty, $elem_u:ty, $n:literal, $big_unsigned_val:literal, $shift_val:literal) => {{
        let unsigned = $unsigned_ty([$big_unsigned_val; $n]);
        // Reinterpret bits: large unsigned value becomes a negative value with the
        // same bit pattern - Rust-level type is now signed => simd_shr must dispatch
        // to an ARITHMETIC shift (sign-fill).
        let signed_a: $signed_ty = unsafe { std::mem::transmute(unsigned) };
        let shift = $signed_ty([$shift_val; $n]);

        let shr_result = unsafe { simd_shr(signed_a, shift) };
        let shr_arr: [$elem_s; $n] = unsafe { std::mem::transmute(shr_result) };
        let expected_shr = [(($big_unsigned_val as $elem_u) as $elem_s) >> $shift_val; $n];
        assert_eq!(
            shr_arr,
            expected_shr,
            "simd_shr wrong for {} (signed vector from transmuted unsigned value) - got {:?}, expected arithmetic shift {:?}",
            stringify!($signed_ty),
            shr_arr,
            expected_shr,
        );

        println!("{}: shr={:?} OK", stringify!($signed_ty), shr_arr);
    }};
}

fn main() {
    check_unsigned_shr!(I8x16, U8x16, i8, u8, 16, -4i8, 1u8);
    check_unsigned_shr!(I16x8, U16x8, i16, u16, 8, -4i16, 1u16);
    check_unsigned_shr!(I32x4, U32x4, i32, u32, 4, -4i32, 1u32);
    check_unsigned_shr!(I64x2, U64x2, i64, u64, 2, -4i64, 1u64);

    check_signed_shr!(I8x16, U8x16, i8, u8, 16, 250u8, 1i8);
    check_signed_shr!(I16x8, U16x8, i16, u16, 8, 65000u16, 1i16);
    check_signed_shr!(I32x4, U32x4, i32, u32, 4, 4000000000u32, 1i32);
    check_signed_shr!(I64x2, U64x2, i64, u64, 2, 18000000000000000000u64, 1i64);

    println!("all checks passed");
}
