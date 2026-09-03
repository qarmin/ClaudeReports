// Standalone microbenchmark for `SortModelInner::row_changed` (internal/core/model/adapters.rs).
//
// `mapping` is sorted by *value*, not original row index, so locating a changed row's current
// position in `mapping` uses `mapping.iter().position(|r| *r == row)` - an O(n) linear scan -
// on every single `set_row_data` call on a sorted model. This bench drives that path through the
// public API (`VecModel::set_row_data` -> `ModelNotify::row_changed` -> `SortModelInner::row_changed`)
// with a deterministic sequence of updates, and reports both wall time and a checksum of the
// final sorted contents (for a bit-exact correctness check between baseline and patched builds).
//
// Usage:
//   bench_sort_model_row_changed <row_count> <updates> [iters=15] [seed=1] [max_delta=0]
//
// `max_delta`: 0 means each update's new value is uniformly random across the whole value range
// (a "jump anywhere in the sort order" edit - worst case for reorder distance). A positive value
// instead perturbs the row's *current* value by at most +/-max_delta (a "nudge" edit - the more
// realistic case for interactively editing a sorted list, e.g. bumping a score/counter by a
// little), which keeps the row's reorder distance in `mapping` small on average.

use i_slint_core::model::{Model, SortModel, VecModel};
use std::env;
use std::hint::black_box;
use std::process;
use std::rc::Rc;
use std::time::Instant;

/// Deterministic scramble so the mapping's sort order differs from the source's index order.
fn scrambled_values(n: usize, seed: u64) -> Vec<u64> {
    let mut v: Vec<u64> = (0..n as u64).collect();
    let mut state: u64 = seed;
    for i in (1..v.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let j = (state as usize) % (i + 1);
        v.swap(i, j);
    }
    v
}

struct RunResult {
    elapsed_us: u64,
    checksum: u64,
}

fn one_run(n: usize, updates: usize, seed: u64, max_delta: u64) -> RunResult {
    let initial = scrambled_values(n, seed);
    let model = Rc::new(VecModel::from(initial.clone()));
    let sort_model = SortModel::new_ascending(model);

    // Force the mapping to be built before timing, so the initial sort isn't attributed to
    // the per-update cost this bench targets.
    let _ = black_box(sort_model.row_data(0));

    // Precompute a deterministic sequence of (wrapped-row, new-value) pairs up front, so the
    // PRNG itself isn't part of the timed section, and the exact same sequence of edits is
    // applied regardless of build (needed for a bit-exact checksum comparison). Picking the
    // *wrapped* (original) row directly, and driving the edit through the source model, means
    // this exercises exactly the same `row_changed(row)` notification `SortModelInner` receives
    // in real use, independent of the current sort order.
    let value_bound = n as u64 * 4;
    let mut state = seed ^ 0xD1B54A32D192ED03;
    let mut current = initial;
    let mut edits: Vec<(usize, u64)> = Vec::with_capacity(updates);
    for _ in 0..updates {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let wrapped_row = (state as usize) % n;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let new_value = if max_delta == 0 {
            state % value_bound
        } else {
            let old_value = current[wrapped_row] as i64;
            let delta = (state % (2 * max_delta + 1)) as i64 - max_delta as i64;
            (old_value + delta).clamp(0, value_bound as i64 - 1) as u64
        };
        current[wrapped_row] = new_value;
        edits.push((wrapped_row, new_value));
    }

    let source = sort_model.source_model();
    let t0 = Instant::now();
    for &(wrapped_row, new_value) in &edits {
        source.set_row_data(wrapped_row, new_value);
    }
    let elapsed_us = t0.elapsed().as_micros() as u64;

    let mut checksum: u64 = 0;
    for i in 0..n {
        checksum = checksum.wrapping_add(black_box(sort_model.row_data(i).unwrap()).wrapping_mul(i as u64 + 1));
    }

    RunResult { elapsed_us, checksum }
}

fn main() {
    let mut args = env::args().skip(1);
    let n: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: bench_sort_model_row_changed <row_count> <updates> [iters=15] [seed=1]");
        process::exit(1);
    });
    let updates: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: bench_sort_model_row_changed <row_count> <updates> [iters=15] [seed=1]");
        process::exit(1);
    });
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(15);
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let max_delta: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    println!("row_count = {n}, updates = {updates}, iters = {iters}, seed = {seed}, max_delta = {max_delta}");

    let mut times_us: Vec<u64> = Vec::with_capacity(iters);
    let mut checksum: Option<u64> = None;
    for i in 0..iters {
        let result = one_run(n, updates, seed.wrapping_add(i as u64), max_delta);
        times_us.push(result.elapsed_us);
        // Each iteration uses a different seed (deterministic per-index), so checksums are only
        // compared run-to-run (baseline vs patched) at the same seed, printed per-iteration.
        println!("  iter {i}: elapsed = {:8.3} ms  checksum = {:#018x}", result.elapsed_us as f64 / 1000.0, result.checksum);
        checksum = Some(result.checksum);
    }

    let warmup = (iters / 10).max(1).min(iters.saturating_sub(1));
    let mut stable: Vec<u64> = times_us[warmup..].to_vec();
    stable.sort_unstable();
    let sn = stable.len();
    let min = stable[0];
    let p50 = stable[sn / 2];
    let mean = stable.iter().sum::<u64>() / sn as u64;
    let p95 = stable[(sn * 95 / 100).min(sn - 1)];
    let max = stable[sn - 1];
    println!(
        "min = {:8.3} ms  p50 = {:8.3} ms  mean = {:8.3} ms  p95 = {:8.3} ms  max = {:8.3} ms  last_checksum = {:#018x}",
        min as f64 / 1000.0,
        p50 as f64 / 1000.0,
        mean as f64 / 1000.0,
        p95 as f64 / 1000.0,
        max as f64 / 1000.0,
        checksum.unwrap()
    );
}
