// Repro for: Circle::path_elements computes an effectively unbounded segment count for
// a near-zero (including exactly zero) `tolerance`.
//
// Exercises the real public API end-to-end: `kurbo::Shape::path_elements` (via `Circle`),
// the same entry point any caller converting a `Circle` into path segments would use
// (e.g. `Shape::to_path(0.0)`). No hand-authored internal state - `tolerance` is a normal
// f64 parameter a caller passes directly; there is no second participant/peer/wire format
// to fake here (CONFIRMATION.md section 11's own carve-out for a single external artifact
// applies).
//
// A cutoff (wall-clock + a generous element cap) stops the loop instead of letting it run
// for real up to usize::MAX iterations, which is the whole point being demonstrated: on
// the buggy code this cutoff is what actually fires (the iterator never finishes on its
// own in any practical amount of time); on the fixed code the iterator finishes on its
// own, quickly, well under the cutoff.

use kurbo::{Circle, Shape};
use std::time::{Duration, Instant};

const WALL_CLOCK_BUDGET: Duration = Duration::from_secs(5);
const ELEMENT_CAP: u64 = 20_000_000;

fn drive(label: &str, circle: Circle, tolerance: f64) -> bool {
    let start = Instant::now();
    let mut iter = circle.path_elements(tolerance);
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
    // The reported trigger: a radius-10 circle with tolerance == 0.0.
    let circle = Circle::new((0.0, 0.0), 10.0);
    let ok = drive("path_elements, tolerance=0.0", circle, 0.0);

    if !ok {
        println!(
            "FAIL: Circle::path_elements did not terminate in a reasonable time/element budget for tolerance=0.0"
        );
        std::process::exit(1);
    } else {
        println!("PASS: Circle::path_elements terminated promptly for tolerance=0.0");
    }
}
