// Candidate 020: the 24 SSE/SSE2 scalar "compare and set flags" intrinsics
// (`llvm.x86.sse.{,u}comi{eq,lt,le,gt,ge,neq}.ss` and `llvm.x86.sse2.{,u}comi{...}.sd`) have no
// lowering in rustc_codegen_cranelift, so every `_mm_comiXX_ss/_sd` and `_mm_ucomiXX_ss/_sd` call
// compiles to `codegen_panic_nounwind` (a compile *warning*, not an error) and aborts the process
// at run time.
//
// All of these are SSE/SSE2 baseline on x86_64: safe to call from stable code on every x86-64 CPU,
// no `target_feature` gate, no `is_x86_feature_detected!` guard needed.
//
// Build with plain rustc (LLVM) -> prints 380 lines, exit 0.
// Build with dist/cargo-clif   -> aborts with "llvm.x86.sse.comieq.ss is not yet supported", exit 134.

#![cfg(target_arch = "x86_64")]

use std::arch::x86_64::*;

/// Lane-0 boundary pairs. The upper lanes are deliberately made *different* between the two
/// operands so that a lowering which looks at anything other than lane 0 is visible as a wrong
/// answer rather than an accidental pass.
const F32_PAIRS: &[(f32, f32)] = &[
    (1.0, 1.0),
    (1.0, 2.0),
    (2.0, 1.0),
    (f32::NAN, 1.0),
    (1.0, f32::NAN),
    (f32::NAN, f32::NAN),
    (-0.0, 0.0),
    (0.0, -0.0),
    (f32::INFINITY, f32::INFINITY),
    (f32::NEG_INFINITY, f32::INFINITY),
    (f32::INFINITY, f32::MAX),
    (f32::MIN_POSITIVE, 0.0),
    (-1.0, -2.0),
];

const F64_PAIRS: &[(f64, f64)] = &[
    (1.0, 1.0),
    (1.0, 2.0),
    (2.0, 1.0),
    (f64::NAN, 1.0),
    (1.0, f64::NAN),
    (f64::NAN, f64::NAN),
    (-0.0, 0.0),
    (0.0, -0.0),
    (f64::INFINITY, f64::INFINITY),
    (f64::NEG_INFINITY, f64::INFINITY),
    (f64::INFINITY, f64::MAX),
    (f64::MIN_POSITIVE, 0.0),
    (-1.0, -2.0),
];

macro_rules! sweep_ss {
    ($($f:ident),* $(,)?) => {$(
        unsafe {
            for &(a0, b0) in F32_PAIRS {
                let a = _mm_set_ps(7.0, 8.0, 9.0, a0);
                let b = _mm_set_ps(-7.0, -8.0, -9.0, b0);
                println!(
                    "{:<16} ({:>10}, {:>10}) = {}",
                    stringify!($f),
                    fmt32(a0),
                    fmt32(b0),
                    $f(a, b)
                );
            }
        }
    )*};
}

macro_rules! sweep_sd {
    ($($f:ident),* $(,)?) => {$(
        unsafe {
            for &(a0, b0) in F64_PAIRS {
                let a = _mm_set_pd(7.0, a0);
                let b = _mm_set_pd(-7.0, b0);
                println!(
                    "{:<16} ({:>10}, {:>10}) = {}",
                    stringify!($f),
                    fmt64(a0),
                    fmt64(b0),
                    $f(a, b)
                );
            }
        }
    )*};
}

fn fmt32(v: f32) -> String {
    format!("{v:?}/{:#x}", v.to_bits())
}

fn fmt64(v: f64) -> String {
    format!("{v:?}/{:#x}", v.to_bits())
}

/// Part 3: a realistic caller - a branch-light "is this sample inside the window" test and a
/// NaN-rejecting minimum search, both written the way SIMD code normally uses these intrinsics
/// (compare lane 0, branch on the returned 0/1).
fn window_and_min(samples: &[f32], lo: f32, hi: f32) -> (usize, f32) {
    unsafe {
        let vlo = _mm_set_ss(lo);
        let vhi = _mm_set_ss(hi);
        let mut inside = 0usize;
        let mut best = _mm_set_ss(f32::INFINITY);
        for &s in samples {
            let v = _mm_set_ss(s);
            // `ucomi` on a NaN sample yields 0 for both, so NaNs are skipped without a branch on
            // `is_nan()`.
            if _mm_ucomige_ss(v, vlo) == 1 && _mm_ucomile_ss(v, vhi) == 1 {
                inside += 1;
                if _mm_ucomilt_ss(v, best) == 1 {
                    best = v;
                }
            }
        }
        (inside, _mm_cvtss_f32(best))
    }
}

/// Same shape in f64, using the `comi` (signalling) spelling and the `neq` predicate.
fn count_distinct_f64(xs: &[f64]) -> usize {
    unsafe {
        let mut n = 0usize;
        for w in xs.windows(2) {
            let a = _mm_set_sd(w[0]);
            let b = _mm_set_sd(w[1]);
            if _mm_comineq_sd(a, b) == 1 {
                n += 1;
            }
        }
        n
    }
}

fn main() {
    println!("--- part 1: all 12 f32 forms x 13 boundary pairs ---");
    sweep_ss!(
        _mm_comieq_ss,
        _mm_comilt_ss,
        _mm_comile_ss,
        _mm_comigt_ss,
        _mm_comige_ss,
        _mm_comineq_ss,
        _mm_ucomieq_ss,
        _mm_ucomilt_ss,
        _mm_ucomile_ss,
        _mm_ucomigt_ss,
        _mm_ucomige_ss,
        _mm_ucomineq_ss,
    );

    println!("--- part 2: all 12 f64 forms x 13 boundary pairs ---");
    sweep_sd!(
        _mm_comieq_sd,
        _mm_comilt_sd,
        _mm_comile_sd,
        _mm_comigt_sd,
        _mm_comige_sd,
        _mm_comineq_sd,
        _mm_ucomieq_sd,
        _mm_ucomilt_sd,
        _mm_ucomile_sd,
        _mm_ucomigt_sd,
        _mm_ucomige_sd,
        _mm_ucomineq_sd,
    );

    println!("--- part 3: realistic callers ---");
    let samples = [
        3.5f32,
        -1.0,
        f32::NAN,
        0.25,
        10.0,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        1.0,
        f32::INFINITY,
    ];
    let (inside, best) = window_and_min(&samples, -0.0, 5.0);
    println!("window_and_min = ({inside}, {best:?}/{:#x})", best.to_bits());

    let xs = [1.0f64, 1.0, 2.0, f64::NAN, f64::NAN, 2.0, -0.0, 0.0];
    println!("count_distinct_f64 = {}", count_distinct_f64(&xs));
}
