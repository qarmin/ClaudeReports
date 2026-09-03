#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i32, 4>::from_array(black_box([0i32, 1i32, -1i32, -2147483648i32]));
    let r: Simd<u16, 4> = v.cast();
    println!("cast/i32x4->u16: {:?}", r);
}
