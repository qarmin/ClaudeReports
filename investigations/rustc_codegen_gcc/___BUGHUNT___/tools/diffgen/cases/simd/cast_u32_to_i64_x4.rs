#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u32, 4>::from_array(black_box([0u32, 1u32, 4294967295u32, 4294967294u32]));
    let r: Simd<i64, 4> = v.cast();
    println!("cast/u32x4->i64: {:?}", r);
}
