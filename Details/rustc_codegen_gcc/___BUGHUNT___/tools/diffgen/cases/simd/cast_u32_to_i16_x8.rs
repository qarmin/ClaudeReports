#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u32, 8>::from_array(black_box([0u32, 1u32, 4294967295u32, 4294967294u32, 2147483648u32, 2147483647u32, 2147483649u32, 2147483649u32]));
    let r: Simd<i16, 8> = v.cast();
    println!("cast/u32x8->i16: {:?}", r);
}
