#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i8, 4>::from_array(black_box([0i8, 1i8, -1i8, -128i8]));
    let r: Simd<i64, 4> = v.cast();
    println!("cast/i8x4->i64: {:?}", r);
}
