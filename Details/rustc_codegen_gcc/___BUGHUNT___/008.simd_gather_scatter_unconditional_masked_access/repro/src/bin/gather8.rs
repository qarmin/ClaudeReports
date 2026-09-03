#![feature(portable_simd)]

use std::simd::{Mask, Simd};

fn main() {
    let values = [10i32, 20, 30, 40, 50, 60, 70, 80];

    // Every lane disabled and every pointer null - this must not touch memory at all.
    let ptrs_all_disabled: Simd<*const i32, 8> = Simd::splat(std::ptr::null());
    let mask_all_disabled = Mask::from_array([false; 8]);
    let or = Simd::from_array([1, 2, 3, 4, 5, 6, 7, 8]);
    let g1 = unsafe { Simd::gather_select_ptr(ptrs_all_disabled, mask_all_disabled, or) };
    assert_eq!(g1.to_array(), [1, 2, 3, 4, 5, 6, 7, 8]);
    eprintln!("all-disabled gather ok");

    // Boundary lanes (first and last) disabled with null pointers, interior lanes enabled.
    let ptrs = Simd::from_array([
        std::ptr::null(),
        values.as_ptr().wrapping_add(1),
        values.as_ptr().wrapping_add(2),
        values.as_ptr().wrapping_add(3),
        values.as_ptr().wrapping_add(4),
        values.as_ptr().wrapping_add(5),
        values.as_ptr().wrapping_add(6),
        std::ptr::null(),
    ]);
    let mask = Mask::from_array([false, true, true, true, true, true, true, false]);
    let or2 = Simd::splat(-1);
    let g2 = unsafe { Simd::gather_select_ptr(ptrs, mask, or2) };
    assert_eq!(g2.to_array(), [-1, 20, 30, 40, 50, 60, 70, -1]);
    eprintln!("boundary-disabled gather ok");
}
