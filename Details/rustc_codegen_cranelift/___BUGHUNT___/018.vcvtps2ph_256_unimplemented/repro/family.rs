use std::arch::x86_64::*;
fn main() {
    unsafe {
        let a4 = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
        let a8 = _mm256_setr_ps(1.0, 2.0, 3.0, 4.0, 0.5, -0.5, 65504.0, 1.0/3.0);
        let mut o = [0u16; 8];
        _mm_storeu_si128(o.as_mut_ptr() as *mut __m128i, _mm_cvtps_ph::<0>(a4));
        println!("ps2ph128 {:04x?}", o);
        _mm_storeu_si128(o.as_mut_ptr() as *mut __m128i, _mm256_cvtps_ph::<0>(a8));
        println!("ps2ph256 {:04x?}", o);
        let h = _mm_setr_epi16(0x3c00, 0x4000, 0x4200, 0x4400, 0x3800, -0x4800, 0x7bff, 0x3555);
        let mut f4 = [0f32; 4];
        _mm_storeu_ps(f4.as_mut_ptr(), _mm_cvtph_ps(h));
        println!("ph2ps128 {:?}", f4);
        let mut f8 = [0f32; 8];
        _mm256_storeu_ps(f8.as_mut_ptr(), _mm256_cvtph_ps(h));
        println!("ph2ps256 {:?}", f8);
    }
}
