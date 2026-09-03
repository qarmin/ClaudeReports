#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u16, 8>::from_array(black_box([0u16, 1u16, 65535u16, 65534u16, 32768u16, 32767u16, 32769u16, 32769u16]));
    println!("shl(0)/u16x8: {:?}", v << 0);
    println!("shr(0)/u16x8: {:?}", v >> 0);
    println!("shl(1)/u16x8: {:?}", v << 1);
    println!("shr(1)/u16x8: {:?}", v >> 1);
    println!("shl(8)/u16x8: {:?}", v << 8);
    println!("shr(8)/u16x8: {:?}", v >> 8);
    println!("shl(15)/u16x8: {:?}", v << 15);
    println!("shr(15)/u16x8: {:?}", v >> 15);
    let shifts = Simd::<u16, 8>::from_array(black_box([0, 3, 6, 9, 12, 15, 2, 5]));
    println!("shl(perlane)/u16x8: {:?}", v << shifts);
    println!("shr(perlane)/u16x8: {:?}", v >> shifts);
}
