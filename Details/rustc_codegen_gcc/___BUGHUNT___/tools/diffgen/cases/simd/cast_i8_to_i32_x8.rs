#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i8, 8>::from_array(black_box([0i8, 1i8, -1i8, -128i8, -127i8, 127i8, 126i8, 126i8]));
    let r: Simd<i32, 8> = v.cast();
    println!("cast/i8x8->i32: {:?}", r);
}
