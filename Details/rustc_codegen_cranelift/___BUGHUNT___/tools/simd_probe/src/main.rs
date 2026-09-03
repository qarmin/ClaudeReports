//! Standalone probe for the x86 SIMD gaps found by the 2026-09-02 hodow audit of cg_clif.
//!
//! Every probe is self-checking: it computes a value whose correct answer is known and prints
//! `ok` / `WRONG`. Build it twice - once with plain rustc, once with `dist/rustc-clif` - and
//! compare. See `../../README.md` for the exact commands (`just probe` in this directory).
//!
//! Probes that make cg_clif fail to *compile* or *ICE* live behind cargo features so the rest
//! still builds: `--features funnel` (hard error: missing `simd_funnel_shl`/`_shr`),
//! `--features stream` (ICE: `Unexpected SSA-value type`), `--features kreg` (inline-asm
//! assembler error on AVX-512 mask registers).

#![allow(internal_features)]

use std::arch::x86_64::*;

static mut FAILURES: u32 = 0;

fn check<T: PartialEq + std::fmt::Debug>(name: &str, got: T, want: T) {
    if got == want {
        println!("ok    {name}");
    } else {
        println!("WRONG {name}\n        got  {got:?}\n        want {want:?}");
        unsafe { FAILURES += 1 };
    }
}

/// Confirmed miscompilation: `llvm.x86.avx2.permd` uses the raw index instead of its low 3 bits,
/// so cg_clif reads past the end of the vector (garbage from the stack).
/// src/intrinsics/llvm_x86.rs, arm `"llvm.x86.avx2.permd"`.
unsafe fn probe_permd() {
    let a = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 8);
    let idx = _mm256_setr_epi32(9, 10, 11, 12, 13, 14, 15, 16); // & 7 == 1,2,3,4,5,6,7,0
    let mut out = [0i32; 8];
    _mm256_storeu_si256(out.as_mut_ptr().cast(), _mm256_permutevar8x32_epi32(a, idx));
    check("_mm256_permutevar8x32_epi32 (index masking)", out, [2, 3, 4, 5, 6, 7, 8, 1]);

    let mut out = [0i32; 8];
    _mm256_storeu_si256(out.as_mut_ptr().cast(), _mm256_permutexvar_epi32(a, idx));
    check("_mm256_permutexvar_epi32 (index masking)", out, [10, 11, 12, 13, 14, 15, 16, 9]);
}

/// Same family, but with a fully in-range index vector - if this one is also wrong the bug is not
/// (only) about masking.
unsafe fn probe_permd_in_range() {
    let a = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 8);
    let idx = _mm256_setr_epi32(7, 6, 5, 4, 3, 2, 1, 0);
    let mut out = [0i32; 8];
    _mm256_storeu_si256(out.as_mut_ptr().cast(), _mm256_permutevar8x32_epi32(a, idx));
    check("_mm256_permutevar8x32_epi32 (in-range index)", out, [8, 7, 6, 5, 4, 3, 2, 1]);
}

/// Behind `--features permilvar`: unimplemented in cg_clif today (traps at run time), so it would
/// abort the whole probe run.
/// Same dynamic-lane-index shape as permd, different intrinsic family - checks whether the
/// missing mask is a one-off or a pattern across every `value_lane_dyn` user.
#[cfg(feature = "permilvar")]
unsafe fn probe_permilvar() {
    let a = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
    let idx = _mm_setr_epi32(3, 2, 1, 0);
    let mut out = [0f32; 4];
    _mm_storeu_ps(out.as_mut_ptr(), _mm_permutevar_ps(a, idx));
    check("_mm_permutevar_ps (in-range index)", out, [4.0, 3.0, 2.0, 1.0]);

    // Only bits [1:0] of each selector are architecturally used.
    let idx_hi = _mm_setr_epi32(7, 6, 5, 4);
    let mut out = [0f32; 4];
    _mm_storeu_ps(out.as_mut_ptr(), _mm_permutevar_ps(a, idx_hi));
    check("_mm_permutevar_ps (selector masking)", out, [4.0, 3.0, 2.0, 1.0]);
}

/// A handful of unrelated intrinsics that the audit reported as passing - a sanity check that a
/// failure above is specific, not the whole build being broken.
unsafe fn probe_control() {
    let a = _mm256_setr_epi32(1, -2, 3, -4, 5, -6, 7, -8);
    let mut out = [0i32; 8];
    _mm256_storeu_si256(out.as_mut_ptr().cast(), _mm256_abs_epi32(a));
    check("_mm256_abs_epi32 (control)", out, [1, 2, 3, 4, 5, 6, 7, 8]);

    let x = _mm512_set1_epi32(3);
    let y = _mm512_set1_epi32(4);
    let mut out = [0i32; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_add_epi32(x, y));
    check("_mm512_add_epi32 (control)", out, [7i32; 16]);
}

/// Hard compile error under cg_clif: `Unknown SIMD intrinsic simd_funnel_shl` / `_shr`.
/// Reached via AVX512F rotates and every AVX512VBMI2 `sh[lr]d` intrinsic.
#[cfg(feature = "funnel")]
unsafe fn probe_funnel() {
    let a = _mm512_set1_epi32(0x8000_0001u32 as i32);
    let mut out = [0i32; 16];
    _mm512_storeu_si512(out.as_mut_ptr().cast(), _mm512_rol_epi32::<1>(a));
    check("_mm512_rol_epi32 (simd_funnel_shl)", out, [3i32; 16]);
}

/// ICE under cg_clif: `Compilation(Unsupported("Unexpected SSA-value type: f64x4"/"i64x8"))`.
/// Every non-temporal 256/512-bit load/store hits it.
#[cfg(feature = "stream")]
unsafe fn probe_stream() {
    let mut buf = [0f64; 4];
    _mm256_stream_pd(buf.as_mut_ptr(), _mm256_set1_pd(1.5));
    check("_mm256_stream_pd", buf, [1.5; 4]);

    let mut buf = [0i64; 8];
    _mm512_stream_si512(buf.as_mut_ptr().cast(), _mm512_set1_epi64(7));
    check("_mm512_stream_si512", buf, [7i64; 8]);
}

/// Assembler error under cg_clif: the generated global_asm saves/restores AVX-512 mask registers
/// with `mov k1, [rbx+0x18]` instead of `kmov`. src/inline_asm.rs save_register/restore_register.
#[cfg(feature = "kreg")]
unsafe fn probe_kreg() {
    let src = [4.0f64];
    let got = _mm_mask_load_sd(_mm_setzero_pd(), 0b1, src.as_ptr());
    let mut out = [0f64; 2];
    _mm_storeu_pd(out.as_mut_ptr(), got);
    check("_mm_mask_load_sd (kreg inline asm)", out, [4.0, 0.0]);
}

fn main() {
    for f in ["avx2", "avx512f", "avx512vl"] {
        if !feature_present(f) {
            eprintln!("skipping: CPU lacks {f}");
            return;
        }
    }
    unsafe {
        probe_permd();
        probe_permd_in_range();
        #[cfg(feature = "permilvar")]
        probe_permilvar();
        probe_control();
        #[cfg(feature = "funnel")]
        probe_funnel();
        #[cfg(feature = "stream")]
        probe_stream();
        #[cfg(feature = "kreg")]
        probe_kreg();

        let n = &raw const FAILURES;
        println!("\n{} probe(s) produced a wrong value", *n);
        if *n > 0 {
            std::process::exit(1);
        }
    }
}

fn feature_present(f: &str) -> bool {
    match f {
        "avx2" => std::arch::is_x86_feature_detected!("avx2"),
        "avx512f" => std::arch::is_x86_feature_detected!("avx512f"),
        "avx512vl" => std::arch::is_x86_feature_detected!("avx512vl"),
        _ => false,
    }
}
