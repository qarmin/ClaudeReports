#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::simd_div;

#[repr(simd)]
#[derive(Clone, Copy)]
struct I32x4([i32; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct U32x4([u32; 4]);

fn main() {
    let signed = I32x4([-4, -4, -4, -4]);
    // Reinterpret a signed vector as unsigned without changing the bit pattern,
    // mirroring what a transmute/bitcast from another type would produce.
    let unsigned: U32x4 = unsafe { std::mem::transmute(signed) };
    let divisor = U32x4([2, 2, 2, 2]);

    let result = unsafe { simd_div(unsigned, divisor) };
    let arr: [u32; 4] = unsafe { std::mem::transmute(result) };

    let expected = [((-4i32) as u32) / 2; 4];
    println!("got {:?}, expected {:?}", arr, expected);
    assert_eq!(arr, expected, "simd_div used wrong sign for unsigned vector");
}
