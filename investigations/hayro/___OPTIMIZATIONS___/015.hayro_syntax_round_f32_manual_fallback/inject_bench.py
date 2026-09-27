#!/usr/bin/env python3
"""Append the temporary round_f32 microbenchmark used to produce this candidate's numbers.

Not part of patch.diff (matches candidate 010's convention: bench instrumentation is not the
fix under test). Usage: inject_bench.py <path-to-math.rs-inside-the-clone>
"""
import sys

BENCH = '''

#[cfg(all(test, feature = "images"))]
mod round_f32_scale_bench {
    use super::round_f32;
    use std::hint::black_box;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn bench_round_f32_scale_loop() {
        let width = 1241usize;
        let height = 1752usize;
        let components = 1usize;
        let n = width * height * components;
        let data: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();

        let div_factor = ((1 << 8) - 1) as f32;
        let mul_factor = ((1_u32 << 4) - 1) as f32; // 4-bit palette, a plausible indexed depth

        let iterations = 50;
        let mut durations = Vec::with_capacity(iterations);
        let mut last_acc: u32 = 0;
        for _ in 0..iterations {
            let start = Instant::now();
            let mut acc: u32 = 0;
            for &byte in &data {
                let scaled = round_f32((byte as f32 / div_factor) * mul_factor) as u32;
                acc = acc.wrapping_add(black_box(scaled));
            }
            durations.push(start.elapsed());
            last_acc = acc;
        }
        durations.sort();
        eprintln!("ACC {last_acc}");
        eprintln!("MIN {:?}", durations[0]);
        eprintln!("P50 {:?}", durations[iterations / 2]);
        eprintln!("MAX {:?}", durations[iterations - 1]);
        for d in &durations {
            eprintln!("ALL {d:?}");
        }
        black_box(last_acc);
    }
}
'''

path = sys.argv[1]
with open(path, "a") as f:
    f.write(BENCH)
