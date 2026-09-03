#![feature(portable_simd)]

use std::simd::ptr::SimdConstPtr;
use std::simd::{Mask, Simd};

fn main() {
    let values = [1i32, 2, 3, 4];

    // Lane 2 is disabled by the mask and its pointer is null. Per the documented safety contract
    // of `gather_select_ptr`, "If a pointer is disabled, ... no read is performed" - so a null
    // pointer in a disabled lane must be completely safe.
    let ptrs: Simd<*const i32, 4> = Simd::from_array([
        values.as_ptr(),
        values.as_ptr().wrapping_add(1),
        std::ptr::null(),
        values.as_ptr().wrapping_add(3),
    ]);
    let mask = Mask::from_array([true, true, false, true]);
    let or = Simd::from_array([-1, -1, -1, -1]);

    eprintln!("about to gather (lane 2 disabled, points to null)");
    let gathered = unsafe { Simd::gather_select_ptr(ptrs, mask, or) };
    eprintln!("gathered = {:?}", gathered.to_array());
    assert_eq!(gathered.to_array(), [1, 2, -1, 4]);
    eprintln!("gather ok");
}
