#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i32, 4>::from_array(black_box([0i32, 1i32, -1i32, -2147483648i32]));
    println!("shl(0)/i32x4: {:?}", v << 0);
    println!("shr(0)/i32x4: {:?}", v >> 0);
    println!("shl(1)/i32x4: {:?}", v << 1);
    println!("shr(1)/i32x4: {:?}", v >> 1);
    println!("shl(16)/i32x4: {:?}", v << 16);
    println!("shr(16)/i32x4: {:?}", v >> 16);
    println!("shl(31)/i32x4: {:?}", v << 31);
    println!("shr(31)/i32x4: {:?}", v >> 31);
    let shifts = Simd::<i32, 4>::from_array(black_box([0, 3, 6, 9]));
    println!("shl(perlane)/i32x4: {:?}", v << shifts);
    println!("shr(perlane)/i32x4: {:?}", v >> shifts);
}
