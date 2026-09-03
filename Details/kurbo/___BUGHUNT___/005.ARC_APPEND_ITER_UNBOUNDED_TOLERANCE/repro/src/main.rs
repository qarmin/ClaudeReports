// Repro for: Arc::append_iter computes an effectively unbounded segment count for
// a near-zero (including exactly zero) `tolerance`.
//
// Exercises the real public API end-to-end: `kurbo::Arc::append_iter` (and the
// `to_cubic_beziers` wrapper around it), the same entry points any caller converting
// an `Arc` into cubic Beziers would use. No hand-authored internal state - `tolerance`
// is a normal f64 parameter a caller passes directly.
//
// A cutoff (wall-clock + a generous element cap) stops the loop instead of letting it
// run for real up to usize::MAX iterations, which is the whole point being demonstrated:
// on the buggy code this cutoff is what actually fires (the iterator never finishes on
// its own in any practical amount of time); on the fixed code the iterator finishes on
// its own, quickly, well under the cutoff.

use kurbo::Arc;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

const WALL_CLOCK_BUDGET: Duration = Duration::from_secs(5);
const ELEMENT_CAP: u64 = 20_000_000;

fn drive(label: &str, arc: Arc, tolerance: f64) -> bool {
    let start = Instant::now();
    let mut iter = arc.append_iter(tolerance);
    let mut count: u64 = 0;
    let finished = loop {
        if iter.next().is_none() {
            break true;
        }
        count += 1;
        if count >= ELEMENT_CAP || start.elapsed() > WALL_CLOCK_BUDGET {
            break false;
        }
    };
    let elapsed = start.elapsed();
    println!(
        "[{label}] tolerance={tolerance:e} -> {count} elements in {elapsed:?}, iterator finished on its own: {finished}"
    );
    finished
}

fn main() {
    // The reported trigger: a half-circle Arc with tolerance == 0.0.
    // `to_cubic_beziers` is a thin wrapper that repeatedly calls `.next()` on the exact
    // same `append_iter` iterator until it returns `None` - it shares the identical `n`
    // computation this repro exercises directly, so testing `append_iter` here covers
    // both public entry points without needing an unguarded second run that could hang.
    let arc = Arc::new((0.0, 0.0), (10.0, 10.0), 0.0, PI, 0.0);
    let ok = drive("append_iter, tolerance=0.0", arc, 0.0);

    if !ok {
        println!(
            "FAIL: Arc::append_iter did not terminate in a reasonable time/element budget for tolerance=0.0"
        );
        std::process::exit(1);
    } else {
        println!("PASS: Arc::append_iter terminated promptly for tolerance=0.0");
    }
}
