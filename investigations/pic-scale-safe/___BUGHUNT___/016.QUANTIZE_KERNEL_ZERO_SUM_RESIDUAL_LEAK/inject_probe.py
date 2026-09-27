import sys

path = "src/filter_weights.rs"
marker = """impl FilterWeights<f32> {
    pub(crate) fn numerical_approximation_i16<const PRECISION: i32>("""

probe_mod = '''
#[cfg(test)]
mod bughunt_probe {
    use super::*;
    use crate::compute_weights::generate_weights;
    use crate::resizer::{resize_plane16, resize_plane8};
    use crate::sampler::ResamplingFunction;

    // Directly demonstrates that `quantize_kernel`'s residual redistribution does NOT put the
    // whole `precision_scale` (32768 at PRECISION=15) on tap 0 for an all-zero-weight row, as
    // candidate 010's spot check assumed - precision_scale (32768) exceeds i16::MAX (32767) by
    // exactly 1, so exactly 1 quantization unit spills onto the *second* tap instead.
    #[test]
    fn probe_zero_sum_row_leaks_onto_second_tap() {
        let taps = 4usize;
        let weights: Vec<f32> = vec![0.0; taps];
        let precision_scale: f64 = (1i64 << 15) as f64;
        let mut scratch = vec![0f64; taps];
        let mut order: Vec<usize> = Vec::with_capacity(taps);
        quantize_kernel(
            &weights,
            taps,
            precision_scale,
            i16::MIN as f64,
            i16::MAX as f64,
            &mut scratch,
            &mut order,
        );
        eprintln!("scratch = {:?}", scratch);
        assert_eq!(scratch[0], 32767.0, "tap 0 should be capped at i16::MAX, not the full residual");
        assert_eq!(scratch[1], 1.0, "the 1-unit leftover should spill onto the second tap");
        let sum: f64 = scratch.iter().sum();
        assert_eq!(sum, precision_scale);
    }

    // Public-API check: does this leak ever flip an actual 8/12-bit output byte away from the
    // "full weight on the first source tap" nearest-fallback expectation? Uses the same
    // HaasnSoft 3->10 case candidate 010 used (known to produce zero-sum rows), with the most
    // divergent boundary source values possible at each reachable bit depth (0/255 for 8-bit,
    // 0/4095 for 12-bit - the only bit depths the fixed-point/i16-quantization path is ever
    // reached with through this crate's public API).
    #[test]
    fn probe_public_api_divergence() {
        let in_size = 3usize;
        let out_size = 10usize;
        let fweights = generate_weights::<f32>(ResamplingFunction::HaasnSoft, in_size, out_size);

        let mut zero_rows: Vec<(usize, usize, usize)> = Vec::new();
        for (i, (chunk, bounds)) in fweights
            .weights
            .chunks_exact(fweights.kernel_size)
            .zip(fweights.bounds.iter())
            .enumerate()
        {
            let taps = bounds.size.min(fweights.kernel_size);
            let sum: f32 = chunk.iter().take(taps).sum();
            if sum == 0.0 && taps > 0 {
                zero_rows.push((i, bounds.start, taps));
            }
        }
        eprintln!("zero-sum rows (idx, start, taps): {:?}", zero_rows);
        assert!(!zero_rows.is_empty(), "expected at least one zero-sum row for this case");

        let mut mismatches: Vec<String> = Vec::new();

        for &(lo, hi) in &[(0u8, 255u8), (255u8, 0u8)] {
            let src8 = vec![lo, hi, lo];
            let dst8 = resize_plane8(
                &src8,
                crate::ImageSize::new(in_size, 1),
                crate::ImageSize::new(out_size, 1),
                ResamplingFunction::HaasnSoft,
            )
            .unwrap();
            for &(idx, start, _taps) in &zero_rows {
                let expected = src8[start];
                let actual = dst8[idx];
                if actual != expected {
                    mismatches.push(format!(
                        "8-bit src={src8:?} row idx={idx} start={start}: expected {expected}, got {actual}"
                    ));
                }
            }
        }

        for &(lo, hi) in &[(0u16, 4095u16), (4095u16, 0u16)] {
            let src16 = vec![lo, hi, lo];
            let dst16 = resize_plane16(
                &src16,
                crate::ImageSize::new(in_size, 1),
                crate::ImageSize::new(out_size, 1),
                12,
                ResamplingFunction::HaasnSoft,
            )
            .unwrap();
            for &(idx, start, _taps) in &zero_rows {
                let expected = src16[start];
                let actual = dst16[idx];
                if actual != expected {
                    mismatches.push(format!(
                        "12-bit src={src16:?} row idx={idx} start={start}: expected {expected}, got {actual}"
                    ));
                }
            }
        }

        eprintln!("mismatches = {:#?}", mismatches);
        assert!(mismatches.is_empty(), "found divergence(s): {:#?}", mismatches);
    }
}
'''

with open(path) as f:
    s = f.read()

if marker not in s:
    sys.exit("marker not found - filter_weights.rs source changed shape, update inject_probe.py")

with open(path, "w") as f:
    f.write(s.replace(marker, probe_mod + "\n" + marker))
