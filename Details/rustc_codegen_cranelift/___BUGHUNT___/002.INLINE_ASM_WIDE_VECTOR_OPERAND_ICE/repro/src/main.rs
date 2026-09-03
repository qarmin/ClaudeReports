//! Every inline-asm operand shape on x86-64 that is wider than 128 bits.
//!
//! Cranelift has no SSA type wider than 128 bits, so cg_clif cannot hand a 256/512-bit vector to
//! its inline-asm wrapper through an SSA value. Built with cg_clif this whole crate fails to
//! compile with an ICE, e.g.
//!
//!   Error while defining ..._mm256_stream_pd: Compilation(Unsupported("Unexpected SSA-value type: f64x4"))
//!
//! Built with LLVM it compiles and prints the expected values. Cases 1-8 are plain stable
//! `core::arch` intrinsics (stdarch implements `_mm*_stream_*` with `asm!(... in(ymm_reg) ...)`);
//! cases 9-13 are hand-written `asm!` with no stdarch involved. The last two cases are controls
//! that already work and must keep working.
//!
//! Requires a CPU with AVX-512F for the `_mm512_*`/`zmm_reg` cases.

use std::arch::x86_64::*;

#[repr(align(64))]
struct A64<T>(T);

// NB: no `_mm_sfence()` after the non-temporal stores. x86 guarantees same-thread
// self-consistency for the reads below, and `llvm.x86.sse.sfence` is a *separate*, unrelated
// unimplemented intrinsic in cg_clif that aborts at run time and would mask this bug.

#[target_feature(enable = "avx")]
unsafe fn stream_pd() {
    let mut m = A64([0.0f64; 4]);
    _mm256_stream_pd(m.0.as_mut_ptr(), _mm256_set1_pd(1.5));
    println!("01 _mm256_stream_pd         {:?}", m.0);
}

#[target_feature(enable = "avx")]
unsafe fn stream_ps() {
    let mut m = A64([0.0f32; 8]);
    _mm256_stream_ps(m.0.as_mut_ptr(), _mm256_set1_ps(2.5));
    println!("02 _mm256_stream_ps         {:?}", m.0);
}

#[target_feature(enable = "avx")]
unsafe fn stream_si256() {
    let mut m = A64([0i64; 4]);
    _mm256_stream_si256(m.0.as_mut_ptr().cast(), _mm256_set1_epi64x(-3));
    println!("03 _mm256_stream_si256      {:?}", m.0);
}

#[target_feature(enable = "avx2")]
unsafe fn stream_load_si256() {
    let m = A64([7i64, 8, 9, 10]);
    let v = _mm256_stream_load_si256(m.0.as_ptr().cast());
    let mut o = A64([0i64; 4]);
    _mm256_storeu_si256(o.0.as_mut_ptr().cast(), v);
    println!("04 _mm256_stream_load_si256 {:?}", o.0);
}

#[target_feature(enable = "avx512f")]
unsafe fn stream512_pd() {
    let mut m = A64([0.0f64; 8]);
    _mm512_stream_pd(m.0.as_mut_ptr(), _mm512_set1_pd(4.5));
    println!("05 _mm512_stream_pd         {:?}", m.0);
}

#[target_feature(enable = "avx512f")]
unsafe fn stream512_ps() {
    let mut m = A64([0.0f32; 16]);
    _mm512_stream_ps(m.0.as_mut_ptr(), _mm512_set1_ps(5.5));
    println!("06 _mm512_stream_ps         {:?}", m.0);
}

#[target_feature(enable = "avx512f")]
unsafe fn stream512_si512() {
    let mut m = A64([0i64; 8]);
    _mm512_stream_si512(m.0.as_mut_ptr().cast(), _mm512_set1_epi64(-6));
    println!("07 _mm512_stream_si512      {:?}", m.0);
}

#[target_feature(enable = "avx512f")]
unsafe fn stream512_load_si512() {
    let m = A64([1i64, 2, 3, 4, 5, 6, 7, 8]);
    let v = _mm512_stream_load_si512(m.0.as_ptr().cast());
    let mut o = A64([0i64; 8]);
    _mm512_storeu_si512(o.0.as_mut_ptr().cast(), v);
    println!("08 _mm512_stream_load_si512 {:?}", o.0);
}

#[target_feature(enable = "avx")]
unsafe fn raw_ymm_in_out() {
    let a = _mm256_set1_epi32(11);
    let r: __m256i;
    std::arch::asm!("vmovdqa {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 8];
    _mm256_storeu_si256(o.as_mut_ptr().cast(), r);
    println!("09 asm in/out(ymm_reg) i32  {:?}", o);
}

#[target_feature(enable = "avx")]
unsafe fn raw_ymm_f64x4() {
    let a = _mm256_set1_pd(1.25);
    let r: __m256d;
    std::arch::asm!("vmovapd {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0.0f64; 4];
    _mm256_storeu_pd(o.as_mut_ptr(), r);
    println!("10 asm in/out(ymm_reg) f64  {:?}", o);
}

#[target_feature(enable = "avx")]
unsafe fn raw_ymm_inout() {
    let mut a = _mm256_set1_epi32(13);
    std::arch::asm!("vpaddd {0}, {0}, {0}", inout(ymm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 8];
    _mm256_storeu_si256(o.as_mut_ptr().cast(), a);
    println!("11 asm inout(ymm_reg)       {:?}", o);
}

#[target_feature(enable = "avx512f")]
unsafe fn raw_zmm_in_out() {
    let a = _mm512_set1_epi32(17);
    let r: __m512i;
    std::arch::asm!("vmovdqa64 {0}, {1}", out(zmm_reg) r, in(zmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 16];
    _mm512_storeu_si512(o.as_mut_ptr().cast(), r);
    println!("12 asm in/out(zmm_reg)      {:?}", o);
}

#[target_feature(enable = "avx512f")]
unsafe fn raw_zmm_inout() {
    let mut a = _mm512_set1_epi64(19);
    std::arch::asm!("vpaddq {0}, {0}, {0}", inout(zmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i64; 8];
    _mm512_storeu_si512(o.as_mut_ptr().cast(), a);
    println!("13 asm inout(zmm_reg)       {:?}", o);
}

// Controls: a scalar in a wide register class, and a 128-bit operand. Both already work today and
// must keep working after the fix.
#[target_feature(enable = "avx")]
unsafe fn control_scalar_in_ymm() {
    let a = 3.5f64;
    let r: f64;
    std::arch::asm!("vmovapd {0}, {1}", out(ymm_reg) r, in(ymm_reg) a, options(nostack, nomem, pure));
    println!("14 control f64 in ymm_reg   {:?}", r);
}

unsafe fn control_xmm() {
    let mut a = _mm_set1_epi32(23);
    std::arch::asm!("paddd {0}, {0}", inout(xmm_reg) a, options(nostack, nomem, pure));
    let mut o = [0i32; 4];
    _mm_storeu_si128(o.as_mut_ptr().cast(), a);
    println!("15 control inout(xmm_reg)   {:?}", o);
}

fn main() {
    unsafe {
        stream_pd();
        stream_ps();
        stream_si256();
        stream_load_si256();
        stream512_pd();
        stream512_ps();
        stream512_si512();
        stream512_load_si512();
        raw_ymm_in_out();
        raw_ymm_f64x4();
        raw_ymm_inout();
        raw_zmm_in_out();
        raw_zmm_inout();
        control_scalar_in_ymm();
        control_xmm();
    }
}
