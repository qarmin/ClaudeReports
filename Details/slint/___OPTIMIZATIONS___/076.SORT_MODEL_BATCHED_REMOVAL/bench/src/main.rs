// Standalone microbenchmark for `SortModelInner::row_removed` (internal/core/model/adapters.rs).
//
// A wrapped model that emits a single batched `row_removed(index, count)` notification (as a
// bulk-delete API on a real data source would) forces `SortModel` to scan its `mapping` and pull
// out every removed entry. The baseline implementation calls `mapping.remove(i)` (an O(n) Vec
// memmove) once per removed row inside that scan, making a single batched event O(n x count)
// overall. This bench isolates just that call, excluding model/SortModel setup, across a range
// of `count` values at a fixed model size to show the scaling.
//
// Usage:
//   bench_sort_model_remove <row_count> <count_csv> [iters=15]
// Example:
//   bench_sort_model_remove 200000 1,50,200,1000,5000 15

use i_slint_core::model::{Model, ModelNotify, ModelTracker};
use std::cell::RefCell;
use std::env;
use std::hint::black_box;
use std::process;
use std::rc::Rc;
use std::time::Instant;

/// A minimal model, standing in for any data source whose backing store supports a bulk
/// delete (a DB view, a batched IPC update, ...) and therefore emits `row_removed(index, count)`
/// once instead of `count` times.
struct BulkModel {
    array: RefCell<Vec<u64>>,
    notify: ModelNotify,
}

impl Model for BulkModel {
    type Data = u64;

    fn row_count(&self) -> usize {
        self.array.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.array.borrow().get(row).copied()
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BulkModel {
    fn bulk_remove(&self, index: usize, count: usize) {
        self.array.borrow_mut().drain(index..index + count);
        self.notify.row_removed(index, count);
    }
}

/// Deterministic scramble so the mapping's sort order differs from the source's index order
/// (a real sorted view - leaderboard, sorted log - never happens to match insertion order).
fn scrambled_values(n: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..n as u64).collect();
    let mut state: u64 = 0x9E3779B97F4A7C15;
    for i in (1..v.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let j = (state as usize) % (i + 1);
        v.swap(i, j);
    }
    v
}

fn one_run(n: usize, count: usize) -> u64 {
    let model = Rc::new(BulkModel { array: RefCell::new(scrambled_values(n)), notify: Default::default() });
    let sort_model = i_slint_core::model::SortModel::new_ascending(model);

    // Force the mapping to be built before timing, so setup cost isn't attributed to removal.
    let _ = black_box(sort_model.row_data(0));

    let source: &BulkModel = sort_model.source_model().as_ref();
    let t0 = Instant::now();
    source.bulk_remove(0, count);
    let elapsed_us = t0.elapsed().as_micros() as u64;

    // Touch the result so the removal (and the notification it drives) can't be optimized away.
    black_box(sort_model.row_count());
    elapsed_us
}

fn main() {
    let mut args = env::args().skip(1);
    let n: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("usage: bench_sort_model_remove <row_count> <count_csv> [iters=15]");
        process::exit(1);
    });
    let counts: Vec<usize> = args
        .next()
        .map(|s| s.split(',').map(|c| c.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![1, 50, 200, 1000, 5000]);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(15);

    println!("row_count = {n}, iters = {iters}");
    for &count in &counts {
        let mut times_us: Vec<u64> = Vec::with_capacity(iters);
        for _ in 0..iters {
            times_us.push(one_run(n, count));
        }
        let warmup = (iters / 10).max(1).min(iters.saturating_sub(1));
        let mut stable: Vec<u64> = times_us[warmup..].to_vec();
        stable.sort_unstable();
        let sn = stable.len();
        let min = stable[0];
        let p50 = stable[sn / 2];
        let mean = stable.iter().sum::<u64>() / sn as u64;
        let max = stable[sn - 1];
        println!(
            "count = {count:6}  min = {:8.3} ms  p50 = {:8.3} ms  mean = {:8.3} ms  max = {:8.3} ms",
            min as f64 / 1000.0,
            p50 as f64 / 1000.0,
            mean as f64 / 1000.0,
            max as f64 / 1000.0
        );
    }
}
