//! The wider family sweep, single-file so it needs no cargo: every `core::arch::x86_64` intrinsic
//! that lowers to an x86 memory-ordering / cache-control instruction, each in its own line of
//! output. `just family` runs this under plain rustc and under cg_clif and diffs.
//!
//! `_mm_clflush` is deliberately included and is *not* covered by this candidate's patch: it needs
//! a maintainer decision (Cranelift has no cache-line-flush instruction, so it would have to be
//! either inline asm or a documented no-op), so the sweep is expected to still differ on it.

use std::arch::x86_64::*;

fn main() {
    let cell = [0u8; 64];

    unsafe {
        _mm_sfence();
        println!("sfence");
        _mm_lfence();
        println!("lfence");
        _mm_mfence();
        println!("mfence");
        _mm_pause();
        println!("pause");

        // Interleaved with real memory traffic in both directions, so the fences sit between
        // stores and loads the way they do in real code.
        let mut v = [1u64, 2, 3, 4];
        v[0] = 41;
        _mm_sfence();
        let a = v[0];
        _mm_lfence();
        v[1] = a + 1;
        _mm_mfence();
        println!("interleaved {:?}", v);

        _mm_clflush(cell.as_ptr());
        println!("clflush");
    }

    println!("family sweep done");
}
