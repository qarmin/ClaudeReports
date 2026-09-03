#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u64, 8>::from_array(black_box([0u64, 1u64, 18446744073709551615u64, 18446744073709551614u64, 9223372036854775808u64, 9223372036854775807u64, 9223372036854775809u64, 9223372036854775809u64]));
    println!("shl(0)/u64x8: {:?}", v << 0);
    println!("shr(0)/u64x8: {:?}", v >> 0);
    println!("shl(1)/u64x8: {:?}", v << 1);
    println!("shr(1)/u64x8: {:?}", v >> 1);
    println!("shl(32)/u64x8: {:?}", v << 32);
    println!("shr(32)/u64x8: {:?}", v >> 32);
    println!("shl(63)/u64x8: {:?}", v << 63);
    println!("shr(63)/u64x8: {:?}", v >> 63);
    let shifts = Simd::<u64, 8>::from_array(black_box([0, 3, 6, 9, 12, 15, 18, 21]));
    println!("shl(perlane)/u64x8: {:?}", v << shifts);
    println!("shr(perlane)/u64x8: {:?}", v >> shifts);
}
