// Compiler:
//
// Run-time:
//   status: 0

// Repro for candidate 018 (gcc_lshr_ashr_missing_sign_cast), following this project's own
// tests/run/ptr_to_int_div.rs convention for the sibling division-sign bug fixed by 6f389ff.
//
// `Builder::lshr`/`Builder::ashr` (src/builder.rs) both delegate straight to the shared
// `gcc_lshr` (src/int.rs) with no sign-correcting cast, unlike `division_operation` which forces
// the GCC type to the correct sign before dividing. GCC's native `>>` follows the *declared*
// sign of the operand's GCC type (unlike LLVM, where integers carry no sign in the type itself),
// so whenever a value's GCC type ends up with the "wrong" sign relative to what the Rust-level
// shift actually needs - e.g. after a pointer-to-integer `transmute`, which bridges through a
// signed `isize` register type in `ptrtoint` - the native shift silently does the wrong kind of
// shift (arithmetic/sign-filling instead of logical/zero-filling, or vice versa).

use std::hint::black_box;
use std::mem::transmute;

fn main() {
    let pointer = black_box(usize::MAX) as *const ();

    // `unsigned` is a semantically-unsigned `usize`, but its declared GCC type after the
    // transmute ends up signed (`ptrtoint` bridges through `isize`) - `>>` on it must still be
    // a logical (zero-filling) shift.
    let unsigned = unsafe { transmute::<*const (), usize>(pointer) };
    assert_eq!(unsigned >> black_box(2), usize::MAX >> 2);

    // Mirror-image case: a semantically-signed `isize` shifted arithmetically (sign-filling).
    let signed = unsafe { transmute::<*const (), isize>(pointer) };
    assert_eq!(signed >> black_box(2), -1isize >> 2);

    // Boundary values not reached through a transmute at all, to confirm the fix does not
    // regress the ordinary, already-correctly-signed case.
    assert_eq!(black_box(i64::MIN) >> black_box(4), i64::MIN >> 4);
    assert_eq!(black_box(0xFFFF_FFFFu32) >> black_box(4), 0x0FFF_FFFFu32);
    assert_eq!(black_box(-8i64) >> black_box(1), -4i64);
}
