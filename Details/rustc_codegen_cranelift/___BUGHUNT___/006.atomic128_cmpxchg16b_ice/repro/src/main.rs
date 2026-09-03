//! `core::arch::x86_64::cmpxchg16b` is a stable public API (stable since 1.67). It lowers to
//! `core::intrinsics::atomic_cxchg::<u128>`, which cg_clif hands straight to Cranelift's
//! `atomic_cas` with type `I128`. Cranelift only has an `I128` lowering rule when the ISA has
//! `has_cmpxchg16b`, and cg_clif never forwards `-C target-feature` to Cranelift, so on the
//! default `x86-64` target-cpu Cranelift returns `Unsupported` and cg_clif turns that into an ICE
//! ("the compiler unexpectedly panicked. This is a bug").
//!
//! Build with `RUSTFLAGS=-Ctarget-feature=+cmpxchg16b` so that plain rustc/LLVM (the oracle)
//! compiles and runs this fine; cg_clif ICEs on the exact same invocation.

use std::sync::atomic::Ordering;

#[repr(align(16))]
struct Aligned(u128);

fn main() {
    // Boundary values, not just one arbitrary bit pattern: 0, all-ones, low/high half only,
    // and a mixed pattern, so a fix that only handles one of them is visible.
    let values: [u128; 6] = [
        0,
        u128::MAX,
        1,
        1u128 << 127,
        0xffff_ffff_ffff_ffff,
        0x1122_3344_5566_7788_99aa_bbcc_ddee_ff00,
    ];

    for &v in &values {
        let mut cell = Aligned(v);
        let p = &mut cell.0 as *mut u128;
        let replacement = !v ^ 0x5555_5555_5555_5555_aaaa_aaaa_aaaa_aaaa;

        // Success case: the comparand matches, so the cell must be replaced.
        let old = unsafe {
            std::arch::x86_64::cmpxchg16b(p, v, replacement, Ordering::SeqCst, Ordering::SeqCst)
        };
        println!("ok   old={old:#034x} now={:#034x}", cell.0);
        assert_eq!(old, v);
        assert_eq!(cell.0, replacement);

        // Failure case: the comparand no longer matches, so the cell must be left alone.
        let old = unsafe {
            std::arch::x86_64::cmpxchg16b(p, v, 0xdead, Ordering::SeqCst, Ordering::SeqCst)
        };
        println!("fail old={old:#034x} now={:#034x}", cell.0);
        assert_eq!(old, replacement);
        assert_eq!(cell.0, replacement);
    }

    println!("all cmpxchg16b cases agreed with the expected values");
}
