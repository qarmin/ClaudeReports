// Temporary isolated benchmark for the sRGB gamma powf -> LUT candidate.
// Not part of the permanent examples; reverted along with the rest of the candidate's patch.
use rawler::imgop::srgb::srgb_apply_gamma_inplace;
use std::hint::black_box;
use std::time::Instant;

// Matches the element count of the real Canon EOS R5 full-develop sample used by
// rawler/benches/demosaic.rs's decoding-full-develop group (134283264 f32 elements).
const N: usize = 134_283_264;
const ITERS: usize = 20;

fn make_buffer() -> Vec<f32> {
  // Deterministic, no external RNG dependency. Spans well below and above the
  // 0.00304 linear/exponential crossover point, and above 1.0 (clip_euclidean_norm_avg
  // in imgop/raw.rs can let values above 1.0 through before gamma is applied).
  (0..N)
    .map(|i| {
      let x = (i as u64).wrapping_mul(2654435761) as f32 / u32::MAX as f32;
      x * 1.3
    })
    .collect()
}

fn main() {
  let template = make_buffer();
  let mut times = Vec::with_capacity(ITERS);
  for _ in 0..ITERS {
    let mut buf = template.clone();
    let start = Instant::now();
    srgb_apply_gamma_inplace(black_box(&mut buf));
    let elapsed = start.elapsed();
    black_box(&buf);
    times.push(elapsed.as_secs_f64() * 1000.0);
  }
  // Discard warmup (first ~10%).
  let warmup = (ITERS / 10).max(1);
  let mut trimmed: Vec<f64> = times[warmup..].to_vec();
  trimmed.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let n = trimmed.len();
  let min = trimmed[0];
  let p50 = trimmed[n / 2];
  let p95 = trimmed[(n * 95 / 100).min(n - 1)];
  let max = trimmed[n - 1];
  let mean: f64 = trimmed.iter().sum::<f64>() / n as f64;
  println!(
    "n={} threads_env={:?} min={:.2}ms p50={:.2}ms mean={:.2}ms p95={:.2}ms max={:.2}ms",
    n,
    std::env::var("RAYON_NUM_THREADS").ok(),
    min,
    p50,
    mean,
    p95,
    max
  );
}
