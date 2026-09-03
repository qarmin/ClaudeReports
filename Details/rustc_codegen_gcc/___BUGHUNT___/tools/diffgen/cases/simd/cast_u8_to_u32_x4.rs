#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u8, 4>::from_array(black_box([0u8, 1u8, 255u8, 254u8]));
    let r: Simd<u32, 4> = v.cast();
    println!("cast/u8x4->u32: {:?}", r);
}
