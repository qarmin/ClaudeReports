#![feature(repr_simd, core_intrinsics)]
#![allow(internal_features, non_camel_case_types)]
use std::intrinsics::simd::{simd_extract_dyn, simd_insert_dyn};

macro_rules! mk {
    ($name:ident, $t:ty, $n:expr) => {
        #[repr(simd)]
        #[derive(Copy, Clone)]
        struct $name([$t; $n]);
    };
}

macro_rules! sweep {
    ($name:ident, $t:ty, $n:expr, $mk:expr) => {{
        let v = $name(core::array::from_fn(|i| $mk(i)));
        let mut out = String::new();
        for i in 0..$n {
            let e: $t = unsafe { simd_extract_dyn(v, blackbox(i as u32)) };
            out.push_str(&format!("{:?} ", e));
        }
        // insert_dyn: overwrite each lane with a marker, read back the whole vector
        for i in 0..$n {
            let w: $name = unsafe { simd_insert_dyn(v, blackbox(i as u32), $mk(99)) };
            for j in 0..$n {
                let e: $t = unsafe { simd_extract_dyn(w, blackbox(j as u32)) };
                out.push_str(&format!("{:?},", e));
            }
            out.push(' ');
        }
        println!("{} {}", stringify!($name), out);
    }};
}

#[inline(never)]
fn blackbox(x: u32) -> u32 { std::hint::black_box(x) }

mk!(u8x2, u8, 2);
mk!(u8x3, u8, 3);
mk!(u8x16, u8, 16);
mk!(u8x32, u8, 32);
mk!(u8x64, u8, 64);
mk!(i16x8, i16, 8);
mk!(i16x5, i16, 5);
mk!(u32x4, u32, 4);
mk!(u32x8, u32, 8);
mk!(u32x1, u32, 1);
mk!(i64x2, i64, 2);
mk!(i64x7, i64, 7);
mk!(f32x4, f32, 4);
mk!(f64x8, f64, 8);
mk!(u128x2, u128, 2);
mk!(i128x3, i128, 3);

fn main() {
    sweep!(u8x2, u8, 2, |i: usize| (i as u8).wrapping_mul(13).wrapping_add(1));
    sweep!(u8x3, u8, 3, |i: usize| (i as u8).wrapping_mul(13).wrapping_add(1));
    sweep!(u8x16, u8, 16, |i: usize| (i as u8).wrapping_mul(13).wrapping_add(1));
    sweep!(u8x32, u8, 32, |i: usize| (i as u8).wrapping_mul(13).wrapping_add(1));
    sweep!(u8x64, u8, 64, |i: usize| (i as u8).wrapping_mul(13).wrapping_add(1));
    sweep!(i16x8, i16, 8, |i: usize| (i as i16).wrapping_mul(-777).wrapping_add(5));
    sweep!(i16x5, i16, 5, |i: usize| (i as i16).wrapping_mul(-777).wrapping_add(5));
    sweep!(u32x4, u32, 4, |i: usize| (i as u32).wrapping_mul(0x0101_0101).wrapping_add(7));
    sweep!(u32x8, u32, 8, |i: usize| (i as u32).wrapping_mul(0x0101_0101).wrapping_add(7));
    sweep!(u32x1, u32, 1, |i: usize| (i as u32).wrapping_mul(0x0101_0101).wrapping_add(7));
    sweep!(i64x2, i64, 2, |i: usize| (i as i64).wrapping_mul(-0x0123_4567_89ab));
    sweep!(i64x7, i64, 7, |i: usize| (i as i64).wrapping_mul(-0x0123_4567_89ab));
    sweep!(f32x4, f32, 4, |i: usize| (i as f32) * -3.25 + 0.5);
    sweep!(f64x8, f64, 8, |i: usize| (i as f64) * -3.25 + 0.5);
    sweep!(u128x2, u128, 2, |i: usize| (i as u128).wrapping_mul(0x0011_2233_4455_6677_8899_aabb_ccdd_eeff));
    sweep!(i128x3, i128, 3, |i: usize| (i as i128).wrapping_mul(-0x0011_2233_4455_6677_8899_aabb_ccdd));
}
