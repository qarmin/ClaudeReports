#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::{simd_eq, simd_lt, simd_gt};
use std::intrinsics::compare_bytes;

#[repr(simd)]
#[derive(Clone, Copy)]
struct I32x4([i32; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct U32x4([u32; 4]);

#[repr(simd)]
#[derive(Clone, Copy)]
struct M32x4([i32; 4]);

fn main() {
    // Case 1: signed comparison with a negative element vs positive - straddling sign bit.
    let a = I32x4([-1, 5, i32::MIN, 0]);
    let b = I32x4([0, 5, 0, -1]);
    let mask: M32x4 = unsafe { simd_lt(a, b) };
    let mask_arr: [i32; 4] = unsafe { std::mem::transmute(mask) };
    // Expected (signed <): -1<0=true(-1), 5<5=false(0), MIN<0=true(-1), 0<-1=false(0)
    let expected = [-1i32, 0, -1, 0];
    println!("signed simd_lt mask: {:?}", mask_arr);
    assert_eq!(mask_arr, expected, "signed simd_lt mismatch");

    // Case 2: unsigned comparison across values that would flip sign if misinterpreted.
    let ua = U32x4([u32::MAX, 5, 2147483648, 0]); // MAX, 5, 2^31, 0
    let ub = U32x4([0, 5, 0, u32::MAX]);
    let umask: M32x4 = unsafe { simd_lt(ua, ub) };
    let umask_arr: [i32; 4] = unsafe { std::mem::transmute(umask) };
    // Expected (unsigned <): MAX<0=false(0),5<5=false(0),2^31<0=false(0),0<MAX=true(-1)
    let uexpected = [0i32, 0, 0, -1];
    println!("unsigned simd_lt mask: {:?}", umask_arr);
    assert_eq!(umask_arr, uexpected, "unsigned simd_lt mismatch");

    // Case 3: eq check basic sanity
    let eqmask: M32x4 = unsafe { simd_eq(a, a) };
    let eqmask_arr: [i32; 4] = unsafe { std::mem::transmute(eqmask) };
    assert_eq!(eqmask_arr, [-1i32, -1, -1, -1], "eq self mismatch");
    println!("eq self mask: {:?}", eqmask_arr);

    // Case 4: gt with unsigned wraparound values
    let ugtmask: M32x4 = unsafe { simd_gt(ub, ua) };
    let ugtmask_arr: [i32; 4] = unsafe { std::mem::transmute(ugtmask) };
    // ub=[0,5,0,MAX], ua=[MAX,5,2^31,0] -> unsigned gt: 0>MAX=false,5>5=false,0>2^31=false,MAX>0=true
    let ugtexpected = [0i32, 0, 0, -1];
    println!("unsigned simd_gt mask: {:?}", ugtmask_arr);
    assert_eq!(ugtmask_arr, ugtexpected, "unsigned simd_gt mismatch");

    // Case 5: compare_bytes (uses sext(cmp, i32) internally in this backend)
    let x = [1u8, 2, 3];
    let y = [1u8, 2, 4];
    let r = unsafe { compare_bytes(x.as_ptr(), y.as_ptr(), 3) };
    println!("compare_bytes(x,y) = {}", r);
    assert!(r < 0, "compare_bytes sign wrong: expected negative, got {}", r);

    let r2 = unsafe { compare_bytes(y.as_ptr(), x.as_ptr(), 3) };
    println!("compare_bytes(y,x) = {}", r2);
    assert!(r2 > 0, "compare_bytes sign wrong: expected positive, got {}", r2);

    println!("ALL CHECKS PASSED");
}
