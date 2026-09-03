// Compiler:
//
// Run-time:
//   status: 0

// Repro for candidate 022 (gcc_icmp_native_sign_dedup_check): mirrors candidate 018's
// transmute-based sign-corruption trick (a pointer-to-integer transmute bridges through a
// signed `isize` register type in `ptrtoint`, so the resulting value's *declared* GCC type ends
// up signed even for a semantically-unsigned `usize`) but exercises `gcc_icmp`'s native
// comparison path (src/int.rs) instead of `gcc_lshr`/`gcc_ashr`. Unlike the shift path,
// `gcc_icmp` already re-derives the correct sign from the requested `IntPredicate` before
// comparing (casting both operands to `a_type.to_unsigned()`/`to_signed()`), so this is expected
// to already be correct - this repro checks that claim empirically.

use std::hint::black_box;
use std::mem::transmute;

fn main() {
    let pointer = black_box(usize::MAX) as *const ();

    // `unsigned` is semantically a `usize` (all bits set), but its declared GCC type after the
    // transmute ends up signed (same root cause as candidate 018). Unsigned comparisons against
    // it must still treat it as the maximum unsigned value, not as -1.
    let unsigned = unsafe { transmute::<*const (), usize>(pointer) };
    assert!(unsigned > black_box(0usize));
    assert!(unsigned >= black_box(usize::MAX));
    assert!(!(unsigned < black_box(1usize)));
    assert_eq!(unsigned == usize::MAX, true);

    // Mirror-image: a semantically-signed `isize` (all bits set == -1) compared signedly.
    let signed = unsafe { transmute::<*const (), isize>(pointer) };
    assert!(signed < black_box(0isize));
    assert!(signed <= black_box(-1isize));
    assert!(!(signed > black_box(0isize)));
    assert_eq!(signed, -1isize);

    // Ordinary, already-correctly-signed boundary values, to confirm no regression.
    assert!(black_box(i64::MIN) < black_box(0i64));
    assert!(black_box(0xFFFF_FFFFu32) > black_box(0u32));
    assert!(black_box(-8i64) < black_box(-1i64));
    assert!(black_box(u64::MAX) > black_box(i64::MAX as u64));

    // Pointer comparisons (a different branch of gcc_icmp - both operands have a pointee).
    let p1 = black_box(1usize) as *const u8;
    let p2 = black_box(2usize) as *const u8;
    assert!(p1 < p2);
    assert!(p2 > p1);

    println!("all scalar/pointer checks passed");
}
