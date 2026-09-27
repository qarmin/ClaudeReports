// Standalone repro for candidate 033 (sse4a_extrq_insertq_zero_length_immediate_bug).
//
// Per the x86 ISA (and stdarch's own doc comment on `_mm_extracti_si64`/`_mm_inserti_si64`), a
// LEN immediate of 0 is a sentinel meaning "64 bits" (the full quadword), not literally zero
// bits. This checks that behavior for LEN=0, IDX=0 - both the specific "full 64 bits" boundary
// case and a broader boundary+random sweep - against real SSE4a hardware, compiled directly
// through this project's own codegen backend (`y.sh rustc`, no cargo wrapper needed).
//
// `#[target_feature(enable = "sse4a")]` + `std::hint::black_box` on every runtime input keeps the
// GCC/LLVM optimizer from constant-folding the intrinsic call away, so the actual `extrq`/
// `insertq` hardware instructions are exercised at runtime, not evaluated at compile time.

use std::arch::x86_64::*;
use std::hint::black_box;

#[target_feature(enable = "sse4a")]
unsafe fn extract_len0_idx0(low: u64, high: u64) -> u64 {
    unsafe {
        let a = _mm_set_epi64x(black_box(high) as i64, black_box(low) as i64);
        let r = _mm_extracti_si64::<0, 0>(a);
        let mut out = [0u64; 2];
        _mm_storeu_si128(out.as_mut_ptr() as *mut __m128i, r);
        out[0]
    }
}

#[target_feature(enable = "sse4a")]
unsafe fn insert_len0_idx0(x_low: u64, y_low: u64) -> u64 {
    unsafe {
        let x = _mm_set_epi64x(0, black_box(x_low) as i64);
        let y = _mm_set_epi64x(0, black_box(y_low) as i64);
        let r = _mm_inserti_si64::<0, 0>(x, y);
        let mut out = [0u64; 2];
        _mm_storeu_si128(out.as_mut_ptr() as *mut __m128i, r);
        out[0]
    }
}

// Small xorshift PRNG so the repro needs no external crates.
struct Xs(u64);
impl Xs {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn main() {
    if !is_x86_feature_detected!("sse4a") {
        println!("SKIP: sse4a not available on this host");
        return;
    }

    let mut rng = Xs(0x1234_5678_9abc_def1);
    let mut failures = 0usize;
    let mut total = 0usize;

    let fixed_values: [u64; 8] = [
        0,
        u64::MAX,
        0x0123_4567_89ab_cdef,
        0x8000_0000_0000_0000,
        0x0000_0000_ffff_ffff,
        0xffff_ffff_0000_0000,
        0x0000_0000_0000_0001,
        0x1122_3344_5566_7788,
    ];

    for &low in fixed_values.iter() {
        for &high in fixed_values.iter() {
            total += 1;
            let got = unsafe { extract_len0_idx0(low, high) };
            if got != low {
                failures += 1;
                println!(
                    "EXTRACT MISMATCH: low={:#018x} high={:#018x} got={:#018x} expected={:#018x}",
                    low, high, got, low
                );
            }
        }
    }
    for _ in 0..2000 {
        let low = rng.next();
        let high = rng.next();
        total += 1;
        let got = unsafe { extract_len0_idx0(low, high) };
        if got != low {
            failures += 1;
            println!(
                "EXTRACT MISMATCH: low={:#018x} high={:#018x} got={:#018x} expected={:#018x}",
                low, high, got, low
            );
        }
    }

    for &x in fixed_values.iter() {
        for &y in fixed_values.iter() {
            total += 1;
            let got = unsafe { insert_len0_idx0(x, y) };
            if got != y {
                failures += 1;
                println!(
                    "INSERT MISMATCH: x={:#018x} y={:#018x} got={:#018x} expected={:#018x}",
                    x, y, got, y
                );
            }
        }
    }
    for _ in 0..2000 {
        let x = rng.next();
        let y = rng.next();
        total += 1;
        let got = unsafe { insert_len0_idx0(x, y) };
        if got != y {
            failures += 1;
            println!(
                "INSERT MISMATCH: x={:#018x} y={:#018x} got={:#018x} expected={:#018x}",
                x, y, got, y
            );
        }
    }

    println!("total={} failures={}", total, failures);
    if failures > 0 {
        std::process::exit(1);
    } else {
        println!("ALL OK");
    }
}
