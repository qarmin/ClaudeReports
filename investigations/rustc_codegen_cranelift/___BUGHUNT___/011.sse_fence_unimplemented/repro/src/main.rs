//! `_mm_sfence` / `_mm_lfence` / `_mm_mfence` (`core::arch::x86_64`, stable since 1.27) lower to
//! `llvm.x86.sse.sfence` / `llvm.x86.sse2.lfence` / `llvm.x86.sse2.mfence`. None of the three has
//! an arm in cg_clif's `src/intrinsics/llvm_x86.rs`, so all three fall into the file's `_ =>`
//! catch-all: a compile-time warning plus `codegen_panic_nounwind`, i.e. the built program aborts
//! at run time the first time a fence is executed.
//!
//! Notably `_mm_pause` ("llvm.x86.sse2.pause") *is* handled, so the gap is specifically the three
//! memory fences.
//!
//! This matters beyond "an intrinsic is missing": stdarch's own documentation for every
//! `_mm*_stream_*` non-temporal store says a `_mm_sfence` must be executed by the storing thread
//! before the stores are guaranteed visible. So the *correct* way to use non-temporal stores is
//! exactly what aborts here - `part2_nontemporal_store_then_sfence` below is that pattern, and the
//! scalar `_mm_stream_si32` part of it already works under cg_clif.
//!
//! LLVM (the oracle) prints all four sections and exits 0.

// The fence intrinsics are safe fns in current stdarch; keep the `unsafe` blocks anyway so the
// same file also builds on older toolchains where they were `unsafe fn`.
#![allow(unused_unsafe)]

use std::arch::x86_64::*;

fn part1_bare_fences() {
    unsafe {
        _mm_sfence();
        println!("part1: sfence ok");
        _mm_lfence();
        println!("part1: lfence ok");
        _mm_mfence();
        println!("part1: mfence ok");
        // Implemented already - included so the output shows which member of the family differs.
        _mm_pause();
        println!("part1: pause ok");
    }
}

fn part2_nontemporal_store_then_sfence() {
    let mut buf = [0i32; 8];
    unsafe {
        for (i, slot) in buf.iter_mut().enumerate() {
            _mm_stream_si32(slot as *mut i32, (i as i32) * 10 + 1);
        }
        // stdarch: "a call to _mm_sfence must be performed by the thread that used the intrinsic"
        _mm_sfence();
    }
    println!("part2: {buf:?}");
}

/// The fences are real barriers, not no-ops: exercise them on a live producer/consumer pair so a
/// lowering that silently dropped them would still be running the same code path a real user does.
fn part3_cross_thread_release_acquire() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    const N: usize = 64;
    let data = Arc::new(std::sync::Mutex::new([0u32; N]));
    let flag = Arc::new(AtomicBool::new(false));

    let producer = {
        let data = Arc::clone(&data);
        let flag = Arc::clone(&flag);
        std::thread::spawn(move || {
            {
                let mut guard = data.lock().unwrap();
                for (i, slot) in guard.iter_mut().enumerate() {
                    *slot = (i as u32) ^ 0xa5a5;
                }
            }
            unsafe { _mm_sfence() };
            flag.store(true, Ordering::Relaxed);
        })
    };

    while !flag.load(Ordering::Relaxed) {
        unsafe { _mm_pause() };
    }
    unsafe { _mm_lfence() };
    let observed = *data.lock().unwrap();
    producer.join().unwrap();

    let mut sum: u64 = 0;
    for (i, v) in observed.iter().enumerate() {
        assert_eq!(*v, (i as u32) ^ 0xa5a5, "producer store not observed at index {i}");
        sum += u64::from(*v);
    }
    println!("part3: checksum {sum}");
}

/// Same three fences reached through non-inlined, separately-monomorphised call sites, so a fix
/// that only happened to work for the one shape above is visible.
fn part4_fences_in_a_loop() {
    #[inline(never)]
    fn store_fence() {
        unsafe { _mm_sfence() }
    }
    #[inline(never)]
    fn load_fence() {
        unsafe { _mm_lfence() }
    }
    #[inline(never)]
    fn full_fence() {
        unsafe { _mm_mfence() }
    }

    let mut acc = 0u64;
    for i in 0..1000u64 {
        store_fence();
        acc = acc.wrapping_add(i);
        load_fence();
        acc ^= i << 3;
        full_fence();
    }
    println!("part4: acc {acc}");
}

fn main() {
    part1_bare_fences();
    part2_nontemporal_store_then_sfence();
    part3_cross_thread_release_acquire();
    part4_fences_in_a_loop();
    println!("ALL SECTIONS COMPLETED");
}
