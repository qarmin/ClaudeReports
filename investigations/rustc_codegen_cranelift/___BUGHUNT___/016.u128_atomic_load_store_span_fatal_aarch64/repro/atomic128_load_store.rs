#![feature(core_intrinsics)]
#![allow(internal_features)]

use std::intrinsics::{AtomicOrdering, atomic_load, atomic_store};

#[repr(align(16))]
struct A16<T>(T);

fn roundtrip_u128(v: u128) -> u128 {
    let cell = A16(v);
    unsafe {
        let p = &cell.0 as *const u128;
        let a = atomic_load::<u128, { AtomicOrdering::SeqCst }, false>(p);
        let b = atomic_load::<u128, { AtomicOrdering::Relaxed }, false>(p);
        let c = atomic_load::<u128, { AtomicOrdering::Acquire }, false>(p);
        assert_eq!(a, b);
        assert_eq!(a, c);
        a
    }
}

fn store_then_load_i128(v: i128) -> i128 {
    let mut cell = A16(0i128);
    unsafe {
        let p = &mut cell.0 as *mut i128;
        atomic_store::<i128, { AtomicOrdering::SeqCst }, false>(p, v);
        atomic_load::<i128, { AtomicOrdering::SeqCst }, false>(p)
    }
}

fn main() {
    for v in [
        0u128,
        1,
        u128::MAX,
        1u128 << 64,
        (1u128 << 64) - 1,
        0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        u128::MAX - 1,
        1u128 << 127,
    ] {
        println!("load u128 {v:#034x} -> {:#034x}", roundtrip_u128(v));
    }
    for v in [0i128, -1, i128::MIN, i128::MAX, -(1i128 << 64), 1i128 << 100] {
        println!("store/load i128 {v} -> {}", store_then_load_i128(v));
    }
}
