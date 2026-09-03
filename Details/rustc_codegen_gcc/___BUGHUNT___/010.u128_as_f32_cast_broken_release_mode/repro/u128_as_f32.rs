// Minimal reproduction for issue #705 ("Cast from u128 to f32 is broken in release mode").
//
// Both literals and expected results are taken verbatim from the (currently disabled) upstream
// rustc test `tests/ui/numbers-arithmetic/u128-as-f32.rs`. Real rustc/LLVM accepts both asserts
// with `-O` (see run.sh step 0) - only this backend, with optimizations enabled, panics on the
// second assert while the printed `left`/`right` values in the panic message both show `inf`,
// exactly matching the symptom reported in the issue.
use std::hint::black_box;

fn main() {
    assert_eq!(black_box::<u128>(0xffffff00000000000000000000000000u128) as f32, f32::MAX);
    assert_eq!(black_box::<u128>(0xffffff80000000000000000000000000u128) as f32, f32::INFINITY);
    println!("ok");
}
