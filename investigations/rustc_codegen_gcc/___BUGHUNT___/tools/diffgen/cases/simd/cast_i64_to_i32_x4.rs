#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i64, 4>::from_array(black_box([0i64, 1i64, -1i64, -9223372036854775808i64]));
    let r: Simd<i32, 4> = v.cast();
    println!("cast/i64x4->i32: {:?}", r);
}
