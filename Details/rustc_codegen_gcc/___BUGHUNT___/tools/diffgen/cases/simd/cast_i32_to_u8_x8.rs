#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i32, 8>::from_array(black_box([0i32, 1i32, -1i32, -2147483648i32, -2147483647i32, 2147483647i32, 2147483646i32, 2147483646i32]));
    let r: Simd<u8, 8> = v.cast();
    println!("cast/i32x8->u8: {:?}", r);
}
