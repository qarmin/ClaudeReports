//! cg_clif has no lowering for `llvm.x86.sse.cmp.ss` / `llvm.x86.sse2.cmp.sd`, so every
//! `_mm_cmpXX_ss` / `_mm_cmpXX_sd` (SSE/SSE2 baseline, stable since 1.27) and every
//! `_mm_cmp_ss::<IMM>` / `_mm_cmp_sd::<IMM>` (AVX) compiles with only a warning and then aborts
//! the process the first time it is executed.
//!
//! Build with plain rustc/LLVM (the oracle) and with cg_clif and compare the output.

#![cfg(target_arch = "x86_64")]

use std::arch::x86_64::*;

fn bits4(v: __m128) -> [u32; 4] {
    let mut o = [0f32; 4];
    unsafe { _mm_storeu_ps(o.as_mut_ptr(), v) };
    o.map(f32::to_bits)
}

fn bits2(v: __m128d) -> [u64; 2] {
    let mut o = [0f64; 2];
    unsafe { _mm_storeu_pd(o.as_mut_ptr(), v) };
    o.map(f64::to_bits)
}

const F32_CASES: [([f32; 4], [f32; 4]); 6] = [
    ([1.0, 2.0, 3.0, 4.0], [1.0, 5.0, 0.0, 4.0]),
    ([2.0, 2.0, 3.0, 4.0], [1.0, 5.0, 0.0, 4.0]),
    ([f32::NAN, 1.0, f32::NAN, 0.0], [1.0, f32::NAN, f32::NAN, 0.0]),
    ([1.0, 1.0, 1.0, 1.0], [f32::NAN, 2.0, 3.0, 4.0]),
    ([-0.0, 0.0, f32::INFINITY, f32::NEG_INFINITY], [0.0, -0.0, f32::INFINITY, f32::INFINITY]),
    ([f32::NEG_INFINITY, 0.0, 0.0, 0.0], [f32::INFINITY, 0.0, 0.0, 0.0]),
];

const F64_CASES: [([f64; 2], [f64; 2]); 6] = [
    ([1.0, 2.0], [1.0, 5.0]),
    ([2.0, 2.0], [1.0, 5.0]),
    ([f64::NAN, 1.0], [1.0, f64::NAN]),
    ([1.0, 1.0], [f64::NAN, 2.0]),
    ([-0.0, f64::INFINITY], [0.0, f64::INFINITY]),
    ([f64::NEG_INFINITY, 0.0], [f64::INFINITY, 0.0]),
];

macro_rules! run_ss {
    ($name:literal, $f:ident) => {{
        for (a, b) in F32_CASES {
            let va = unsafe { _mm_set_ps(a[3], a[2], a[1], a[0]) };
            let vb = unsafe { _mm_set_ps(b[3], b[2], b[1], b[0]) };
            println!("{:<16} {:?} {:?} -> {:?}", $name, a, b, bits4(unsafe { $f(va, vb) }));
        }
    }};
}

macro_rules! run_sd {
    ($name:literal, $f:ident) => {{
        for (a, b) in F64_CASES {
            let va = unsafe { _mm_set_pd(a[1], a[0]) };
            let vb = unsafe { _mm_set_pd(b[1], b[0]) };
            println!("{:<16} {:?} {:?} -> {:?}", $name, a, b, bits2(unsafe { $f(va, vb) }));
        }
    }};
}

/// Part 1: the SSE/SSE2 baseline public API. No `target_feature` needed on x86-64 - these are
/// callable from safe stable code on every x86-64 CPU.
fn part1_public_sse() {
    println!("--- part 1: _mm_cmpXX_ss / _mm_cmpXX_sd (SSE/SSE2 baseline) ---");
    run_ss!("cmpeq_ss", _mm_cmpeq_ss);
    run_ss!("cmplt_ss", _mm_cmplt_ss);
    run_ss!("cmple_ss", _mm_cmple_ss);
    run_ss!("cmpgt_ss", _mm_cmpgt_ss);
    run_ss!("cmpge_ss", _mm_cmpge_ss);
    run_ss!("cmpneq_ss", _mm_cmpneq_ss);
    run_ss!("cmpnlt_ss", _mm_cmpnlt_ss);
    run_ss!("cmpnle_ss", _mm_cmpnle_ss);
    run_ss!("cmpngt_ss", _mm_cmpngt_ss);
    run_ss!("cmpnge_ss", _mm_cmpnge_ss);
    run_ss!("cmpord_ss", _mm_cmpord_ss);
    run_ss!("cmpunord_ss", _mm_cmpunord_ss);
    run_sd!("cmpeq_sd", _mm_cmpeq_sd);
    run_sd!("cmplt_sd", _mm_cmplt_sd);
    run_sd!("cmple_sd", _mm_cmple_sd);
    run_sd!("cmpgt_sd", _mm_cmpgt_sd);
    run_sd!("cmpge_sd", _mm_cmpge_sd);
    run_sd!("cmpneq_sd", _mm_cmpneq_sd);
    run_sd!("cmpnlt_sd", _mm_cmpnlt_sd);
    run_sd!("cmpnle_sd", _mm_cmpnle_sd);
    run_sd!("cmpngt_sd", _mm_cmpngt_sd);
    run_sd!("cmpnge_sd", _mm_cmpnge_sd);
    run_sd!("cmpord_sd", _mm_cmpord_sd);
    run_sd!("cmpunord_sd", _mm_cmpunord_sd);
}

macro_rules! imm_ss {
    ($($imm:expr),* $(,)?) => {{ $(
        for (a, b) in F32_CASES {
            let va = _mm_set_ps(a[3], a[2], a[1], a[0]);
            let vb = _mm_set_ps(b[3], b[2], b[1], b[0]);
            println!("cmp_ss::<{:#04x}> {:?} {:?} -> {:?}", $imm, a, b, bits4(_mm_cmp_ss::<$imm>(va, vb)));
        }
    )* }};
}

macro_rules! imm_sd {
    ($($imm:expr),* $(,)?) => {{ $(
        for (a, b) in F64_CASES {
            let va = _mm_set_pd(a[1], a[0]);
            let vb = _mm_set_pd(b[1], b[0]);
            println!("cmp_sd::<{:#04x}> {:?} {:?} -> {:?}", $imm, a, b, bits2(_mm_cmp_sd::<$imm>(va, vb)));
        }
    )* }};
}

/// Part 2: the AVX `_mm_cmp_ss` / `_mm_cmp_sd` form, over every predicate immediate that maps
/// onto a Cranelift `FloatCC`. The four constant predicates (0x0b/0x0f/0x1b/0x1f) are left out
/// on purpose - they are a *different*, pre-existing backend bug (a `todo!()` ICE that also
/// affects the packed `_mm_cmp_ps`/`_mm_cmp_pd`), tracked separately.
#[target_feature(enable = "avx")]
unsafe fn part2_avx_immediates() {
    println!("--- part 2: _mm_cmp_ss / _mm_cmp_sd, all 28 FloatCC-expressible immediates ---");
    imm_ss!(
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0c, 0x0d, 0x0e, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1c, 0x1d, 0x1e,
    );
    imm_sd!(
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0c, 0x0d, 0x0e, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1c, 0x1d, 0x1e,
    );
}

/// Part 3: how these actually get used - a scalar branch-free min/max/clamp over real data,
/// mask-selected with `_mm_cmplt_ss` / `_mm_cmpgt_ss`, plus a NaN-count via `_mm_cmpunord_sd`.
fn part3_realistic() {
    println!("--- part 3: branch-free scalar clamp via _mm_cmplt_ss / _mm_cmpgt_ss ---");
    let data: [f32; 12] = [
        -3.5, 0.0, 1.25, 7.5, -0.0, f32::INFINITY, 2.5, f32::NEG_INFINITY, 4.0, f32::NAN, 0.75,
        -1.0,
    ];
    let lo = 0.0f32;
    let hi = 5.0f32;
    let mut out = Vec::new();
    for &v in &data {
        unsafe {
            let x = _mm_set_ss(v);
            let vlo = _mm_set_ss(lo);
            let vhi = _mm_set_ss(hi);
            // x = if x < lo { lo } else { x }
            let m = _mm_cmplt_ss(x, vlo);
            let x = _mm_or_ps(_mm_and_ps(m, vlo), _mm_andnot_ps(m, x));
            // x = if x > hi { hi } else { x }
            let m = _mm_cmpgt_ss(x, vhi);
            let x = _mm_or_ps(_mm_and_ps(m, vhi), _mm_andnot_ps(m, x));
            out.push(_mm_cvtss_f32(x).to_bits());
        }
    }
    println!("clamped bits: {:?}", out);

    let mut nan_count = 0u32;
    for &v in &data {
        unsafe {
            let m = _mm_cmpunord_sd(_mm_set_sd(f64::from(v)), _mm_set_sd(0.0));
            nan_count += (bits2(m)[0] != 0) as u32;
        }
    }
    println!("nan_count: {}", nan_count);
}

fn main() {
    part1_public_sse();
    if std::arch::is_x86_feature_detected!("avx") {
        unsafe { part2_avx_immediates() };
    } else {
        println!("--- part 2 skipped: no AVX on this CPU ---");
    }
    part3_realistic();
    println!("done");
}
