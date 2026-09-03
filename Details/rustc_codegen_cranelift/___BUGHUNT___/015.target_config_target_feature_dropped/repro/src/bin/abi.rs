// `__m256` has no `#[repr(C)]`, so rustc warns about it in an `extern "C"` signature; the warning
// is identical under both compilers and is not what this repro is about.
#![allow(improper_ctypes_definitions)]
// An `extern "C"` function taking and returning a 256-bit SIMD vector by value, with the `avx`
// feature requested through `-C target-feature=+avx` alone (no per-function `#[target_feature]`).
//
// rustc's `abi_required_features` check reads `sess.internal_target_features`, which cg_clif's
// `target_config` hardcodes, so cg_clif rejects this program - and its own diagnostic suggests
// doing exactly what the build already does ("consider enabling it globally
// (`-C target-feature=+avx`)"). LLVM compiles it and prints the sum.
use std::arch::x86_64::*;

#[inline(never)]
unsafe extern "C" fn c_add(a: __m256, b: __m256) -> __m256 {
    unsafe { _mm256_add_ps(a, b) }
}

fn main() {
    unsafe {
        let a = _mm256_setr_ps(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = _mm256_set1_ps(0.5);
        let r = c_add(a, b);
        let mut out = [0f32; 8];
        _mm256_storeu_ps(out.as_mut_ptr(), r);
        println!("extern \"C\" __m256 by value: {:?}", out);
    }
}
