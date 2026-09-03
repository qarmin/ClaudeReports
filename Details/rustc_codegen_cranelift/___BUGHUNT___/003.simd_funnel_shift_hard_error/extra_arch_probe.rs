// Extra hand-checked probe for candidate 003 - the wider AVX512 rotate / funnel-shift family
// beyond what repro/ covers. Single-file, meant for the differential loop:
//
//   bash ___BUGHUNT___/tools/diffcheck.sh ___BUGHUNT___/003.simd_funnel_shift_hard_error/extra_arch_probe.rs
//   CLIF_ROOT=<patched worktree> bash ___BUGHUNT___/tools/diffcheck.sh .../extra_arch_probe.rs
//
// Unpatched cg_clif: 'error: Unknown SIMD intrinsic simd_funnel_shl'. Patched: identical to LLVM.
// Needs an AVX-512 host to run (it is unconditionally target_feature-gated, not cfg-gated).
use std::arch::x86_64::*;

macro_rules! p32 { ($n:expr, $v:expr) => { println!("{} {:08x?}", $n, std::mem::transmute::<_, [u32; 16]>($v)) } }

#[target_feature(enable = "avx512f")]
unsafe fn f512() {
    let a = std::mem::transmute::<[u32; 16], __m512i>([
        0x8000_0001, 0x0000_0000, 0xFFFF_FFFF, 0x1234_5678,
        0x0000_0001, 0x7FFF_FFFF, 0xAAAA_AAAA, 0x5555_5555,
        0xDEAD_BEEF, 0xCAFE_BABE, 0x0000_00FF, 0xFF00_0000,
        0x0F0F_0F0F, 0xF0F0_F0F0, 0x8000_0000, 0x0000_0002,
    ]);
    p32!("rol1 ", _mm512_rol_epi32::<1>(a));
    p32!("rol0 ", _mm512_rol_epi32::<0>(a));
    p32!("rol31", _mm512_rol_epi32::<31>(a));
    p32!("rol16", _mm512_rol_epi32::<16>(a));
    p32!("ror1 ", _mm512_ror_epi32::<1>(a));
    p32!("ror0 ", _mm512_ror_epi32::<0>(a));
    p32!("ror31", _mm512_ror_epi32::<31>(a));
    p32!("rolv ", _mm512_rolv_epi32(a, _mm512_setr_epi32(0,1,2,3,4,5,6,7,8,9,10,11,29,30,31,15)));
    p32!("rorv ", _mm512_rorv_epi32(a, _mm512_setr_epi32(0,1,2,3,4,5,6,7,8,9,10,11,29,30,31,15)));
    println!("rol64_1  {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_rol_epi64::<1>(a)));
    println!("rol64_63 {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_rol_epi64::<63>(a)));
    println!("ror64_0  {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_ror_epi64::<0>(a)));
    println!("rolv64   {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_rolv_epi64(a, _mm512_set_epi64(0,1,31,32,33,62,63,17))));
    // masked variants (blend on top of the same funnel shift)
    p32!("mrol ", _mm512_mask_rol_epi32::<3>(_mm512_set1_epi32(-1), 0b1010_1010_1010_1010, a));
    p32!("zrol ", _mm512_maskz_rol_epi32::<3>(0b0000_1111_1111_0000, a));
}

#[target_feature(enable = "avx512f,avx512vl")]
unsafe fn fvl() {
    let a = std::mem::transmute::<[u32; 8], __m256i>([1, 0x8000_0000, 0xFFFF_FFFF, 3, 0x1234_5678, 0, 0xAAAA_AAAA, 7]);
    println!("256 rol1 {:08x?}", std::mem::transmute::<_, [u32; 8]>(_mm256_rol_epi32::<1>(a)));
    println!("256 ror5 {:08x?}", std::mem::transmute::<_, [u32; 8]>(_mm256_ror_epi32::<5>(a)));
    println!("256 rolv {:016x?}", std::mem::transmute::<_, [u64; 4]>(_mm256_rolv_epi64(a, _mm256_set_epi64x(0, 1, 63, 32))));
    let b = std::mem::transmute::<[u32; 4], __m128i>([1, 0x8000_0000, 0xDEAD_BEEF, 0]);
    println!("128 rol7 {:08x?}", std::mem::transmute::<_, [u32; 4]>(_mm_rol_epi32::<7>(b)));
    println!("128 ror0 {:016x?}", std::mem::transmute::<_, [u64; 2]>(_mm_ror_epi64::<0>(b)));
}

#[target_feature(enable = "avx512vbmi2")]
unsafe fn fvbmi2() {
    let a = std::mem::transmute::<[u32; 16], __m512i>([0x8000_0001; 16]);
    let b = std::mem::transmute::<[u32; 16], __m512i>([0x1234_5678; 16]);
    p32!("shldi32_1 ", _mm512_shldi_epi32::<1>(a, b));
    p32!("shldi32_0 ", _mm512_shldi_epi32::<0>(a, b));
    p32!("shldi32_31", _mm512_shldi_epi32::<31>(a, b));
    p32!("shrdi32_1 ", _mm512_shrdi_epi32::<1>(a, b));
    p32!("shrdi32_0 ", _mm512_shrdi_epi32::<0>(a, b));
    println!("shldi16 {:04x?}", std::mem::transmute::<_, [u16; 32]>(_mm512_shldi_epi16::<7>(a, b)));
    println!("shrdi16 {:04x?}", std::mem::transmute::<_, [u16; 32]>(_mm512_shrdi_epi16::<15>(a, b)));
    println!("shldi64 {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_shldi_epi64::<33>(a, b)));
    println!("shrdi64 {:016x?}", std::mem::transmute::<_, [u64; 8]>(_mm512_shrdi_epi64::<0>(a, b)));
    let c = _mm512_setr_epi32(0,1,2,3,4,5,6,7,8,9,10,11,29,30,31,15);
    p32!("shldv32   ", _mm512_shldv_epi32(a, b, c));
    p32!("shrdv32   ", _mm512_shrdv_epi32(a, b, c));
}

fn main() {
    unsafe { f512(); fvl(); fvbmi2(); }
}
