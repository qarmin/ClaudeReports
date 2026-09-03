#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let v = Simd::<i64, 8>::from_array(black_box([0i64, 1i64, -1i64, -9223372036854775808i64, -9223372036854775807i64, 9223372036854775807i64, 9223372036854775806i64, 9223372036854775806i64]));
    let r: Simd<i16, 8> = v.cast();
    println!("cast/i64x8->i16: {:?}", r);
}
