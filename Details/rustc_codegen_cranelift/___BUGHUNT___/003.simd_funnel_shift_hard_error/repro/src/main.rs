//! Standalone reproduction: `simd_funnel_shl` / `simd_funnel_shr` are not implemented by
//! rustc_codegen_cranelift, and the fallback arm in `src/intrinsics/simd.rs` is a hard
//! `span_err`, so any crate that reaches them fails to build outright.
//!
//! Part A uses the generic intrinsics directly (hardware independent).
//! Part B goes through the real `std::arch` AVX512 rotate / funnel-shift intrinsics, which is how
//! ordinary code reaches them - `_mm512_rol_epi32` and friends are implemented in stdarch in terms
//! of `simd_funnel_shl`. Part B only *runs* when the host actually has AVX-512, but it is compiled
//! unconditionally, and compiling is where the bug bites.
//!
//! Every value is checked against a scalar reference implementation of the funnel shift, so this
//! also catches a wrong-answer regression, not just the build failure.

#![feature(core_intrinsics, repr_simd)]
#![allow(internal_features)]

use std::intrinsics::simd::{simd_funnel_shl, simd_funnel_shr};
use std::mem::transmute;

static mut FAILURES: u32 = 0;

fn fail(msg: String) {
    println!("WRONG {msg}");
    unsafe { FAILURES += 1 };
}

macro_rules! funnel_case {
    ($name:ident, $lane:ty, $uns:ty, $bits:expr, $count:expr) => {
        mod $name {
            use super::*;

            #[repr(simd)]
            #[derive(Copy, Clone)]
            struct V([$lane; $count]);

            fn ref_shl(a: $uns, b: $uns, s: u32) -> $uns {
                let s = s % $bits;
                let lo = if s == 0 { 0 } else { b >> ($bits - s) };
                (a << s) | lo
            }

            fn ref_shr(a: $uns, b: $uns, s: u32) -> $uns {
                let s = s % $bits;
                let hi = if s == 0 { 0 } else { a << ($bits - s) };
                hi | (b >> s)
            }

            fn operand(i: usize) -> $uns {
                match i % 8 {
                    0 => 0,
                    1 => 1,
                    2 => <$uns>::MAX,
                    3 => 1 << ($bits - 1),
                    4 => <$uns>::MAX >> 1,
                    5 => !(<$uns>::MAX >> 1) | 1,
                    6 => 0x5555_5555_5555_5555u64 as $uns,
                    _ => 0xAAAA_AAAA_AAAA_AAAAu64 as $uns,
                }
            }

            /// Every shift amount 0..bits-1 lands on every lane at least once.
            pub fn run() {
                let mut a = [0 as $uns; $count];
                let mut b = [0 as $uns; $count];
                let mut s = [0 as $uns; $count];
                let mut checked = 0u32;
                for round in 0..$bits {
                    for lane in 0..$count {
                        a[lane] = operand(round + lane);
                        b[lane] = operand(round + lane + 3);
                        s[lane] = ((round + lane) % $bits) as $uns;
                    }
                    let (va, vb, vs): (V, V, V) =
                        unsafe { (transmute(a), transmute(b), transmute(s)) };
                    let got_shl: [$uns; $count] =
                        unsafe { transmute(simd_funnel_shl(va, vb, vs)) };
                    let got_shr: [$uns; $count] =
                        unsafe { transmute(simd_funnel_shr(va, vb, vs)) };
                    for lane in 0..$count {
                        let sh = s[lane] as u32;
                        let want_shl = ref_shl(a[lane], b[lane], sh);
                        let want_shr = ref_shr(a[lane], b[lane], sh);
                        if got_shl[lane] != want_shl {
                            fail(format!(
                                "simd_funnel_shl {} lane={lane} a={:x} b={:x} s={sh} got={:x} want={:x}",
                                stringify!($name),
                                a[lane],
                                b[lane],
                                got_shl[lane],
                                want_shl
                            ));
                        }
                        if got_shr[lane] != want_shr {
                            fail(format!(
                                "simd_funnel_shr {} lane={lane} a={:x} b={:x} s={sh} got={:x} want={:x}",
                                stringify!($name),
                                a[lane],
                                b[lane],
                                got_shr[lane],
                                want_shr
                            ));
                        }
                        checked += 2;
                    }
                }
                println!("ok    {} ({checked} lane results)", stringify!($name));
            }
        }
    };
}

funnel_case!(u8x16, u8, u8, 8, 16);
funnel_case!(i8x32, i8, u8, 8, 32);
funnel_case!(u16x8, u16, u16, 16, 8);
funnel_case!(i16x32, i16, u16, 16, 32);
funnel_case!(u32x4, u32, u32, 32, 4);
funnel_case!(i32x16, i32, u32, 32, 16);
funnel_case!(u64x2, u64, u64, 64, 2);
funnel_case!(i64x8, i64, u64, 64, 8);
funnel_case!(u128x2, u128, u128, 128, 2);
funnel_case!(i128x4, i128, u128, 128, 4);

// ---------------------------------------------------------------------------------------------
// Part B: the real std::arch path.

#[cfg(target_arch = "x86_64")]
mod arch {
    use super::*;
    use std::arch::x86_64::*;

    fn ref_rol32(a: u32, s: u32) -> u32 {
        a.rotate_left(s)
    }

    fn check32(name: &str, got: [u32; 16], want: [u32; 16]) {
        if got == want {
            println!("ok    {name}");
        } else {
            fail(format!("{name}\n        got  {got:08x?}\n        want {want:08x?}"));
        }
    }

    const A: [u32; 16] = [
        0x8000_0001,
        0x0000_0000,
        0xFFFF_FFFF,
        0x1234_5678,
        0x0000_0001,
        0x7FFF_FFFF,
        0xAAAA_AAAA,
        0x5555_5555,
        0xDEAD_BEEF,
        0xCAFE_BABE,
        0x0000_00FF,
        0xFF00_0000,
        0x0F0F_0F0F,
        0xF0F0_F0F0,
        0x8000_0000,
        0x0000_0002,
    ];
    const B: [u32; 16] = [
        0x1234_5678,
        0xFFFF_FFFF,
        0x0000_0001,
        0xAAAA_AAAA,
        0xCAFE_BABE,
        0x0F0F_0F0F,
        0x0000_0000,
        0x8000_0000,
        0x7FFF_FFFF,
        0x5555_5555,
        0xF0F0_F0F0,
        0x0000_00FF,
        0xDEAD_BEEF,
        0x1111_1111,
        0x0000_0003,
        0xFFFF_0000,
    ];

    /// `_mm512_rol_epi32` / `_mm512_ror_epi32` / `_mm512_rolv_epi32` are all stdarch wrappers over
    /// `simd_funnel_shl` / `simd_funnel_shr`.
    #[target_feature(enable = "avx512f")]
    unsafe fn avx512f() {
        let a: __m512i = transmute(A);

        for (name, got, sh, left) in [
            ("_mm512_rol_epi32::<0>", transmute::<_, [u32; 16]>(_mm512_rol_epi32::<0>(a)), 0, true),
            ("_mm512_rol_epi32::<1>", transmute::<_, [u32; 16]>(_mm512_rol_epi32::<1>(a)), 1, true),
            (
                "_mm512_rol_epi32::<16>",
                transmute::<_, [u32; 16]>(_mm512_rol_epi32::<16>(a)),
                16,
                true,
            ),
            (
                "_mm512_rol_epi32::<31>",
                transmute::<_, [u32; 16]>(_mm512_rol_epi32::<31>(a)),
                31,
                true,
            ),
            ("_mm512_ror_epi32::<0>", transmute::<_, [u32; 16]>(_mm512_ror_epi32::<0>(a)), 0, false),
            ("_mm512_ror_epi32::<1>", transmute::<_, [u32; 16]>(_mm512_ror_epi32::<1>(a)), 1, false),
            (
                "_mm512_ror_epi32::<31>",
                transmute::<_, [u32; 16]>(_mm512_ror_epi32::<31>(a)),
                31,
                false,
            ),
        ] {
            let eff = if left { sh } else { (32 - sh) % 32 };
            let mut want = [0u32; 16];
            for i in 0..16 {
                want[i] = ref_rol32(A[i], eff);
            }
            check32(name, got, want);
        }

        // variable, per-lane rotate amount
        let amounts: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 29, 30, 31, 15];
        let got = transmute::<_, [u32; 16]>(_mm512_rolv_epi32(a, transmute(amounts)));
        let mut want = [0u32; 16];
        for i in 0..16 {
            want[i] = ref_rol32(A[i], amounts[i]);
        }
        check32("_mm512_rolv_epi32", got, want);

        let got = transmute::<_, [u32; 16]>(_mm512_rorv_epi32(a, transmute(amounts)));
        let mut want = [0u32; 16];
        for i in 0..16 {
            want[i] = A[i].rotate_right(amounts[i]);
        }
        check32("_mm512_rorv_epi32", got, want);
    }

    /// AVX512VBMI2 `sh[lr]d` - the double-operand funnel shift, i.e. `simd_funnel_sh*` verbatim.
    #[target_feature(enable = "avx512vbmi2")]
    unsafe fn avx512vbmi2() {
        let a: __m512i = transmute(A);
        let b: __m512i = transmute(B);

        for (name, got, sh) in [
            ("_mm512_shldi_epi32::<0>", transmute::<_, [u32; 16]>(_mm512_shldi_epi32::<0>(a, b)), 0),
            ("_mm512_shldi_epi32::<1>", transmute::<_, [u32; 16]>(_mm512_shldi_epi32::<1>(a, b)), 1),
            (
                "_mm512_shldi_epi32::<31>",
                transmute::<_, [u32; 16]>(_mm512_shldi_epi32::<31>(a, b)),
                31,
            ),
        ] {
            let mut want = [0u32; 16];
            for i in 0..16 {
                want[i] = if sh == 0 { A[i] } else { (A[i] << sh) | (B[i] >> (32 - sh)) };
            }
            check32(name, got, want);
        }

        for (name, got, sh) in [
            ("_mm512_shrdi_epi32::<0>", transmute::<_, [u32; 16]>(_mm512_shrdi_epi32::<0>(a, b)), 0),
            ("_mm512_shrdi_epi32::<1>", transmute::<_, [u32; 16]>(_mm512_shrdi_epi32::<1>(a, b)), 1),
            (
                "_mm512_shrdi_epi32::<31>",
                transmute::<_, [u32; 16]>(_mm512_shrdi_epi32::<31>(a, b)),
                31,
            ),
        ] {
            // Intel's shrd concatenates as b:a (b is the high half) and keeps the low 32 bits.
            let mut want = [0u32; 16];
            for i in 0..16 {
                want[i] = if sh == 0 { A[i] } else { (B[i] << (32 - sh)) | (A[i] >> sh) };
            }
            check32(name, got, want);
        }

        let amounts: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 29, 30, 31, 15];
        let got = transmute::<_, [u32; 16]>(_mm512_shldv_epi32(a, b, transmute(amounts)));
        let mut want = [0u32; 16];
        for i in 0..16 {
            let sh = amounts[i];
            want[i] = if sh == 0 { A[i] } else { (A[i] << sh) | (B[i] >> (32 - sh)) };
        }
        check32("_mm512_shldv_epi32", got, want);
    }

    pub fn run() {
        if std::is_x86_feature_detected!("avx512f") {
            unsafe { avx512f() };
        } else {
            println!("skip  AVX512F rotates (host has no avx512f) - they were still compiled");
        }
        if std::is_x86_feature_detected!("avx512vbmi2") {
            unsafe { avx512vbmi2() };
        } else {
            println!("skip  AVX512VBMI2 funnel shifts (host has no avx512vbmi2) - still compiled");
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
mod arch {
    pub fn run() {
        println!("skip  std::arch part (not x86_64)");
    }
}

fn main() {
    u8x16::run();
    i8x32::run();
    u16x8::run();
    i16x32::run();
    u32x4::run();
    i32x16::run();
    u64x2::run();
    i64x8::run();
    u128x2::run();
    i128x4::run();
    arch::run();

    let failures = unsafe { FAILURES };
    println!("failures: {failures}");
    if failures != 0 {
        std::process::exit(1);
    }
}
