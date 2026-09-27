// Repro for cg_clif candidate 013: `simd_cast`/`simd_as` between two *identical* `f16` (or
// `f128`) SIMD types ICEs the backend at src/codegen_f16_f128.rs:197 `unreachable!`.
//
// Under LLVM this program compiles and prints the lane bit patterns unchanged (the cast is a
// no-op). Under cg_clif it does not compile at all.
//
// Every `go*` function is `#[inline(never)]` and the crate is built at opt-level 0 so rustc's MIR
// GVN cannot const-fold the intrinsic through the const-eval interpreter and hide the backend.

#![feature(f16, f128, core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::{simd_as, simd_cast};

macro_rules! vec_ty {
    ($name:ident, $elem:ty, $n:expr) => {
        #[repr(simd)]
        #[derive(Copy, Clone)]
        struct $name([$elem; $n]);
    };
}

vec_ty!(F16x1, f16, 1);
vec_ty!(F16x2, f16, 2);
vec_ty!(F16x4, f16, 4);
vec_ty!(F16x8, f16, 8);
vec_ty!(F16x16, f16, 16);
vec_ty!(F128x1, f128, 1);
vec_ty!(F128x2, f128, 2);
vec_ty!(F128x4, f128, 4);
vec_ty!(F32x4, f32, 4);
vec_ty!(F64x4, f64, 4);

// The bug: source and destination lane types are the same `f16`/`f128`.
#[inline(never)]
fn cast_f16x1(a: F16x1) -> F16x1 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f16x2(a: F16x2) -> F16x2 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f16x4(a: F16x4) -> F16x4 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f16x8(a: F16x8) -> F16x8 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f16x16(a: F16x16) -> F16x16 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn as_f16x4(a: F16x4) -> F16x4 {
    unsafe { simd_as(a) }
}
#[inline(never)]
fn cast_f128x1(a: F128x1) -> F128x1 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f128x2(a: F128x2) -> F128x2 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f128x4(a: F128x4) -> F128x4 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn as_f128x2(a: F128x2) -> F128x2 {
    unsafe { simd_as(a) }
}

// Controls: the *widening/narrowing* f16/f128 SIMD casts, which already worked. They must keep
// working after the fix.
#[inline(never)]
fn cast_f16x4_to_f32x4(a: F16x4) -> F32x4 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f32x4_to_f16x4(a: F32x4) -> F16x4 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f64x4_to_f128x4(a: F64x4) -> F128x4 {
    unsafe { simd_cast(a) }
}
#[inline(never)]
fn cast_f128x4_to_f64x4(a: F128x4) -> F64x4 {
    unsafe { simd_cast(a) }
}

// `#[repr(simd)]` fields cannot be projected into (MCP#838), so read the lanes back by transmute.
macro_rules! bits {
    ($v:expr, $elem:ty, $n:expr) => {{
        let arr: [$elem; $n] = unsafe { std::mem::transmute($v) };
        arr.map(|x| x.to_bits())
    }};
}

const F16_VALS: [f16; 16] = [
    0.0,
    -0.0,
    1.5,
    -2.25,
    3.0,
    0.5,
    f16::NAN,
    f16::INFINITY,
    f16::NEG_INFINITY,
    f16::MIN,
    f16::MAX,
    f16::MIN_POSITIVE,
    6.0e-8, // subnormal
    -1.0,
    65504.0,
    1.0,
];

const F128_VALS: [f128; 4] = [0.0, -0.0, f128::NAN, f128::MAX];

fn main() {
    println!("--- f16 identity simd_cast, lane counts 1/2/4/8/16 ---");
    println!("{:?}", bits!(cast_f16x1(F16x1([F16_VALS[6]])), f16, 1));
    println!("{:?}", bits!(cast_f16x2(F16x2([F16_VALS[0], F16_VALS[1]])), f16, 2));
    let v4 = F16x4([F16_VALS[2], F16_VALS[3], F16_VALS[9], F16_VALS[12]]);
    println!("{:?}", bits!(cast_f16x4(v4), f16, 4));
    let mut a8 = [0.0f16; 8];
    a8.copy_from_slice(&F16_VALS[..8]);
    println!("{:?}", bits!(cast_f16x8(F16x8(a8)), f16, 8));
    println!("{:?}", bits!(cast_f16x16(F16x16(F16_VALS)), f16, 16));

    println!("--- f16 identity simd_as ---");
    println!("{:?}", bits!(as_f16x4(v4), f16, 4));

    println!("--- f128 identity simd_cast, lane counts 1/2/4 ---");
    println!("{:?}", bits!(cast_f128x1(F128x1([F128_VALS[2]])), f128, 1));
    println!("{:?}", bits!(cast_f128x2(F128x2([F128_VALS[0], F128_VALS[1]])), f128, 2));
    println!("{:?}", bits!(cast_f128x4(F128x4(F128_VALS)), f128, 4));

    println!("--- f128 identity simd_as ---");
    println!("{:?}", bits!(as_f128x2(F128x2([F128_VALS[3], F128_VALS[1]])), f128, 2));

    println!("--- controls: non-identity f16/f128 SIMD casts ---");
    println!("{:?}", bits!(cast_f16x4_to_f32x4(v4), f32, 4));
    println!("{:?}", bits!(cast_f32x4_to_f16x4(F32x4([1.5, -2.25, 1.0e30, f32::NAN])), f16, 4));
    println!("{:?}", bits!(cast_f64x4_to_f128x4(F64x4([1.5, -2.25, 0.0, f64::NAN])), f128, 4));
    println!("{:?}", bits!(cast_f128x4_to_f64x4(F128x4(F128_VALS)), f64, 4));
}
