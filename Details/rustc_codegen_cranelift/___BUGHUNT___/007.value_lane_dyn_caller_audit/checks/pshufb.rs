use std::arch::x86_64::*;

#[inline(never)]
fn hash(v: &[u8]) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for &b in v { h ^= b as u64; h = h.wrapping_mul(1099511628211); }
    h
}

fn main() {
    // Deterministic LCG selector bytes, deliberately covering 0x00..0xff including >=0x10 and >=0x80.
    let mut s: u32 = 0x1234_5678;
    let mut next = move || { s = s.wrapping_mul(1664525).wrapping_add(1013904223); (s >> 16) as u8 };

    let a16: [i8; 16] = core::array::from_fn(|i| (i as i8).wrapping_mul(7).wrapping_add(3));
    let a32: [i8; 32] = core::array::from_fn(|i| (i as i8).wrapping_mul(11).wrapping_sub(5));

    let mut acc128: Vec<u8> = Vec::new();
    let mut acc256: Vec<u8> = Vec::new();

    // exhaustive single-byte selectors: b = [k; 16] / [k; 32] for k in 0..=255
    for k in 0u16..=255 {
        let k = k as i8;
        unsafe {
            let a = _mm_loadu_si128(a16.as_ptr() as *const __m128i);
            let b = _mm_set1_epi8(k);
            let r = _mm_shuffle_epi8(a, b);
            let mut out = [0i8; 16];
            _mm_storeu_si128(out.as_mut_ptr() as *mut __m128i, r);
            acc128.extend(out.iter().map(|&x| x as u8));

            let a2 = _mm256_loadu_si256(a32.as_ptr() as *const __m256i);
            let b2 = _mm256_set1_epi8(k);
            let r2 = _mm256_shuffle_epi8(a2, b2);
            let mut out2 = [0i8; 32];
            _mm256_storeu_si256(out2.as_mut_ptr() as *mut __m256i, r2);
            acc256.extend(out2.iter().map(|&x| x as u8));
        }
    }

    // random selector vectors
    for _ in 0..2000 {
        let b16: [i8; 16] = core::array::from_fn(|_| next() as i8);
        let b32: [i8; 32] = core::array::from_fn(|_| next() as i8);
        unsafe {
            let a = _mm_loadu_si128(a16.as_ptr() as *const __m128i);
            let b = _mm_loadu_si128(b16.as_ptr() as *const __m128i);
            let r = _mm_shuffle_epi8(a, b);
            let mut out = [0i8; 16];
            _mm_storeu_si128(out.as_mut_ptr() as *mut __m128i, r);
            acc128.extend(out.iter().map(|&x| x as u8));

            let a2 = _mm256_loadu_si256(a32.as_ptr() as *const __m256i);
            let b2 = _mm256_loadu_si256(b32.as_ptr() as *const __m256i);
            let r2 = _mm256_shuffle_epi8(a2, b2);
            let mut out2 = [0i8; 32];
            _mm256_storeu_si256(out2.as_mut_ptr() as *mut __m256i, r2);
            acc256.extend(out2.iter().map(|&x| x as u8));
        }
    }

    println!("pshufb128 hash {:016x} len {}", hash(&acc128), acc128.len());
    println!("pshufb256 hash {:016x} len {}", hash(&acc256), acc256.len());
}
