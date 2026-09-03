#![feature(core_intrinsics)]
#![allow(internal_features)]

use std::intrinsics::{AtomicOrdering as O, *};

#[repr(align(16))]
struct A(u128);

macro_rules! rmw {
    ($name:ident, $f:ident, $init:expr, $arg:expr) => {{
        let mut c = A($init);
        let p = &mut c.0 as *mut u128;
        let old = unsafe { $f::<u128, { O::SeqCst }>(p, $arg) };
        println!("{:<6} old={:#034x} now={:#034x}", stringify!($name), old, c.0);
    }};
}

macro_rules! rmw2 {
    ($name:ident, $f:ident, $init:expr, $arg:expr) => {{
        let mut c = A($init);
        let p = &mut c.0 as *mut u128;
        let old = unsafe { $f::<u128, u128, { O::SeqCst }>(p, $arg) };
        println!("{:<6} old={:#034x} now={:#034x}", stringify!($name), old, c.0);
    }};
}

fn main() {
    let vals: [u128; 4] = [
        0,
        u128::MAX,
        1u128 << 64,
        0x1122334455667788_99aabbccddeeff00,
    ];
    for &v in &vals {
        let mut c = A(v);
        let p = &mut c.0 as *mut u128;
        unsafe {
            println!("load  {:#034x}", atomic_load::<u128, { O::SeqCst }, false>(p));
            atomic_store::<u128, { O::SeqCst }, false>(p, !v);
            println!("store {:#034x}", c.0);
            let (old, ok) = atomic_cxchg::<u128, { O::SeqCst }, { O::SeqCst }>(p, !v, v);
            println!("cas   {:#034x} {} -> {:#034x}", old, ok, c.0);
            let (old, ok) = atomic_cxchg::<u128, { O::SeqCst }, { O::SeqCst }>(p, !v, 7);
            println!("cas!  {:#034x} {} -> {:#034x}", old, ok, c.0);
            let (old, ok) = atomic_cxchgweak::<u128, { O::SeqCst }, { O::SeqCst }>(p, v, 9);
            println!("casw  {:#034x} {} -> {:#034x}", old, ok, c.0);
        }
    }
    for &v in &vals {
        rmw!(xchg, atomic_xchg, v, 3);
        rmw2!(xadd, atomic_xadd, v, 3);
        rmw2!(xsub, atomic_xsub, v, 3);
        rmw2!(and, atomic_and, v, 0xff00ff);
        rmw2!(or, atomic_or, v, 0xff00ff);
        rmw2!(xor, atomic_xor, v, 0xff00ff);
        rmw2!(nand, atomic_nand, v, 0xff00ff);
        rmw!(umax, atomic_umax, v, 1u128 << 100);
        rmw!(umin, atomic_umin, v, 1u128 << 100);
    }
    // signed min/max on i128
    for &v in &vals {
        let mut c = A(v);
        let p = &mut c.0 as *mut u128 as *mut i128;
        unsafe {
            let old = atomic_max::<i128, { O::SeqCst }>(p, -5);
            println!("smax  old={} now={}", old, *p);
            let old = atomic_min::<i128, { O::SeqCst }>(p, -5);
            println!("smin  old={} now={}", old, *p);
        }
    }
}
