use std::arch::x86_64::*;
fn main() {
    unsafe {
        let a = _mm256_setr_epi32(10, 11, 12, 13, 14, 15, 16, 17);
        // out-of-range selectors: hardware masks to the low 3 bits
        let idx = _mm256_setr_epi32(8, 9, 100, -1, 0x7fff_ffff, 3, 2, 1);
        let r = _mm256_permutevar8x32_epi32(a, idx);
        let mut out = [0i32; 8];
        _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, r);
        println!("{:?}", out);
    }
}
