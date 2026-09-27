#![feature(portable_simd)]

use std::simd::ptr::SimdMutPtr;
use std::simd::{Mask, Simd};

fn main() {
    let mut values = [0i32; 4];

    // Lane 2 is disabled and its pointer is null. Per the documented safety contract of
    // `scatter_select_ptr`, "If a pointer is disabled, the write to its pointee is skipped" - so
    // a null pointer in a disabled lane must be completely safe.
    let ptrs: Simd<*mut i32, 4> = Simd::from_array([
        values.as_mut_ptr(),
        values.as_mut_ptr().wrapping_add(1),
        std::ptr::null_mut(),
        values.as_mut_ptr().wrapping_add(3),
    ]);
    let mask = Mask::from_array([true, true, false, true]);
    let to_write = Simd::from_array([10, 20, 30, 40]);

    eprintln!("about to scatter (lane 2 disabled, points to null)");
    unsafe { to_write.scatter_select_ptr(ptrs, mask) };
    eprintln!("values = {:?}", values);
    assert_eq!(values, [10, 20, 0, 40]);
    eprintln!("scatter ok");
}
