#![feature(portable_simd)]
use std::simd::{Simd, simd_swizzle};

fn main() {
    // Case A (no OOB expected): 1-lane -> 2-lane broadcast via swizzle. Mask length (2) does not
    // exceed twice the input length (2*1=2), so the padding branch in `shuffle_vector`
    // (src/builder.rs, `vec_num_units < mask_num_units`) reads only index 0 of the (1-element)
    // dummy `v2`, which is in bounds.
    let v1: Simd<u8, 1> = Simd::from_array([11]);
    let r1: Simd<u8, 2> = simd_swizzle!(v1, [0, 0]);
    assert_eq!(r1.to_array(), [11, 11]);
    eprintln!("=== case A done (no OOB expected) ===");

    // Case B (OOB expected, matches GitHub issue #605's repro shape): 1-lane -> 4-lane broadcast.
    // Mask length (4) exceeds twice the input length (2*1=2), so the padding branch reads
    // `v2[1]`/`v2[2]` out of bounds on the 1-element dummy vector.
    let v2: Simd<u8, 1> = Simd::from_array([42]);
    let r2: Simd<u8, 4> = simd_swizzle!(v2, [0, 0, 0, 0]);
    assert_eq!(r2.to_array(), [42, 42, 42, 42]);
    eprintln!("=== case B done (OOB expected) ===");

    // Case C (OOB expected, different element width/lane count - same bug class):
    // 2-lane u32 -> 8-lane broadcast. Mask length (8) exceeds twice the input length (2*2=4).
    let v3: Simd<u32, 2> = Simd::from_array([7, 9]);
    let r3: Simd<u32, 8> = simd_swizzle!(v3, [0, 1, 0, 1, 0, 1, 0, 1]);
    assert_eq!(r3.to_array(), [7, 9, 7, 9, 7, 9, 7, 9]);
    eprintln!("=== case C done (OOB expected) ===");

    println!("ok");
}
