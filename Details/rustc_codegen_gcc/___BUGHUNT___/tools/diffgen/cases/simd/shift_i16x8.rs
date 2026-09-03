#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i16, 8>::from_array(black_box([0i16, 1i16, -1i16, -32768i16, -32767i16, 32767i16, 32766i16, 32766i16]));
    println!("shl(0)/i16x8: {:?}", v << 0);
    println!("shr(0)/i16x8: {:?}", v >> 0);
    println!("shl(1)/i16x8: {:?}", v << 1);
    println!("shr(1)/i16x8: {:?}", v >> 1);
    println!("shl(8)/i16x8: {:?}", v << 8);
    println!("shr(8)/i16x8: {:?}", v >> 8);
    println!("shl(15)/i16x8: {:?}", v << 15);
    println!("shr(15)/i16x8: {:?}", v >> 15);
    let shifts = Simd::<i16, 8>::from_array(black_box([0, 3, 6, 9, 12, 15, 2, 5]));
    println!("shl(perlane)/i16x8: {:?}", v << shifts);
    println!("shr(perlane)/i16x8: {:?}", v >> shifts);
}
