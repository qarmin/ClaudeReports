// Differential boundary-value test for src/int.rs width-cast logic (gcc_int_cast, intcast/trunc/
// zext/sext, gcc_lshr/gcc_shl) - specifically the native<->non-native (array-based) i128/u128
// boundary, separate from the division/remainder sign logic already covered by candidates 002/003.
use std::hint::black_box;

macro_rules! show {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), black_box($e));
    };
}

fn main() {
    // --- Widening casts: native int -> u128/i128 ---
    let a_i8: i8 = black_box(-1);
    let a_u8: u8 = black_box(0xFFu8);
    let a_i32: i32 = black_box(-1);
    let a_u32: u32 = black_box(0xFFFF_FFFFu32);
    let a_i64: i64 = black_box(-1);
    let a_u64: u64 = black_box(0xFFFF_FFFF_FFFF_FFFFu64);
    let a_bool: bool = black_box(true);

    show!(a_i8 as u128);
    show!(a_i8 as i128);
    show!(a_u8 as u128);
    show!(a_u8 as i128);
    show!(a_i32 as u128);
    show!(a_i32 as i128);
    show!(a_u32 as u128);
    show!(a_u32 as i128);
    show!(a_i64 as u128);
    show!(a_i64 as i128);
    show!(a_u64 as u128);
    show!(a_u64 as i128);
    show!(a_bool as u128);
    show!(a_bool as i128);

    // Positive boundary values too.
    show!(black_box(127i8) as u128);
    show!(black_box(127i8) as i128);
    show!(black_box(i32::MIN) as u128);
    show!(black_box(i32::MIN) as i128);
    show!(black_box(i64::MIN) as u128);
    show!(black_box(i64::MIN) as i128);

    // --- Narrowing casts: u128/i128 -> native int ---
    let b_i128_neg: i128 = black_box(-1i128);
    let b_i128_min: i128 = black_box(i128::MIN);
    let b_u128_max: u128 = black_box(u128::MAX);
    let b_u128_hi: u128 = black_box(0xFFFF_FFFF_0000_0000_FFFF_FFFF_0000_0000u128);

    show!(b_i128_neg as i8);
    show!(b_i128_neg as u8);
    show!(b_i128_neg as i32);
    show!(b_i128_neg as u32);
    show!(b_i128_neg as i64);
    show!(b_i128_neg as u64);
    show!(b_i128_min as i64);
    show!(b_i128_min as u64);
    show!(b_i128_min as i32);
    show!(b_u128_max as u64);
    show!(b_u128_max as i64);
    show!(b_u128_max as u32);
    show!(b_u128_hi as u64);
    show!(b_u128_hi as u32);

    // --- Same-width reinterpret casts: i128 <-> u128 ---
    show!(black_box(-1i128) as u128);
    show!(black_box(i128::MIN) as u128);
    show!(black_box(u128::MAX) as i128);
    show!(black_box(u128::MAX / 2 + 1) as i128); // sign bit only

    // --- Shifts on 128-bit values (array-based path exercises gcc_lshr/gcc_shl heavily) ---
    let s_i128: i128 = black_box(i128::MIN);
    let s_u128: u128 = black_box(u128::MAX);
    for shift in [0u32, 1, 31, 32, 63, 64, 65, 96, 100, 127] {
        show!(s_i128 >> black_box(shift));
        show!(s_i128 << black_box(shift));
        show!(s_u128 >> black_box(shift));
        show!(s_u128 << black_box(shift));
    }
    let s_i128_pos: i128 = black_box(12345);
    let s_i128_neg: i128 = black_box(-12345);
    for shift in [0u32, 1, 7, 63, 64, 65, 100, 127] {
        show!(s_i128_pos >> black_box(shift));
        show!(s_i128_neg >> black_box(shift));
        show!(s_i128_pos << black_box(shift));
        show!(s_i128_neg << black_box(shift));
    }

    // --- Comparisons on 128-bit values (gcc_icmp non-native path) ---
    let c1: i128 = black_box(-1);
    let c2: i128 = black_box(1);
    let c3: u128 = black_box(u128::MAX);
    let c4: u128 = black_box(1);
    show!(c1 < c2);
    show!(c1 > c2);
    show!(c3 < c4);
    show!(c3 > c4);
    show!(c1 == c2);

    // --- Add/sub in array path (dyncast_array signed-detection, sibling of div/rem) ---
    show!(black_box(i128::MAX).wrapping_add(black_box(1i128)));
    show!(black_box(u128::MAX).wrapping_add(black_box(1u128)));
    show!(black_box(i128::MIN).wrapping_sub(black_box(1i128)));
    show!(black_box(0u128).wrapping_sub(black_box(1u128)));

    // --- bswap / reverse_bits (gcc_bswap recursion into element halves) ---
    show!(black_box(0x0102030405060708090A0B0C0D0E0F10u128).swap_bytes());
    show!(black_box(-1i128).swap_bytes());
    show!(black_box(1i128).swap_bytes());
    show!(black_box(0x0102030405060708090A0B0C0D0E0F10u128).reverse_bits());
    show!(black_box(1u128).reverse_bits());
    show!(black_box(-1i128).reverse_bits());

    // --- narrowing then widening round trips (double cast through native boundary) ---
    show!((black_box(-1i128) as i8) as i128);
    show!((black_box(u128::MAX) as u8) as u128);
    show!((black_box(i128::MIN) as u64) as u128);
    show!((black_box(i128::MIN) as i64) as i128);

    // wrapping neg/not on 128-bit (gcc_neg/gcc_not array path)
    show!(black_box(i128::MIN).wrapping_neg());
    show!(!black_box(0u128));
    show!(!black_box(i128::MIN));
}
