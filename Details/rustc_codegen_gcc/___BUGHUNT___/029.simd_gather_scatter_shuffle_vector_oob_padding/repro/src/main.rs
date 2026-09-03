#![feature(portable_simd)]

use std::simd::ptr::{SimdConstPtr, SimdMutPtr};
use std::simd::{Mask, Simd};

fn main() {
    // 4-lane gather, all pointers valid, boundary lanes disabled (mask = [false, true, true, false]).
    let values4 = [10i32, 20, 30, 40];
    let ptrs4: Simd<*const i32, 4> = Simd::from_array([
        values4.as_ptr(),
        values4.as_ptr().wrapping_add(1),
        values4.as_ptr().wrapping_add(2),
        values4.as_ptr().wrapping_add(3),
    ]);
    let mask4 = Mask::from_array([false, true, true, false]);
    let or4 = Simd::from_array([-1, -1, -1, -1]);
    let g4 = unsafe { Simd::gather_select_ptr(ptrs4, mask4, or4) };
    println!("g4 = {:?}", g4.to_array());
    assert_eq!(g4.to_array(), [-1, 20, 30, -1]);

    // 8-lane gather, valid pointers throughout, alternating mask - exercises in_len=8.
    let values8 = [1i32, 2, 3, 4, 5, 6, 7, 8];
    let ptrs8: Simd<*const i32, 8> = Simd::from_array([
        values8.as_ptr(),
        values8.as_ptr().wrapping_add(1),
        values8.as_ptr().wrapping_add(2),
        values8.as_ptr().wrapping_add(3),
        values8.as_ptr().wrapping_add(4),
        values8.as_ptr().wrapping_add(5),
        values8.as_ptr().wrapping_add(6),
        values8.as_ptr().wrapping_add(7),
    ]);
    let mask8 = Mask::from_array([true, false, true, false, true, false, true, false]);
    let or8 = Simd::from_array([0i32; 8]);
    let g8 = unsafe { Simd::gather_select_ptr(ptrs8, mask8, or8) };
    println!("g8 = {:?}", g8.to_array());
    assert_eq!(g8.to_array(), [1, 0, 3, 0, 5, 0, 7, 0]);

    // 16-lane gather - larger in_len, still equal-length vector/default/mask throughout gather().
    let values16: [i32; 16] = core::array::from_fn(|i| (i as i32) + 1);
    let mut ptrs16_arr = [std::ptr::null::<i32>(); 16];
    for i in 0..16 {
        ptrs16_arr[i] = values16.as_ptr().wrapping_add(i);
    }
    let ptrs16: Simd<*const i32, 16> = Simd::from_array(ptrs16_arr);
    let mut mask16_arr = [false; 16];
    for i in 0..16 {
        mask16_arr[i] = i % 3 == 0;
    }
    let mask16 = Mask::from_array(mask16_arr);
    let or16 = Simd::from_array([-1i32; 16]);
    let g16 = unsafe { Simd::gather_select_ptr(ptrs16, mask16, or16) };
    println!("g16 = {:?}", g16.to_array());
    for i in 0..16 {
        let expected = if i % 3 == 0 { values16[i] } else { -1 };
        assert_eq!(g16.to_array()[i], expected);
    }

    // Scatter, same shape.
    let mut out4 = [0i32; 4];
    let out_ptrs4: Simd<*mut i32, 4> = Simd::from_array([
        out4.as_mut_ptr(),
        out4.as_mut_ptr().wrapping_add(1),
        out4.as_mut_ptr().wrapping_add(2),
        out4.as_mut_ptr().wrapping_add(3),
    ]);
    let mask_s4 = Mask::from_array([true, false, true, false]);
    let to_write4 = Simd::from_array([100i32, 200, 300, 400]);
    unsafe { to_write4.scatter_select_ptr(out_ptrs4, mask_s4) };
    println!("out4 = {:?}", out4);
    assert_eq!(out4, [100, 0, 300, 0]);

    println!("ok");
}
