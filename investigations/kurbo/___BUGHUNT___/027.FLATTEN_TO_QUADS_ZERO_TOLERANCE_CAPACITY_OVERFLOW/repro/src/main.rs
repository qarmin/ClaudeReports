// Repro for: `kurbo::flatten` (the crate's public path-flattening entry point) panics with
// "capacity overflow" for a `CurveTo` segment when called with `tolerance == 0.0`, and separately
// spins effectively forever for a `QuadTo` segment with the same `tolerance` on certain quad
// shapes.
//
// Exercises the real public API end-to-end: `kurbo::flatten(path, tolerance, callback)`, the same
// entry point `kurbo::bezpath::BezPath::flatten` and every renderer built on this crate uses to
// turn a path into polylines. No hand-authored internal state - `tolerance` is a normal `f64`
// parameter a caller passes directly, and the path is built via the crate's own public
// `BezPath`/`move_to`/`curve_to`/`quad_to` builders. This is a single-participant repro (a
// malformed/edge-case parameter handed straight to a parser-like public API) - there is no second
// participant/peer to fake, per CONFIRMATION.md section 11's own carve-out.
//
// Case A (the originally reported mechanism): a `CurveTo` element with `tolerance = 0.0`.
// `CubicBez::to_quads` computes a segment count `n` that becomes `+inf` (saturating to
// `usize::MAX` on the `f64 as usize` cast) because it divides by `432.0 * accuracy * accuracy`
// with `accuracy == 0.0`. `flatten`'s cubic arm immediately calls
// `quad_buf.reserve(iter.size_hint().0)` with that `usize::MAX`, which panics with
// "capacity overflow" before a single point is ever produced.
//
// Case B (an equivalent mechanism at the QuadTo call site, same root cause class, bundled into
// this same candidate): a `QuadTo` element with `tolerance = 0.0` and a quad shape whose
// `estimate_subdiv` value doesn't itself collapse to NaN for `sqrt_tol == 0.0` (unlike a
// symmetric "hump" quad, which happens to yield NaN and safely no-ops). For such a shape, the same
// division-by-zero-tolerance pattern makes the loop-bound `n` saturate to `usize::MAX`, and
// because this call site loops directly (`for i in 1..n`) instead of calling `Vec::reserve`, it
// doesn't panic - it just never finishes in any practical amount of time. A wall-clock + element
// cutoff distinguishes "still spinning" from "finished on its own", the same technique candidate
// 005's own repro (`ARC_APPEND_ITER_UNBOUNDED_TOLERANCE`) used for an analogous unbounded loop.

use kurbo::{flatten, BezPath, PathEl};
use std::panic::AssertUnwindSafe;
use std::time::{Duration, Instant};

const WALL_CLOCK_BUDGET: Duration = Duration::from_secs(5);
const ELEMENT_CAP: u64 = 5_000_000;

// Case A: run the real `flatten` entry point and check whether it panics.
// Returns true (bug NOT present) if it completes normally; false (bug present) if it panics with
// "capacity overflow".
fn case_a_curveto_capacity_overflow() -> bool {
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.curve_to((10.0, 10.0), (20.0, -10.0), (30.0, 0.0));
        let mut count = 0usize;
        flatten(path, 0.0, |_el: PathEl| {
            count += 1;
        });
        count
    }));

    match result {
        Ok(count) => {
            println!(
                "[case A: CurveTo, tolerance=0.0] flatten() completed without panicking, {count} elements produced"
            );
            true
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!("[case A: CurveTo, tolerance=0.0] flatten() PANICKED: {msg}");
            false
        }
    }
}

// Case B: run the real `flatten` entry point on a QuadTo path that hits the same unguarded
// division, with a wall-clock + element-count cutoff so a still-buggy build doesn't hang the
// repro forever. Returns true (bug NOT present) if the iterator finishes on its own well within
// the cutoff; false (bug present) if the cutoff had to fire.
fn case_b_quadto_unbounded_loop() -> bool {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    // This specific shape was found (by direct instrumentation of `estimate_subdiv` during this
    // session's investigation) to land in the non-cusp branch, whose `val` does not depend on
    // `sqrt_tol` at all - so `0.5 * val / sqrt_tol` is a genuine `+inf`, not a NaN that happens
    // to safely collapse to `n = 1`, for `tolerance = 0.0`.
    path.quad_to((0.0, 1.0), (0.5, 1.5));

    let start = Instant::now();
    let mut count: u64 = 0;
    let mut cutoff_fired = false;
    let mut finished_normally = false;
    // flatten() takes an FnMut callback with no way to abort from inside, so we detect
    // "still going past the cutoff" via a panic thrown from the callback itself, caught here -
    // this is the same "the real entry point either returns or we forcibly stop it" pattern
    // candidate 005 used for `Arc::append_iter`'s truly unbounded loop.
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        flatten(path, 0.0, |_el: PathEl| {
            count += 1;
            if count >= ELEMENT_CAP || start.elapsed() > WALL_CLOCK_BUDGET {
                panic!("CUTOFF");
            }
        });
    }));
    match result {
        Ok(()) => finished_normally = true,
        Err(payload) => {
            let is_cutoff = payload
                .downcast_ref::<&str>()
                .map(|s| *s == "CUTOFF")
                .unwrap_or(false);
            if is_cutoff {
                cutoff_fired = true;
            } else {
                // Some other, unrelated panic - re-raise so it isn't silently swallowed.
                std::panic::resume_unwind(payload);
            }
        }
    }
    let elapsed = start.elapsed();
    println!(
        "[case B: QuadTo, tolerance=0.0] {count} elements in {elapsed:?}, finished on its own: {finished_normally}, cutoff fired: {cutoff_fired}"
    );
    finished_normally
}

fn main() {
    // Silence the default panic hook's noisy backtrace for the *expected* panics above - we
    // print our own summary line for each case regardless of outcome.
    std::panic::set_hook(Box::new(|_info| {}));

    let a_ok = case_a_curveto_capacity_overflow();
    let b_ok = case_b_quadto_unbounded_loop();

    if a_ok && b_ok {
        println!("PASS: flatten() handled tolerance=0.0 safely in both cases");
    } else {
        println!("FAIL: flatten() mishandled tolerance=0.0 (see per-case output above)");
        std::process::exit(1);
    }
}
