// `simd_div` / `simd_rem` on a `#[repr(simd)]` vector with 128-bit lanes.
//
// cg_clif lowers these lane-by-lane straight to Cranelift's `sdiv`/`udiv`/`srem`/`urem`. No
// Cranelift backend has an `I128` lowering for those four instructions, so `define_function`
// returns `Err(ModuleError::Compilation(Unsupported("... sdiv.i128 ...")))`, which `src/base.rs`
// turns into `bug!()` - an ICE ("the compiler unexpectedly panicked. This is a bug").
//
// LLVM compiles the same source and prints the correct values (it emits the same
// __divti3/__udivti3/__modti3/__umodti3 compiler-builtins calls that cg_clif already uses for
// *scalar* 128-bit division).
#![feature(repr_simd, core_intrinsics)]
#![allow(internal_features)]

use std::hint::black_box;
use std::intrinsics::simd::{simd_div, simd_rem};

macro_rules! simd_case {
    ($vec:ident, $check:ident, $ty:ty, $n:literal) => {
        #[repr(simd)]
        #[derive(Copy, Clone)]
        struct $vec([$ty; $n]);

        impl $vec {
            #[inline(never)]
            fn div(self, rhs: Self) -> Self {
                unsafe { simd_div(self, rhs) }
            }

            #[inline(never)]
            fn rem(self, rhs: Self) -> Self {
                unsafe { simd_rem(self, rhs) }
            }

            // Reading `self.0` directly is banned for `#[repr(simd)]` types (MCP#838), and
            // `simd_extract` needs a const index, so copy the whole vector out instead.
            fn lanes(self) -> [$ty; $n] {
                unsafe { std::mem::transmute_copy::<Self, [$ty; $n]>(&self) }
            }
        }

        fn $check(a: [$ty; $n], b: [$ty; $n]) {
            let va = black_box($vec(a));
            let vb = black_box($vec(b));
            println!("{} ({} x {})", stringify!($vec), stringify!($ty), $n);
            println!("  a   = {:?}", a);
            println!("  b   = {:?}", b);
            println!("  div = {:?}", va.div(vb).lanes());
            println!("  rem = {:?}", va.rem(vb).lanes());
        }
    };
}

simd_case!(I128x1, check_i128x1, i128, 1);
simd_case!(I128x2, check_i128x2, i128, 2);
simd_case!(I128x4, check_i128x4, i128, 4);
simd_case!(I128x8, check_i128x8, i128, 8);
simd_case!(I128x16, check_i128x16, i128, 16);
simd_case!(I128x32, check_i128x32, i128, 32);
simd_case!(U128x1, check_u128x1, u128, 1);
simd_case!(U128x2, check_u128x2, u128, 2);
simd_case!(U128x4, check_u128x4, u128, 4);
simd_case!(U128x8, check_u128x8, u128, 8);
simd_case!(U128x16, check_u128x16, u128, 16);
simd_case!(U128x32, check_u128x32, u128, 32);

fn nonzero_i(i: usize) -> i128 {
    let d = i as i128 - 8;
    if d == 0 { 1 } else { d }
}

fn main() {
    const IMAX: i128 = i128::MAX;
    const IMIN: i128 = i128::MIN;
    const UMAX: u128 = u128::MAX;

    // Note: `i128::MIN / -1` is overflow (UB) in Rust, so that exact pair is deliberately absent.
    check_i128x1([IMIN], [3]);
    check_i128x2([IMIN, IMAX], [IMAX, -1]);
    check_i128x4([IMIN + 1, IMAX, -(1i128 << 100), 0], [3, -7, 1i128 << 64, IMAX]);
    check_i128x8(
        [IMIN, IMAX, -1, 0, 1i128 << 126, -(1i128 << 126), 1234567890123456789, -3],
        [IMAX, 2, IMIN, IMAX, 3, -3, 1i128 << 64, 1],
    );

    check_u128x1([UMAX], [3]);
    check_u128x2([UMAX, 1u128 << 127], [3, UMAX]);
    check_u128x4(
        [UMAX, 0, 1u128 << 64, 340282366920938463463374607431768211],
        [1, UMAX, 3, 1u128 << 100],
    );
    check_u128x8(
        [UMAX, 0, 1, 1u128 << 127, (1u128 << 127) - 1, 7, 1u128 << 64, 99999999999999999999],
        [UMAX, UMAX, UMAX, 2, 3, 1u128 << 100, 1u128 << 32, 1_000_000_007],
    );

    // Wider lane counts, same bug class: the vector no longer fits any register, so the lanes are
    // handled purely through memory - but each lane's division is still an `sdiv.i128`.
    check_i128x16(std::array::from_fn(|i| (i as i128 - 8) << 90), std::array::from_fn(nonzero_i));
    check_i128x32(
        std::array::from_fn(|i| (i as i128 - 16) << 80),
        std::array::from_fn(|i| nonzero_i(i % 16)),
    );
    check_u128x16(
        std::array::from_fn(|i| (i as u128) << 100 | 7),
        std::array::from_fn(|i| i as u128 + 1),
    );
    check_u128x32(std::array::from_fn(|i| UMAX >> i), std::array::from_fn(|i| i as u128 + 1));
}
