// LLVM-vs-cg_clif differential repro for the x86 cache-line-flush intrinsics.
//
// `_mm_clflush` (stable since 1.27, SSE2 baseline) lowers to `llvm.x86.sse2.clflush` and
// `_mm_clflushopt` lowers to `llvm.x86.clflushopt`. Neither has an arm in
// src/intrinsics/llvm_x86.rs, so both land in that file's `_ =>` catch-all: a compile-time
// warning plus `codegen_panic_nounwind`, i.e. the built program aborts the first time it
// executes a flush.
//
// Must be built at opt-level 0: at -O, rustc's MIR GVN can const-fold intrinsic calls through
// the const-eval interpreter and the backend is never exercised.
#![feature(simd_x86_clflushopt, clflushopt_target_feature)]

use std::arch::x86_64::{_mm_clflush, _mm_clflushopt};
use std::sync::atomic::{Ordering, fence};

#[inline(never)]
fn part1_clflush_basic() {
    let mut buf = [0u8; 256];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = i as u8;
    }
    unsafe {
        _mm_clflush(buf.as_ptr());
        _mm_clflush(buf.as_ptr().add(64));
        _mm_clflush(buf.as_ptr().add(128));
        _mm_clflush(buf.as_ptr().add(192));
    }
    let sum: u32 = buf.iter().map(|&b| u32::from(b)).sum();
    println!("part1 clflush 4 lines: sum={sum}");
}

#[target_feature(enable = "clflushopt")]
#[inline(never)]
unsafe fn flushopt(p: *const u8) {
    _mm_clflushopt(p);
}

#[inline(never)]
fn part2_clflushopt_basic() {
    let mut buf = [0u8; 256];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (255 - i) as u8;
    }
    unsafe {
        flushopt(buf.as_ptr());
        flushopt(buf.as_ptr().add(64));
    }
    fence(Ordering::SeqCst);
    let sum: u32 = buf.iter().map(|&b| u32::from(b)).sum();
    println!("part2 clflushopt 2 lines: sum={sum}");
}

// The lowering pins the pointer into rax. Keep several other values live across the flush so a
// clobbered/unsaved register would show up as a wrong sum rather than as nothing at all.
#[inline(never)]
fn part3_register_pressure() {
    let heap = vec![1u64; 512];
    let mut a: u64 = 0x0123_4567_89ab_cdef;
    let mut b: u64 = 0xfedc_ba98_7654_3210;
    let mut c: u64 = 1;
    let mut d: u64 = 2;
    let mut e: u64 = 3;
    let mut acc: u64 = 0;
    for i in 0..512usize {
        let p = unsafe { heap.as_ptr().add(i) } as *const u8;
        unsafe {
            _mm_clflush(p);
        }
        a = a.rotate_left(1) ^ i as u64;
        b = b.wrapping_mul(3).wrapping_add(i as u64);
        c = c.wrapping_add(a);
        d ^= b;
        e = e.wrapping_add(c ^ d);
        acc = acc.wrapping_add(heap[i]).wrapping_add(e);
    }
    println!("part3 pressure: a={a:#x} b={b:#x} c={c:#x} d={d:#x} e={e:#x} acc={acc:#x}");
}

// Flush a line, then write and read it back: the value must survive a flush.
#[inline(never)]
fn part4_flush_then_readback() {
    let mut boxed = vec![0u32; 64].into_boxed_slice();
    for (i, v) in boxed.iter_mut().enumerate() {
        *v = (i as u32).wrapping_mul(2_654_435_761);
    }
    let base = boxed.as_ptr() as *const u8;
    let mut off = 0usize;
    while off < 64 * 4 {
        unsafe {
            _mm_clflush(base.add(off));
        }
        off += 64;
    }
    fence(Ordering::SeqCst);
    for (i, v) in boxed.iter_mut().enumerate() {
        *v = v.wrapping_add(i as u32);
    }
    off = 0;
    while off < 64 * 4 {
        unsafe {
            _mm_clflush(base.add(off));
        }
        off += 64;
    }
    let sum: u64 = boxed.iter().map(|&v| u64::from(v)).sum();
    println!("part4 readback: sum={sum} first={} last={}", boxed[0], boxed[63]);
}

// Interleave both flavours plus an unaligned pointer and a pointer to a stack local.
#[inline(never)]
fn part5_mixed() {
    let local: u64 = 0xdead_beef_cafe_babe;
    let buf = vec![0xa5u8; 4096];
    unsafe {
        _mm_clflush(&local as *const u64 as *const u8);
        _mm_clflush(buf.as_ptr().add(1));
        flushopt(buf.as_ptr().add(1));
        _mm_clflush(buf.as_ptr().add(4095));
        flushopt(buf.as_ptr().add(4095));
        for i in (0..4096).step_by(64) {
            flushopt(buf.as_ptr().add(i));
        }
        _mm_clflush(&local as *const u64 as *const u8);
    }
    fence(Ordering::SeqCst);
    println!("part5 mixed: local={local:#x} buf0={} buf4095={}", buf[0], buf[4095]);
}

fn main() {
    part1_clflush_basic();
    part2_clflushopt_basic();
    part3_register_pressure();
    part4_flush_then_readback();
    part5_mixed();
    println!("all parts done");
}
