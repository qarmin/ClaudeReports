#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<u32, 4>::from_array(black_box([0u32, 1u32, 4294967295u32, 4294967294u32]));
    println!("shl(0)/u32x4: {:?}", v << 0);
    println!("shr(0)/u32x4: {:?}", v >> 0);
    println!("shl(1)/u32x4: {:?}", v << 1);
    println!("shr(1)/u32x4: {:?}", v >> 1);
    println!("shl(16)/u32x4: {:?}", v << 16);
    println!("shr(16)/u32x4: {:?}", v >> 16);
    println!("shl(31)/u32x4: {:?}", v << 31);
    println!("shr(31)/u32x4: {:?}", v >> 31);
    let shifts = Simd::<u32, 4>::from_array(black_box([0, 3, 6, 9]));
    println!("shl(perlane)/u32x4: {:?}", v << shifts);
    println!("shr(perlane)/u32x4: {:?}", v >> shifts);
}
