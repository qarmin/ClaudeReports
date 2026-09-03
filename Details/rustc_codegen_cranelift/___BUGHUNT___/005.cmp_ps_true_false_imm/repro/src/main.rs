// _mm_cmp_ps / _mm_cmp_pd with the _CMP_FALSE_OQ (0x0b), _CMP_TRUE_UQ (0x0f),
// _CMP_FALSE_OS (0x1b) and _CMP_TRUE_US (0x1f) predicate immediates.
//
// LLVM: prints all-zero lanes for the FALSE predicates and all-ones lanes for the
// TRUE predicates.
// cg_clif: ICEs at codegen time ("not yet implemented", src/intrinsics/llvm_x86.rs)
// so this binary never even gets built.
use std::arch::x86_64::*;

fn main() {
    unsafe {
        let a4 = _mm_set_ps(4.0, f32::NAN, 2.0, 1.0);
        let b4 = _mm_set_ps(4.0, 1.0, 5.0, 1.0);
        let a2 = _mm_set_pd(f32::NAN as f64, 1.0);
        let b2 = _mm_set_pd(1.0, 1.0);

        show_ps("_mm_cmp_ps::<_CMP_FALSE_OQ> (0x0b)", _mm_cmp_ps::<_CMP_FALSE_OQ>(a4, b4));
        show_ps("_mm_cmp_ps::<_CMP_TRUE_UQ>  (0x0f)", _mm_cmp_ps::<_CMP_TRUE_UQ>(a4, b4));
        show_ps("_mm_cmp_ps::<_CMP_FALSE_OS> (0x1b)", _mm_cmp_ps::<_CMP_FALSE_OS>(a4, b4));
        show_ps("_mm_cmp_ps::<_CMP_TRUE_US>  (0x1f)", _mm_cmp_ps::<_CMP_TRUE_US>(a4, b4));

        show_pd("_mm_cmp_pd::<_CMP_FALSE_OQ> (0x0b)", _mm_cmp_pd::<_CMP_FALSE_OQ>(a2, b2));
        show_pd("_mm_cmp_pd::<_CMP_TRUE_UQ>  (0x0f)", _mm_cmp_pd::<_CMP_TRUE_UQ>(a2, b2));
        show_pd("_mm_cmp_pd::<_CMP_FALSE_OS> (0x1b)", _mm_cmp_pd::<_CMP_FALSE_OS>(a2, b2));
        show_pd("_mm_cmp_pd::<_CMP_TRUE_US>  (0x1f)", _mm_cmp_pd::<_CMP_TRUE_US>(a2, b2));

        // control: a predicate cg_clif already implements, to show the rest of the path works
        show_ps("_mm_cmp_ps::<_CMP_EQ_OQ>    (0x00)", _mm_cmp_ps::<_CMP_EQ_OQ>(a4, b4));
    }
}

unsafe fn show_ps(label: &str, v: __m128) {
    let mut o = [0f32; 4];
    _mm_storeu_ps(o.as_mut_ptr(), v);
    println!("{label} -> {:08x?}", o.map(|x| x.to_bits()));
}

unsafe fn show_pd(label: &str, v: __m128d) {
    let mut o = [0f64; 2];
    _mm_storeu_pd(o.as_mut_ptr(), v);
    println!("{label} -> {:016x?}", o.map(|x| x.to_bits()));
}
