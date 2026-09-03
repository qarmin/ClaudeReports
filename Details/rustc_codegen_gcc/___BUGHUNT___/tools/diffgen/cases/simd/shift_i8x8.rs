#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i8, 8>::from_array(black_box([0i8, 1i8, -1i8, -128i8, -127i8, 127i8, 126i8, 126i8]));
    println!("shl(0)/i8x8: {:?}", v << 0);
    println!("shr(0)/i8x8: {:?}", v >> 0);
    println!("shl(1)/i8x8: {:?}", v << 1);
    println!("shr(1)/i8x8: {:?}", v >> 1);
    println!("shl(4)/i8x8: {:?}", v << 4);
    println!("shr(4)/i8x8: {:?}", v >> 4);
    println!("shl(7)/i8x8: {:?}", v << 7);
    println!("shr(7)/i8x8: {:?}", v >> 7);
    let shifts = Simd::<i8, 8>::from_array(black_box([0, 3, 6, 1, 4, 7, 2, 5]));
    println!("shl(perlane)/i8x8: {:?}", v << shifts);
    println!("shr(perlane)/i8x8: {:?}", v >> shifts);
}
