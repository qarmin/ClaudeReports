// Compiler:
//
// Run-time:
//   status: 0
#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::simd_scatter;

#[repr(simd)]
#[derive(Clone, Copy)]
struct U8x4([u8; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct U8x4Ptr([*mut u8; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct I8x4([i8; 4]);

fn main() {
    let mut data = [0u8; 4];
    let ptrs = U8x4Ptr([&mut data[0] as *mut u8, &mut data[1], &mut data[2], &mut data[3]]);
    let values = U8x4([200u8, 201, 202, 255]);
    let mask = I8x4([-1, -1, -1, -1]);

    unsafe {
        simd_scatter(values, ptrs, mask);
    }

    assert_eq!(data, [200, 201, 202, 255]);
}
