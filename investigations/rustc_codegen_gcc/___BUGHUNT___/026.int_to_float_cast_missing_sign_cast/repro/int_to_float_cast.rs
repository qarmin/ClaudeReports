// Compiler:
//
// Run-time:
//   status: 0

// Repro for candidate 026 (int_to_float_cast_missing_sign_cast), found while looking for more
// instances of this workspace's dominant "GCC-declared-signedness-vs-Rust-semantic-signedness"
// bug family - the same family already confirmed for scalar division (fixed upstream, 6f389ff),
// atomic fetch_max/fetch_min (candidate 004), type_uint_from_ty (candidate 006), and lshr/ashr
// (candidate 018).
//
// `CodegenCx::int_to_float_cast` (src/int.rs) is the shared implementation behind both
// `Builder::uitofp` (unsigned-int-to-float) and `Builder::sitofp` (signed-int-to-float). For a
// *native* GCC int type, it returns a plain `new_cast(None, value, dest_typ)` with no attempt to
// force `value`'s declared sign to match the `signed: bool` flag the caller passed in - unlike
// `division_operation`/`gcc_lshr`/`gcc_ashr`, which all force the operand(s) to the
// caller-specified correct sign before the native operation. GCC's native int-to-float cast
// follows the operand's *declared* GCC type sign (the same GCC-vs-LLVM semantic gap behind every
// other confirmed bug in this family), so whenever a value's GCC-declared sign ends up "wrong"
// relative to what the Rust-level cast actually needs - e.g. after a pointer-to-integer
// `transmute`, which is already known (candidate 018) to bridge through a signed `isize`
// GCC-level type - `uitofp`/`sitofp` silently converts using the wrong sign interpretation.

use std::hint::black_box;
use std::mem::transmute;

fn main() {
    let pointer = black_box(usize::MAX) as *const ();

    // `unsigned` is a semantically-unsigned `usize`, but its declared GCC type after the
    // transmute ends up signed (see candidate 018's own root-cause instrumentation) - converting
    // it to `f64` must still treat it as unsigned (a huge positive value), not as -1.
    let unsigned: usize = unsafe { transmute(pointer) };
    let as_float = unsigned as f64;
    assert_eq!(as_float, usize::MAX as f64, "uitofp mismatch via transmute");

    // Mirror-image case: a semantically-signed `isize` converted to float must still be -1.0.
    let signed: isize = unsafe { transmute(pointer) };
    let as_float_signed = signed as f64;
    assert_eq!(as_float_signed, -1isize as f64, "sitofp mismatch via transmute");

    // Boundary values not reached through a transmute at all, to confirm the fix does not
    // regress the ordinary, already-correctly-signed case.
    assert_eq!(black_box(u64::MAX) as f64, u64::MAX as f64);
    assert_eq!(black_box(i64::MIN) as f64, i64::MIN as f64);
    assert_eq!(black_box(0u32) as f32, 0.0f32);
    assert_eq!(black_box(-1i32) as f32, -1.0f32);
}
