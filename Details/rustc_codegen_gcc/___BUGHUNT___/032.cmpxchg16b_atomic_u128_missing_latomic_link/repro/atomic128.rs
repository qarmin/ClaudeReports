// Repro for candidate 032 (cmpxchg16b_atomic_u128_missing_latomic_link).
//
// `AtomicU128`/`AtomicI128::compare_exchange` are NOT reachable on x86_64-unknown-linux-gnu at
// all: `std`/`core` gate those types behind `#[cfg(target_has_atomic_load_store = "128")]`, and
// rustc's own built-in target spec for this triple sets `max-atomic-width: 64` (verified this
// session with `rustc -Z unstable-options --print target-spec-json --target
// x86_64-unknown-linux-gnu`), independently of any `-C target-feature` flag. That part of the
// original candidate's premise does not hold for this target.
//
// What *is* real and reachable on this target: the same underlying compiler intrinsics that
// `AtomicU128::compare_exchange`/`fetch_add` desugar to on any target where the type IS available
// (aarch64-unknown-linux-gnu, x86_64-apple-darwin - both have `max-atomic-width: 128`) can be
// called directly via `core::intrinsics::atomic_cxchg`/`atomic_xadd` with `T = u128`, gated only
// by the internal, discouraged `#![feature(core_intrinsics)]` (not by `target_has_atomic`). This
// drives the exact same codegen path in `src/builder.rs`'s `compare_exchange`/`atomic_rmw` that
// `AtomicU128`/`AtomicI128` would use on a target where they're available, through a real rustc
// compilation and a real link, with no hand-built GCC IR or object-file stand-in.
#![feature(core_intrinsics)]
#![allow(internal_features)]
use std::intrinsics::{AtomicOrdering, atomic_cxchg, atomic_xadd};

fn main() {
    let mut x: u128 = 1;
    let (old, ok) = unsafe {
        atomic_cxchg::<u128, { AtomicOrdering::SeqCst }, { AtomicOrdering::SeqCst }>(
            &mut x as *mut u128,
            1u128,
            2u128,
        )
    };
    println!("cmpxchg: old={old} ok={ok} x={x}");
    assert_eq!((old, ok, x), (1, true, 2));

    let mut y: u128 = 10;
    let prev = unsafe {
        atomic_xadd::<u128, u128, { AtomicOrdering::SeqCst }>(&mut y as *mut u128, 5u128)
    };
    println!("fetch_add: prev={prev} y={y}");
    assert_eq!((prev, y), (10, 15));

    println!("OK");
}
