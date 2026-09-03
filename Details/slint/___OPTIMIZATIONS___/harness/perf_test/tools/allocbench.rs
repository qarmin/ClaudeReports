//! Cost of the allocation pattern the property system produces, so that allocation counts can
//! be turned into milliseconds. Two patterns, because they differ by a lot:
//!
//! * `batch`  - allocate N boxes, then free all of them (what a growing cache does).
//! * `churn`  - allocate and free N boxes with the frees interleaved, the same size class every
//!   time (what per-frame `DependencyNode` traffic does: the list is rebuilt every frame, so the
//!   allocator's per-size-class free list stays hot).
//!
//! Run: cargo run --release [n]   (or `just allocbench`)
//! With mimalloc: rustc --cfg use_mimalloc ... (see justfile recipe `allocbench-mimalloc`)

use std::time::{Duration, Instant};

const ROUNDS: usize = 20;

fn batch(n: usize) -> Duration {
    let mut total = Duration::ZERO;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        let mut kept: Vec<Box<[usize; 4]>> = Vec::with_capacity(n);
        for i in 0..n {
            kept.push(Box::new([i, i, i, i]));
        }
        drop(kept);
        total += start.elapsed();
    }
    total / ROUNDS as u32
}

fn churn(n: usize) -> Duration {
    // Keep a small window of live boxes, like a linked list that is rebuilt every frame.
    const WINDOW: usize = 64;
    let mut window: Vec<Box<[usize; 4]>> = Vec::with_capacity(WINDOW);
    let mut total = Duration::ZERO;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        for i in 0..n {
            if window.len() == WINDOW {
                window.clear();
            }
            window.push(Box::new([i, i, i, i]));
        }
        window.clear();
        total += start.elapsed();
    }
    total / ROUNDS as u32
}

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(700_000);
    let batch = batch(n);
    let churn = churn(n);
    println!(
        "batch  {n} x (alloc 32B) then free all: {batch:.2?} per round = {:.1} ns per pair",
        batch.as_nanos() as f64 / n as f64
    );
    println!(
        "churn  {n} x (alloc 32B + free, hot free list): {churn:.2?} per round = {:.1} ns per pair",
        churn.as_nanos() as f64 / n as f64
    );
}
