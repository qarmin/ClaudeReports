// Compiler:
//
// Run-time:
//   status: 0
#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::simd_scatter;

#[repr(simd)]
#[derive(Clone, Copy)]
struct U32x4([u32; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct U32x4Ptr([*mut u32; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct I32x4([i32; 4]);

fn main() {
    let mut data = [0u32; 4];
    let ptrs =
        U32x4Ptr([&mut data[0] as *mut u32, &mut data[1], &mut data[2], &mut data[3]]);
    let values = U32x4([10u32, 20, 30, 40]);
    let mask = I32x4([-1, -1, -1, -1]);

    unsafe {
        simd_scatter(values, ptrs, mask);
    }

    assert_eq!(data, [10, 20, 30, 40]);
}
