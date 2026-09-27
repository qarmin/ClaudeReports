#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u16, 8>::from_array(black_box([0u16, 1u16, 65535u16, 65534u16, 32768u16, 32767u16, 32769u16, 32769u16]));
    let r: Simd<i32, 8> = v.cast();
    println!("cast/u16x8->i32: {:?}", r);
}
