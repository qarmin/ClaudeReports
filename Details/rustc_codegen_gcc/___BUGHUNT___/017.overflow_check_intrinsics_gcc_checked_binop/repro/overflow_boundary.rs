// Differential test for overflow-check intrinsics (gcc_checked_binop and friends, src/int.rs).
//
// The "expected" value for every check is a `const` evaluated by rustc's own CTFE interpreter
// (completely independent of this backend's codegen), while the "actual" value is computed at
// runtime on `black_box`-ed operands so it is genuinely routed through the codegen backend under
// test (checked_add/sub/mul, overflowing_add/sub/mul and wrapping_add/sub/mul on integers all
// lower to the `add/sub/mul_with_overflow` intrinsics, i.e. `Builder::checked_binop` ->
// `gcc_checked_binop` in src/int.rs).

use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};

static FAILED: AtomicBool = AtomicBool::new(false);

macro_rules! t {
    ($ty:ty, $a:expr, $b:expr) => {{
        const A: $ty = $a;
        const B: $ty = $b;
        const EXP_ADD: Option<$ty> = A.checked_add(B);
        const EXP_SUB: Option<$ty> = A.checked_sub(B);
        const EXP_MUL: Option<$ty> = A.checked_mul(B);
        const EXP_OADD: ($ty, bool) = A.overflowing_add(B);
        const EXP_OSUB: ($ty, bool) = A.overflowing_sub(B);
        const EXP_OMUL: ($ty, bool) = A.overflowing_mul(B);
        const EXP_WADD: $ty = A.wrapping_add(B);
        const EXP_WSUB: $ty = A.wrapping_sub(B);
        const EXP_WMUL: $ty = A.wrapping_mul(B);
        const EXP_SADD: $ty = A.saturating_add(B);
        const EXP_SSUB: $ty = A.saturating_sub(B);

        let a: $ty = black_box(A);
        let b: $ty = black_box(B);

        macro_rules! check {
            ($name:literal, $actual:expr, $expected:expr) => {{
                let actual = $actual;
                let expected = $expected;
                if actual != expected {
                    println!(
                        "FAIL {}::{}({:?}, {:?}) = {:?}, expected {:?}",
                        stringify!($ty),
                        $name,
                        a,
                        b,
                        actual,
                        expected
                    );
                    FAILED.store(true, Ordering::SeqCst);
                }
            }};
        }

        check!("checked_add", a.checked_add(b), EXP_ADD);
        check!("checked_sub", a.checked_sub(b), EXP_SUB);
        check!("checked_mul", a.checked_mul(b), EXP_MUL);
        check!("overflowing_add", a.overflowing_add(b), EXP_OADD);
        check!("overflowing_sub", a.overflowing_sub(b), EXP_OSUB);
        check!("overflowing_mul", a.overflowing_mul(b), EXP_OMUL);
        check!("wrapping_add", a.wrapping_add(b), EXP_WADD);
        check!("wrapping_sub", a.wrapping_sub(b), EXP_WSUB);
        check!("wrapping_mul", a.wrapping_mul(b), EXP_WMUL);
        check!("saturating_add", a.saturating_add(b), EXP_SADD);
        check!("saturating_sub", a.saturating_sub(b), EXP_SSUB);
    }};
}

macro_rules! cross {
    ($ty:ty, [$($v:expr),* $(,)?]) => {
        cross!(@row $ty, [$($v),*], [$($v),*]);
    };
    (@row $ty:ty, [$($a:expr),*], $bs:tt) => {
        $(
            cross!(@col $ty, $a, $bs);
        )*
    };
    (@col $ty:ty, $a:expr, [$($b:expr),*]) => {
        $(
            t!($ty, $a, $b);
        )*
    };
}

fn main() {
    cross!(
        i8,
        [0, 1, -1, i8::MAX, i8::MIN, i8::MAX - 1, i8::MIN + 1, i8::MAX / 2, i8::MIN / 2]
    );
    cross!(
        i16,
        [0, 1, -1, i16::MAX, i16::MIN, i16::MAX - 1, i16::MIN + 1, i16::MAX / 2, i16::MIN / 2]
    );
    cross!(
        i32,
        [0, 1, -1, i32::MAX, i32::MIN, i32::MAX - 1, i32::MIN + 1, i32::MAX / 2, i32::MIN / 2]
    );
    cross!(
        i64,
        [0, 1, -1, i64::MAX, i64::MIN, i64::MAX - 1, i64::MIN + 1, i64::MAX / 2, i64::MIN / 2]
    );
    cross!(
        i128,
        [0, 1, -1, i128::MAX, i128::MIN, i128::MAX - 1, i128::MIN + 1, i128::MAX / 2, i128::MIN / 2]
    );
    cross!(
        isize,
        [
            0,
            1,
            -1,
            isize::MAX,
            isize::MIN,
            isize::MAX - 1,
            isize::MIN + 1,
            isize::MAX / 2,
            isize::MIN / 2
        ]
    );

    cross!(u8, [0, 1, u8::MAX, u8::MAX - 1, u8::MAX / 2]);
    cross!(u16, [0, 1, u16::MAX, u16::MAX - 1, u16::MAX / 2]);
    cross!(u32, [0, 1, u32::MAX, u32::MAX - 1, u32::MAX / 2]);
    cross!(u64, [0, 1, u64::MAX, u64::MAX - 1, u64::MAX / 2]);
    cross!(u128, [0, 1, u128::MAX, u128::MAX - 1, u128::MAX / 2]);
    cross!(usize, [0, 1, usize::MAX, usize::MAX - 1, usize::MAX / 2]);

    if FAILED.load(Ordering::SeqCst) {
        println!("OVERALL: FAIL");
        std::process::exit(1);
    } else {
        println!("OVERALL: PASS");
    }
}
