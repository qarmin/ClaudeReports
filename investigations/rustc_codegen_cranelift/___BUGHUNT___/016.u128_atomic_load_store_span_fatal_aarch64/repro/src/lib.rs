//! A `#![no_std]` library with nothing interesting in it.
//!
//! The point of the repro is not this code: it is that building it for aarch64 (or s390x, or
//! x86_64-apple-darwin) with `-Zbuild-std=core` requires `core` itself to be codegened, and
//! `core`'s `AtomicU128`/`AtomicI128` make cg_clif hard-error out with
//! "128bit atomics not yet supported".
#![no_std]
#![cfg_attr(feature = "atomic_u128", feature(integer_atomics))]

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(feature = "atomic_u128")]
pub mod atomic_u128 {
    use core::sync::atomic::{AtomicU128, Ordering};

    pub fn load(a: &AtomicU128) -> u128 {
        a.load(Ordering::SeqCst)
    }

    pub fn store(a: &AtomicU128, v: u128) {
        a.store(v, Ordering::SeqCst)
    }
}
