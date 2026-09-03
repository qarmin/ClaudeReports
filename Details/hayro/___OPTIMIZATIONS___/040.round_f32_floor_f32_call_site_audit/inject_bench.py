#!/usr/bin/env python3
"""Append the temporary floor_f32 (horizontal+vertical IDWT) microbenchmark used to produce
this candidate's numbers.

Not part of patch.diff (matches candidates 010/015's own convention: bench instrumentation is
not the fix under test). Exercises the REAL `filter_horizontal`/`filter_vertical` private
functions with `WaveletTransform::Reversible53` (the only two direct call sites of
`math::floor_f32` outside of `round_f32`'s own no_std fallback - no PDF in this repo's test
corpus reaches this transform, see RESULT.md's "What's not done" section).

Usage: inject_bench.py <path-to-j2c/idwt.rs-inside-the-clone>
"""
import sys

BENCH = '''

// Candidate 040 (perf-optimize workspace): temporary microbenchmark for the real
// `reversible_filter_53r`/`reversible_filter_53r_simd` code paths, which are the only two
// call sites of `math::floor_f32` outside of `round_f32`'s own no_std fallback. Not part of
// the patch, reverted before finishing (see 010/015's own precedent for this pattern).
#[cfg(test)]
mod floor_f32_bench {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn floor_f32_hor_ver_bench() {
        let width = 1024usize;
        let height = 64usize;
        let rect = IntRect::from_xywh(0, 0, width as u32, height as u32);
        let iterations = 2000usize;

        let mut buf = vec![0.0f32; width * height];
        for (i, v) in buf.iter_mut().enumerate() {
            *v = ((i * 2654435761) % 4001) as f32 / 4000.0 - 0.5;
        }

        let start = Instant::now();
        let mut checksum: u64 = 0;
        for _ in 0..iterations {
            let mut scratch = buf.clone();
            filter_horizontal(&mut scratch, rect, WaveletTransform::Reversible53);
            filter_vertical(&mut scratch, rect, WaveletTransform::Reversible53);
            for v in &scratch {
                checksum = checksum.wrapping_add(v.to_bits() as u64);
            }
        }
        let elapsed = start.elapsed();

        println!(
            "floor_f32_hor_ver_bench: {} iterations, {:.3} ms total, {:.6} ms/iter, checksum={checksum}",
            iterations,
            elapsed.as_secs_f64() * 1000.0,
            elapsed.as_secs_f64() * 1000.0 / iterations as f64,
        );
    }
}
'''

path = sys.argv[1]
with open(path, "a") as f:
    f.write(BENCH)
