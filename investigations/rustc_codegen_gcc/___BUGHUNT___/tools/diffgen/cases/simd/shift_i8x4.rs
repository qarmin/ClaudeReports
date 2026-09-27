#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i8, 4>::from_array(black_box([0i8, 1i8, -1i8, -128i8]));
    println!("shl(0)/i8x4: {:?}", v << 0);
    println!("shr(0)/i8x4: {:?}", v >> 0);
    println!("shl(1)/i8x4: {:?}", v << 1);
    println!("shr(1)/i8x4: {:?}", v >> 1);
    println!("shl(4)/i8x4: {:?}", v << 4);
    println!("shr(4)/i8x4: {:?}", v >> 4);
    println!("shl(7)/i8x4: {:?}", v << 7);
    println!("shr(7)/i8x4: {:?}", v >> 7);
    let shifts = Simd::<i8, 4>::from_array(black_box([0, 3, 6, 1]));
    println!("shl(perlane)/i8x4: {:?}", v << shifts);
    println!("shr(perlane)/i8x4: {:?}", v >> shifts);
}
