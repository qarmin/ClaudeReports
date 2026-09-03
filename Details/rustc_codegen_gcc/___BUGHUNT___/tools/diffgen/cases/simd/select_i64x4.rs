#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let a = Simd::<i64, 4>::from_array(black_box([0i64, 1i64, -1i64, -9223372036854775808i64]));
    let b = Simd::<i64, 4>::from_array(black_box([-9223372036854775808i64, -1i64, 1i64, 0i64]));
    let m0 = Mask::<i64, 4>::from_array([false, false, false, false]);
    println!("select(0000)/i64x4: {:?}", m0.select(a, b));
    let m1 = Mask::<i64, 4>::from_array([true, false, false, false]);
    println!("select(0001)/i64x4: {:?}", m1.select(a, b));
    let m2 = Mask::<i64, 4>::from_array([false, true, false, false]);
    println!("select(0010)/i64x4: {:?}", m2.select(a, b));
    let m3 = Mask::<i64, 4>::from_array([true, true, false, false]);
    println!("select(0011)/i64x4: {:?}", m3.select(a, b));
    let m4 = Mask::<i64, 4>::from_array([false, false, true, false]);
    println!("select(0100)/i64x4: {:?}", m4.select(a, b));
    let m5 = Mask::<i64, 4>::from_array([true, false, true, false]);
    println!("select(0101)/i64x4: {:?}", m5.select(a, b));
    let m6 = Mask::<i64, 4>::from_array([false, true, true, false]);
    println!("select(0110)/i64x4: {:?}", m6.select(a, b));
    let m7 = Mask::<i64, 4>::from_array([true, true, true, false]);
    println!("select(0111)/i64x4: {:?}", m7.select(a, b));
    let m8 = Mask::<i64, 4>::from_array([false, false, false, true]);
    println!("select(1000)/i64x4: {:?}", m8.select(a, b));
    let m9 = Mask::<i64, 4>::from_array([true, false, false, true]);
    println!("select(1001)/i64x4: {:?}", m9.select(a, b));
    let m10 = Mask::<i64, 4>::from_array([false, true, false, true]);
    println!("select(1010)/i64x4: {:?}", m10.select(a, b));
    let m11 = Mask::<i64, 4>::from_array([true, true, false, true]);
    println!("select(1011)/i64x4: {:?}", m11.select(a, b));
    let m12 = Mask::<i64, 4>::from_array([false, false, true, true]);
    println!("select(1100)/i64x4: {:?}", m12.select(a, b));
    let m13 = Mask::<i64, 4>::from_array([true, false, true, true]);
    println!("select(1101)/i64x4: {:?}", m13.select(a, b));
    let m14 = Mask::<i64, 4>::from_array([false, true, true, true]);
    println!("select(1110)/i64x4: {:?}", m14.select(a, b));
    let m15 = Mask::<i64, 4>::from_array([true, true, true, true]);
    println!("select(1111)/i64x4: {:?}", m15.select(a, b));
}
