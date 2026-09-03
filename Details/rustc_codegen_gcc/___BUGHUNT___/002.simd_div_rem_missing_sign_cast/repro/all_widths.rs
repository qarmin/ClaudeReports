#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::{simd_div, simd_rem};

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

macro_rules! check_unsigned_div_rem {
    ($signed_ty:ident, $unsigned_ty:ident, $elem_s:ty, $elem_u:ty, $n:literal, $neg_val:literal, $divisor_val:literal) => {{
        let signed = $signed_ty([$neg_val; $n]);
        // Reinterpret bits: signed value becomes a large unsigned value with the same bit pattern.
        let unsigned_a: $unsigned_ty = unsafe { std::mem::transmute(signed) };
        let divisor = $unsigned_ty([$divisor_val; $n]);

        let div_result = unsafe { simd_div(unsigned_a, divisor) };
        let div_arr: [$elem_u; $n] = unsafe { std::mem::transmute(div_result) };
        let expected_div = [(($neg_val as $elem_s) as $elem_u) / $divisor_val; $n];
        assert_eq!(
            div_arr,
            expected_div,
            "simd_div wrong for {} (unsigned vector from transmuted signed value)",
            stringify!($unsigned_ty)
        );

        let rem_result = unsafe { simd_rem(unsigned_a, divisor) };
        let rem_arr: [$elem_u; $n] = unsafe { std::mem::transmute(rem_result) };
        let expected_rem = [(($neg_val as $elem_s) as $elem_u) % $divisor_val; $n];
        assert_eq!(
            rem_arr,
            expected_rem,
            "simd_rem wrong for {} (unsigned vector from transmuted signed value)",
            stringify!($unsigned_ty)
        );

        println!("{}: div={:?} rem={:?} OK", stringify!($unsigned_ty), div_arr, rem_arr);
    }};
}

macro_rules! check_signed_div_rem {
    ($signed_ty:ident, $unsigned_ty:ident, $elem_s:ty, $elem_u:ty, $n:literal, $big_unsigned_val:literal, $divisor_val:literal) => {{
        let unsigned = $unsigned_ty([$big_unsigned_val; $n]);
        // Reinterpret bits: large unsigned value becomes a negative value with the same bit pattern.
        let signed_a: $signed_ty = unsafe { std::mem::transmute(unsigned) };
        let divisor = $signed_ty([$divisor_val; $n]);

        let div_result = unsafe { simd_div(signed_a, divisor) };
        let div_arr: [$elem_s; $n] = unsafe { std::mem::transmute(div_result) };
        let expected_div = [(($big_unsigned_val as $elem_u) as $elem_s) / $divisor_val; $n];
        assert_eq!(
            div_arr,
            expected_div,
            "simd_div wrong for {} (signed vector from transmuted unsigned value)",
            stringify!($signed_ty)
        );

        let rem_result = unsafe { simd_rem(signed_a, divisor) };
        let rem_arr: [$elem_s; $n] = unsafe { std::mem::transmute(rem_result) };
        let expected_rem = [(($big_unsigned_val as $elem_u) as $elem_s) % $divisor_val; $n];
        assert_eq!(
            rem_arr,
            expected_rem,
            "simd_rem wrong for {} (signed vector from transmuted unsigned value)",
            stringify!($signed_ty)
        );

        println!("{}: div={:?} rem={:?} OK", stringify!($signed_ty), div_arr, rem_arr);
    }};
}

fn main() {
    check_unsigned_div_rem!(I8x16, U8x16, i8, u8, 16, -4i8, 3u8);
    check_unsigned_div_rem!(I16x8, U16x8, i16, u16, 8, -4i16, 3u16);
    check_unsigned_div_rem!(I32x4, U32x4, i32, u32, 4, -4i32, 3u32);
    check_unsigned_div_rem!(I64x2, U64x2, i64, u64, 2, -4i64, 3u64);

    check_signed_div_rem!(I8x16, U8x16, i8, u8, 16, 250u8, 3i8);
    check_signed_div_rem!(I16x8, U16x8, i16, u16, 8, 65000u16, 3i16);
    check_signed_div_rem!(I32x4, U32x4, i32, u32, 4, 4000000000u32, 3i32);
    check_signed_div_rem!(I64x2, U64x2, i64, u64, 2, 18000000000000000000u64, 3i64);

    println!("all checks passed");
}
