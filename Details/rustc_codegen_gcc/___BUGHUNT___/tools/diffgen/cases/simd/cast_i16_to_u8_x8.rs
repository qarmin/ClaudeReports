#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i16, 8>::from_array(black_box([0i16, 1i16, -1i16, -32768i16, -32767i16, 32767i16, 32766i16, 32766i16]));
    let r: Simd<u8, 8> = v.cast();
    println!("cast/i16x8->u8: {:?}", r);
}
