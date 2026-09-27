// Candidate 018: `_mm256_cvtps_ph` (llvm.x86.vcvtps2ph.256) has no arm in cg_clif's
// src/intrinsics/llvm_x86.rs, so it falls into that file's `_ =>` catch-all: a compile-time
// warning plus `codegen_panic_nounwind`. The crate builds "successfully" and the real binary
// aborts the first time the intrinsic is reached. Its 128-bit sibling `_mm_cvtps_ph`
// (llvm.x86.vcvtps2ph.128) does have an arm and works, so the two halves of the same F16C
// conversion behave differently.
//
// Built and run with plain rustc (LLVM = the oracle) and with dist/cargo-clif; the outputs are
// compared. Everything is printed as raw f16 bit patterns so rounding differences cannot hide.

#![cfg(target_arch = "x86_64")]

use std::arch::x86_64::*;

/// Boundary-value inputs: exact powers of two, ties, the f16 overflow threshold (65520 rounds to
/// inf), f16 subnormal range, zeroes/signed zeroes, inf/NaN, and f32 values that are exactly
/// halfway between two f16s.
const INPUTS: [[f32; 8]; 8] = [
    [1.0, 2.0, 3.0, 4.0, 0.5, -0.5, 65504.0, 1.0 / 3.0],
    [0.0, -0.0, 1.0e-8, -1.0e-8, 6.0e-8, 5.960_464_5e-8, 1.0e-5, 6.103_515_6e-5],
    [65519.0, 65520.0, 65536.0, 1.0e30, -65519.0, -65520.0, -1.0e30, 131_008.0],
    [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        -f32::NAN,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
    ],
    [1.000_976_5, 1.000_488_3, 1.001_464_8, 2.000_976_6, 0.999_511_72, 1.999_023_4, 1.0e-7, 3.0e-8],
    [0.1, 0.2, 0.3, 0.7, 1.5, 2.5, -1.5, -2.5],
    [1023.5, 1024.5, 2047.0, 2049.0, -2049.0, 4097.0, 8193.0, 16385.0],
    [
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x8000_0001),
        f32::from_bits(0x7f80_0001),
        f32::from_bits(0xff80_0001),
        f32::from_bits(0x3fff_ffff),
        f32::from_bits(0x477f_efff),
        f32::from_bits(0x477f_f000),
        f32::from_bits(0x3800_0000),
    ],
];

#[inline(never)]
unsafe fn bits128(v: __m128i) -> [u16; 8] {
    let mut o = [0u16; 8];
    _mm_storeu_si128(o.as_mut_ptr().cast::<__m128i>(), v);
    o
}

/// The bug: the 256-bit half-float conversion. `IMM8` is the rounding control
/// (0 = nearest-even, 1 = down, 2 = up, 3 = truncate, 4 = use MXCSR).
#[inline(never)]
unsafe fn part1_ps2ph_256<const IMM8: i32>() {
    for (n, inp) in INPUTS.iter().enumerate() {
        let a = _mm256_loadu_ps(inp.as_ptr());
        println!("ps2ph.256 imm={IMM8} in={n} -> {:04x?}", bits128(_mm256_cvtps_ph::<IMM8>(a)));
    }
}

/// The control: the 128-bit sibling, which cg_clif already implements. Run on the same data, as
/// two halves, so its result must equal part1's lane-for-lane.
#[inline(never)]
unsafe fn part2_ps2ph_128<const IMM8: i32>() {
    for (n, inp) in INPUTS.iter().enumerate() {
        let lo = _mm_loadu_ps(inp.as_ptr());
        let hi = _mm_loadu_ps(inp.as_ptr().add(4));
        println!(
            "ps2ph.128 imm={IMM8} in={n} -> {:04x?} {:04x?}",
            bits128(_mm_cvtps_ph::<IMM8>(lo)),
            bits128(_mm_cvtps_ph::<IMM8>(hi))
        );
    }
}

/// The other direction of the same F16C pair, both widths - already implemented, included so a
/// regression in either shows up here too.
#[inline(never)]
unsafe fn part3_ph2ps() {
    let h = _mm_setr_epi16(0x3c00, 0x4000, 0x4200, 0x4400, 0x3800, -0x4800, 0x7bff, 0x3555);
    let mut f4 = [0f32; 4];
    _mm_storeu_ps(f4.as_mut_ptr(), _mm_cvtph_ps(h));
    let mut f8 = [0f32; 8];
    _mm256_storeu_ps(f8.as_mut_ptr(), _mm256_cvtph_ps(h));
    println!("ph2ps.128 -> {f4:?}");
    println!("ph2ps.256 -> {f8:?}");
}

/// Repeated use inside one function plus a round-trip, to catch a lowering that clobbers its own
/// input register or reuses a stack slot across two uses.
#[inline(never)]
unsafe fn part4_repeat_and_roundtrip() {
    let a = _mm256_setr_ps(1.000_976_5, 2.000_976_6, 0.999_511_72, 1.999_023_4, 0.1, 0.2, 0.3, 0.7);
    let mut acc = 0u64;
    for _ in 0..3 {
        acc = acc.wrapping_add(bits128(_mm256_cvtps_ph::<4>(a)).iter().map(|&x| u64::from(x)).sum());
    }
    println!("repeat acc={acc}");

    let out = bits128(_mm256_cvtps_ph::<0>(a));
    let mut input_after = [0f32; 8];
    _mm256_storeu_ps(input_after.as_mut_ptr(), a);
    println!("input-after-use {input_after:?} out {out:04x?}");

    let mut back = [0f32; 8];
    _mm256_storeu_ps(back.as_mut_ptr(), _mm256_cvtph_ps(_mm256_cvtps_ph::<0>(a)));
    println!("roundtrip {back:?}");
}

fn main() {
    unsafe {
        // The very first call is the one that aborts under unpatched cg_clif.
        part1_ps2ph_256::<0>();
        part1_ps2ph_256::<1>();
        part1_ps2ph_256::<2>();
        part1_ps2ph_256::<3>();
        part1_ps2ph_256::<4>();
        part2_ps2ph_128::<0>();
        part2_ps2ph_128::<1>();
        part2_ps2ph_128::<2>();
        part2_ps2ph_128::<3>();
        part2_ps2ph_128::<4>();
        part3_ph2ps();
        part4_repeat_and_roundtrip();
    }
    println!("done");
}
