// Compiler:
//
// Run-time:
//   status: 0

// Repro for candidate 030 (x86_sse2_psubs_saturating_subtract_min_negation): the x86-specific
// SSE2 `_mm_subs_epi8`/`_mm_subs_epi16` intrinsics (`llvm.x86.sse2.psubs.b`/`.w`) are dispatched
// straight through to libgccjit's `__builtin_ia32_psubsb128`/`__builtin_ia32_psubsw128` via
// `src/intrinsic/llvm.rs`'s `get_target_builtin_function`, with no rustc_codegen_gcc-side
// handling of the operands at all. This checks whether an `i8::MIN`/`i16::MIN` operand triggers
// the same wrong-saturation symptom candidate 025 already confirmed and fixed for the
// target-independent `core::simd::saturating_sub` Rust-level lowering.

use std::arch::x86_64::*;
use std::hint::black_box;

fn main() {
    unsafe {
        let a = black_box(_mm_setr_epi8(127, 0, -128, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        let b = black_box(_mm_setr_epi8(-128, -128, -128, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        let got = _mm_subs_epi8(a, b);
        let mut out = [0i8; 16];
        std::ptr::copy_nonoverlapping(&got as *const _ as *const i8, out.as_mut_ptr(), 16);
        let expected = [
            127i8.saturating_sub(-128),
            0i8.saturating_sub(-128),
            (-128i8).saturating_sub(-128),
            (-1i8).saturating_sub(1),
        ];
        assert_eq!(&out[..4], &expected[..], "_mm_subs_epi8 mismatch (operand includes i8::MIN)");

        let wa = black_box(_mm_setr_epi16(32767, 0, -32768, -1, 0, 0, 0, 0));
        let wb = black_box(_mm_setr_epi16(-32768, -32768, -32768, 1, 0, 0, 0, 0));
        let wgot = _mm_subs_epi16(wa, wb);
        let mut wout = [0i16; 8];
        std::ptr::copy_nonoverlapping(&wgot as *const _ as *const i16, wout.as_mut_ptr(), 8);
        let wexpected = [
            32767i16.saturating_sub(-32768),
            0i16.saturating_sub(-32768),
            (-32768i16).saturating_sub(-32768),
            (-1i16).saturating_sub(1),
        ];
        assert_eq!(
            &wout[..4],
            &wexpected[..],
            "_mm_subs_epi16 mismatch (operand includes i16::MIN)"
        );

        // Unsigned saturating subtract as a control - not expected to be affected (unsigned MIN
        // is 0, whose negation doesn't overflow).
        let ua = black_box(_mm_setr_epi8(5, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        let ub = black_box(_mm_setr_epi8(10, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        let ugot = _mm_subs_epu8(ua, ub);
        let mut uout = [0u8; 16];
        std::ptr::copy_nonoverlapping(&ugot as *const _ as *const u8, uout.as_mut_ptr(), 16);
        assert_eq!(&uout[..2], &[0u8, 0u8][..], "_mm_subs_epu8 control mismatch");

        println!("all assertions passed");
    }
}
