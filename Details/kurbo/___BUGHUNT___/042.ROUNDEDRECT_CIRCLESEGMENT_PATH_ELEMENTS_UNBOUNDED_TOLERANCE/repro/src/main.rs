// Repro for: RoundedRect::path_elements and CircleSegment::path_elements both delegate to
// Arc::append_iter(tolerance), which computes an effectively unbounded segment count for a
// near-zero (including exactly zero) `tolerance` - the same root cause already confirmed for
// `Arc` itself (candidate 005) and for `Circle`'s own independent implementation (candidate 028),
// but these are two more call sites hitting it, both funneled through `Arc::append_iter` rather
// than reimplementing the arithmetic themselves.
//
// Exercises the real public API end-to-end: `kurbo::Shape::path_elements` via `RoundedRect` and
// via `CircleSegment`, the same entry point any caller converting either shape into path segments
// would use (e.g. `Shape::to_path(0.0)`). No hand-authored internal state - `tolerance` is a
// normal f64 parameter a caller passes directly; there is no second participant/peer/wire format
// to fake here (CONFIRMATION.md section 11's own carve-out for a single external artifact
// applies).
//
// A cutoff (wall-clock + a generous element cap) stops the loop instead of letting it run for
// real up to usize::MAX iterations, which is the whole point being demonstrated: on the buggy
// code this cutoff is what actually fires (the iterator never finishes on its own in any
// practical amount of time); on the fixed code the iterator finishes on its own, quickly, well
// under the cutoff.

use kurbo::{CircleSegment, RoundedRect, Shape};
use std::time::{Duration, Instant};

const WALL_CLOCK_BUDGET: Duration = Duration::from_secs(5);
const ELEMENT_CAP: u64 = 20_000_000;

fn drive(label: &str, mut iter: impl Iterator<Item = kurbo::PathEl>) -> bool {
    let start = Instant::now();
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
    println!("[{label}] -> {count} elements in {elapsed:?}, iterator finished on its own: {finished}");
    finished
}

fn main() {
    let mut all_ok = true;

    let rr = RoundedRect::new(0.0, 0.0, 100.0, 100.0, 10.0);
    let ok = drive(
        "RoundedRect::path_elements, tolerance=0.0",
        rr.path_elements(0.0),
    );
    all_ok &= ok;
    if !ok {
        println!("FAIL: RoundedRect::path_elements did not terminate in a reasonable time/element budget for tolerance=0.0");
    }

    let cs = CircleSegment::new((0.0, 0.0), 100.0, 10.0, 0.0, std::f64::consts::PI);
    let ok = drive(
        "CircleSegment::path_elements, tolerance=0.0",
        cs.path_elements(0.0),
    );
    all_ok &= ok;
    if !ok {
        println!("FAIL: CircleSegment::path_elements did not terminate in a reasonable time/element budget for tolerance=0.0");
    }

    if all_ok {
        println!("PASS: RoundedRect::path_elements and CircleSegment::path_elements both terminated promptly for tolerance=0.0");
    } else {
        std::process::exit(1);
    }
}
