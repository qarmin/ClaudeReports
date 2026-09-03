#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u16, 4>::from_array(black_box([0u16, 1u16, 65535u16, 65534u16]));
    let r: Simd<i64, 4> = v.cast();
    println!("cast/u16x4->i64: {:?}", r);
}
