#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u64, 8>::from_array(black_box([0u64, 1u64, 18446744073709551615u64, 18446744073709551614u64, 9223372036854775808u64, 9223372036854775807u64, 9223372036854775809u64, 9223372036854775809u64]));
    let r: Simd<u16, 8> = v.cast();
    println!("cast/u64x8->u16: {:?}", r);
}
