#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u64, 4>::from_array(black_box([0u64, 1u64, 18446744073709551615u64, 18446744073709551614u64]));
    let r: Simd<u8, 4> = v.cast();
    println!("cast/u64x4->u8: {:?}", r);
}
