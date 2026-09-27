#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u8, 8>::from_array(black_box([0u8, 1u8, 255u8, 254u8, 128u8, 127u8, 129u8, 129u8]));
    println!("shl(0)/u8x8: {:?}", v << 0);
    println!("shr(0)/u8x8: {:?}", v >> 0);
    println!("shl(1)/u8x8: {:?}", v << 1);
    println!("shr(1)/u8x8: {:?}", v >> 1);
    println!("shl(4)/u8x8: {:?}", v << 4);
    println!("shr(4)/u8x8: {:?}", v >> 4);
    println!("shl(7)/u8x8: {:?}", v << 7);
    println!("shr(7)/u8x8: {:?}", v >> 7);
    let shifts = Simd::<u8, 8>::from_array(black_box([0, 3, 6, 1, 4, 7, 2, 5]));
    println!("shl(perlane)/u8x8: {:?}", v << shifts);
    println!("shr(perlane)/u8x8: {:?}", v >> shifts);
}
