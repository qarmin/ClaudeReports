#![feature(portable_simd)]
use std::simd::prelude::*;
use std::hint::black_box;

fn main() {
    let a = Simd::<u8, 4>::from_array(black_box([0u8, 1u8, 255u8, 254u8]));
    let b = Simd::<u8, 4>::from_array(black_box([254u8, 255u8, 1u8, 0u8]));
    let m0 = Mask::<i8, 4>::from_array([false, false, false, false]);
    println!("select(0000)/u8x4: {:?}", m0.select(a, b));
    let m1 = Mask::<i8, 4>::from_array([true, false, false, false]);
    println!("select(0001)/u8x4: {:?}", m1.select(a, b));
    let m2 = Mask::<i8, 4>::from_array([false, true, false, false]);
    println!("select(0010)/u8x4: {:?}", m2.select(a, b));
    let m3 = Mask::<i8, 4>::from_array([true, true, false, false]);
    println!("select(0011)/u8x4: {:?}", m3.select(a, b));
    let m4 = Mask::<i8, 4>::from_array([false, false, true, false]);
    println!("select(0100)/u8x4: {:?}", m4.select(a, b));
    let m5 = Mask::<i8, 4>::from_array([true, false, true, false]);
    println!("select(0101)/u8x4: {:?}", m5.select(a, b));
    let m6 = Mask::<i8, 4>::from_array([false, true, true, false]);
    println!("select(0110)/u8x4: {:?}", m6.select(a, b));
    let m7 = Mask::<i8, 4>::from_array([true, true, true, false]);
    println!("select(0111)/u8x4: {:?}", m7.select(a, b));
    let m8 = Mask::<i8, 4>::from_array([false, false, false, true]);
    println!("select(1000)/u8x4: {:?}", m8.select(a, b));
    let m9 = Mask::<i8, 4>::from_array([true, false, false, true]);
    println!("select(1001)/u8x4: {:?}", m9.select(a, b));
    let m10 = Mask::<i8, 4>::from_array([false, true, false, true]);
    println!("select(1010)/u8x4: {:?}", m10.select(a, b));
    let m11 = Mask::<i8, 4>::from_array([true, true, false, true]);
    println!("select(1011)/u8x4: {:?}", m11.select(a, b));
    let m12 = Mask::<i8, 4>::from_array([false, false, true, true]);
    println!("select(1100)/u8x4: {:?}", m12.select(a, b));
    let m13 = Mask::<i8, 4>::from_array([true, false, true, true]);
    println!("select(1101)/u8x4: {:?}", m13.select(a, b));
    let m14 = Mask::<i8, 4>::from_array([false, true, true, true]);
    println!("select(1110)/u8x4: {:?}", m14.select(a, b));
    let m15 = Mask::<i8, 4>::from_array([true, true, true, true]);
    println!("select(1111)/u8x4: {:?}", m15.select(a, b));
}
