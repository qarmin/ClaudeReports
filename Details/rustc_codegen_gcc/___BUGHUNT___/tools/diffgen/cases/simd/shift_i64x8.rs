#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i64, 8>::from_array(black_box([0i64, 1i64, -1i64, -9223372036854775808i64, -9223372036854775807i64, 9223372036854775807i64, 9223372036854775806i64, 9223372036854775806i64]));
    println!("shl(0)/i64x8: {:?}", v << 0);
    println!("shr(0)/i64x8: {:?}", v >> 0);
    println!("shl(1)/i64x8: {:?}", v << 1);
    println!("shr(1)/i64x8: {:?}", v >> 1);
    println!("shl(32)/i64x8: {:?}", v << 32);
    println!("shr(32)/i64x8: {:?}", v >> 32);
    println!("shl(63)/i64x8: {:?}", v << 63);
    println!("shr(63)/i64x8: {:?}", v >> 63);
    let shifts = Simd::<i64, 8>::from_array(black_box([0, 3, 6, 9, 12, 15, 18, 21]));
    println!("shl(perlane)/i64x8: {:?}", v << shifts);
    println!("shr(perlane)/i64x8: {:?}", v >> shifts);
}
