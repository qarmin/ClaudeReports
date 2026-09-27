//! Scratch repro (iterated in /tmp before being copied into the candidate folder).
//!
//! Compares `pic-scale-safe`'s real, public `resize_plane_f32` API (using
//! `ResamplingFunction::Robidoux`) against an independent oracle that reimplements the
//! *documented* Mitchell-Netravali BC-spline weight-generation algorithm (transcribed faithfully
//! from `compute_weights.rs`'s own "not is_area" branch, which is not itself in question here)
//! with the textbook-correct Robidoux constants B = 12/(19+9*sqrt(2)), C = 113/(58+216*sqrt(2)).

use pic_scale_safe::{resize_plane_f32, ImageSize, ResamplingFunction};

const SQRT2: f32 = std::f32::consts::SQRT_2;

fn bc_spline(d: f32, b: f32, c: f32) -> f32 {
    let mut x = d;
    if x < 0.0 {
        x = -x;
    }
    let dp = x * x;
    let tp = dp * x;
    if x < 1.0 {
        ((12.0 - 9.0 * b - 6.0 * c) * tp + (-18.0 + 12.0 * b + 6.0 * c) * dp + (6.0 - 2.0 * b))
            * (1.0 / 6.0)
    } else if x < 2.0 {
        ((-b - 6.0 * c) * tp + (6.0 * b + 30.0 * c) * dp + (-12.0 * b - 48.0 * c) * x + (8.0 * b + 24.0 * c))
            * (1.0 / 6.0)
    } else {
        0.0
    }
}

fn robidoux_b() -> f32 {
    12.0 / (19.0 + 9.0 * SQRT2)
}
fn robidoux_c_correct() -> f32 {
    113.0 / (58.0 + 216.0 * SQRT2)
}
fn robidoux_c_buggy() -> f32 {
    13.0 / (58.0 + 216.0 * SQRT2)
}

/// Faithful reimplementation of `compute_weights.rs::generate_weights`'s "not is_area" branch,
/// for `min_kernel_size = 2.0` (as registered for `Robidoux` in `sampler.rs`) and a 1-D row.
fn generate_weights_oracle(
    in_size: usize,
    out_size: usize,
    min_kernel_size: f32,
    kernel: impl Fn(f32) -> f32,
) -> Vec<(usize, Vec<f32>)> {
    let scale = in_size as f32 / out_size as f32;
    let filter_scale_cutoff = scale.max(1.0);
    let filter_base_size = min_kernel_size * 2.0;
    let base_size = (filter_base_size * filter_scale_cutoff).round() as usize;
    let kernel_size = base_size;
    let filter_radius = base_size as f32 / 2.0;
    let filter_scale = 1.0 / filter_scale_cutoff;

    let mut result = Vec::with_capacity(out_size);
    for i in 0..out_size {
        let center_x = ((i as f32 + 0.5) * scale).min(in_size as f32);
        let start = (center_x - filter_radius).floor().max(0.0) as usize;
        let end = (center_x + filter_radius)
            .ceil()
            .min(in_size as f32)
            .min((start + kernel_size) as f32) as usize;
        let center = center_x - 0.5;
        let mut weights = Vec::with_capacity(end - start);
        let mut sum = 0f32;
        for k in start..end {
            let dx = (k as f32 - center).abs();
            let w = kernel(dx * filter_scale);
            weights.push(w);
            sum += w;
        }
        if sum != 0.0 {
            for w in weights.iter_mut() {
                *w /= sum;
            }
        } else if !weights.is_empty() {
            let n = weights.len();
            for w in weights.iter_mut() {
                *w = 0.0;
            }
            let center_tap = ((center - start as f32).round().max(0.0) as usize).min(n - 1);
            weights[center_tap] = 1.0;
        }
        result.push((start, weights));
    }
    result
}

fn apply(source: &[f32], weights: &[(usize, Vec<f32>)]) -> Vec<f32> {
    weights
        .iter()
        .map(|(start, w)| w.iter().enumerate().map(|(i, &wt)| wt * source[start + i]).sum())
        .collect()
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).fold(0f32, f32::max)
}

fn main() {
    let source: Vec<f32> = vec![0.10, 0.90, 0.20, 0.80, 0.30, 0.70];
    let in_size = source.len();
    let out_size = 17usize;

    let oracle_correct = apply(
        &source,
        &generate_weights_oracle(in_size, out_size, 2.0, |x| {
            bc_spline(x, robidoux_b(), robidoux_c_correct())
        }),
    );
    let oracle_buggy = apply(
        &source,
        &generate_weights_oracle(in_size, out_size, 2.0, |x| {
            bc_spline(x, robidoux_b(), robidoux_c_buggy())
        }),
    );

    let actual = resize_plane_f32(
        &source,
        ImageSize::new(in_size, 1),
        ImageSize::new(out_size, 1),
        ResamplingFunction::Robidoux,
    )
    .expect("resize_plane_f32 failed");

    println!("oracle (correct C=113/...): {:?}", oracle_correct);
    println!("oracle (buggy   C=13/...) : {:?}", oracle_buggy);
    println!("actual (real crate call)  : {:?}", actual);

    let dev_vs_correct = max_abs_diff(&actual, &oracle_correct);
    let dev_vs_buggy = max_abs_diff(&actual, &oracle_buggy);
    println!("max |actual - oracle_correct| = {dev_vs_correct:.6}");
    println!("max |actual - oracle_buggy|   = {dev_vs_buggy:.6}");

    if dev_vs_correct < 1e-4 {
        println!("RESULT: actual output MATCHES the textbook-correct-C oracle (bug not present / fixed)");
    } else if dev_vs_buggy < 1e-4 {
        println!("RESULT: actual output MATCHES the buggy-C (13/...) oracle (bug present, as coded)");
    } else {
        println!("RESULT: actual output matches NEITHER oracle - unexpected, needs investigation");
    }
}
