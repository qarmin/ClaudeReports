// `__m256`/`__m128` have no `#[repr(C)]`, so rustc warns about them in an `extern "C"`
// signature; the warning is identical under both compilers and is not what this repro is about.
#![allow(improper_ctypes_definitions)]
// An `extern "C"` function that takes and returns a 256-bit SIMD vector by value.
//
// rustc only accepts this if the `avx` target feature is enabled for both the definition and the
// caller (`abi_required_features`). On cg_clif a per-function `#[target_feature(enable = "avx")]`
// is the ONLY way to satisfy that check: cg_clif's `target_config` hardcodes the globally-enabled
// feature set to fxsr/sse/sse2/x87 and ignores both `-C target-feature` and `-C target-cpu`.
//
// LLVM compiles this and prints the sum. cg_clif ICEs:
//   src/base.rs:222: Error while defining ...: Compilation(Unsupported("Unexpected SSA-value type: f32x8"))
use std::arch::x86_64::*;

#[target_feature(enable = "avx")]
#[inline(never)]
unsafe extern "C" fn c_add(a: __m256, b: __m256) -> __m256 {
    _mm256_add_ps(a, b)
}

#[target_feature(enable = "avx")]
unsafe fn run() {
    let a = _mm256_setr_ps(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    let b = _mm256_set1_ps(0.5);
    let r = c_add(a, b);
    let mut out = [0f32; 8];
    _mm256_storeu_ps(out.as_mut_ptr(), r);
    println!("extern \"C\" __m256 by value: {:?}", out);
}

fn main() {
    unsafe { run() }
}
