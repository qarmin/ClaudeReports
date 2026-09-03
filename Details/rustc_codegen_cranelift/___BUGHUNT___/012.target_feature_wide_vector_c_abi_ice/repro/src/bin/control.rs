// `__m256`/`__m128` have no `#[repr(C)]`, so rustc warns about them in an `extern "C"`
// signature; the warning is identical under both compilers and is not what this repro is about.
#![allow(improper_ctypes_definitions)]
// Controls that must keep working and keep matching LLVM byte-for-byte: a 128-bit vector through
// the same `extern "C"` by-value path (Cranelift has an I8X16/F32X4 type for those), and a 256-bit
// vector through the Rust ABI (rustc passes anything over 16 bytes indirectly there, so no wide
// Cranelift SSA value is ever needed).
use std::arch::x86_64::*;

#[inline(never)]
unsafe extern "C" fn c_add_128(a: __m128, b: __m128) -> __m128 {
    _mm_add_ps(a, b)
}

#[inline(never)]
unsafe extern "C" fn c_add_128i(a: __m128i, b: __m128i) -> __m128i {
    _mm_add_epi32(a, b)
}

#[target_feature(enable = "avx")]
#[inline(never)]
unsafe fn rust_add_256(a: __m256, b: __m256) -> __m256 {
    _mm256_add_ps(a, b)
}

#[target_feature(enable = "avx")]
unsafe fn run_rust_abi() {
    let r = rust_add_256(
        _mm256_setr_ps(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0),
        _mm256_set1_ps(0.5),
    );
    let mut out = [0f32; 8];
    _mm256_storeu_ps(out.as_mut_ptr(), r);
    println!("Rust-ABI __m256 by value:    {:?}", out);
}

fn main() {
    unsafe {
        let r = c_add_128(_mm_setr_ps(1.0, 2.0, 3.0, 4.0), _mm_set1_ps(0.5));
        let mut o = [0f32; 4];
        _mm_storeu_ps(o.as_mut_ptr(), r);
        println!("extern \"C\" __m128 by value:  {:?}", o);

        let ri = c_add_128i(_mm_setr_epi32(1, 2, 3, 4), _mm_set1_epi32(10));
        let mut oi = [0i32; 4];
        _mm_storeu_si128(oi.as_mut_ptr() as *mut __m128i, ri);
        println!("extern \"C\" __m128i by value: {:?}", oi);

        run_rust_abi();
    }
}
