// Standalone repro for candidate 046 (DASH_ZERO_PERIOD_NAN_SILENT_SOLID_LINE).
//
// Calls kurbo's real, public `dash()` entry point (kurbo::dash) directly with a plain `&[f64]`
// dash-pattern argument, exactly as any real caller would. The `dashes: &[f64]` parameter is a
// plain function argument the caller supplies directly - there is no second participant/peer to
// fake here (see CONFIRMATION.md section 11's own carve-out for single-artifact input), so a
// single-process, single-call repro is the correct shape.
//
// It demonstrates two related failures that both stem from `dash_iter` (kurbo/src/stroke.rs)
// never checking that the dash pattern's period (`dashes.iter().sum()`, doubled for odd-length
// arrays) is strictly positive before using it:
//
//   1. period == 0.0 (e.g. &[0.0, 0.0], or &[-3.0, 3.0]): `dash_offset.rem_euclid(period)`
//      becomes NaN, is_active is initialized true and never toggled, and the dash pattern is
//      silently ignored - the whole path is emitted as one continuous, undashed segment.
//   2. period < 0.0 (e.g. &[-5.0, -5.0]): rem_euclid with a negative divisor is well-defined (no
//      NaN), but the subsequent `while dash_remaining < 0.0 { ...; dash_remaining += dashes[ix]; }`
//      search loop only ever adds more negative dash lengths to `dash_remaining`, so it can never
//      reach `>= 0.0` and the call hangs forever. This is checked with a bounded timeout so this
//      repro itself terminates and reports the hang instead of hanging along with the bug.

use kurbo::{dash, BezPath, PathEl, Point};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn undashed_line() -> BezPath {
    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, 0.0));
    path.line_to(Point::new(100.0, 0.0));
    path
}

fn run_dash(dashes: Vec<f64>, offset: f64) -> BezPath {
    let path = undashed_line();
    dash(path.into_iter(), offset, &dashes).collect()
}

/// Runs `run_dash` on its own thread with a timeout, since case 2 above is an infinite loop in
/// the pristine code and must not be allowed to hang this repro itself.
fn run_dash_with_timeout(dashes: Vec<f64>, offset: f64, timeout: Duration) -> Option<BezPath> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = run_dash(dashes, offset);
        // The receiver may already have timed out and gone away; ignore that.
        let _ = tx.send(result);
    });
    rx.recv_timeout(timeout).ok()
}

fn main() {
    let expected_undashed: Vec<PathEl> = undashed_line().into_iter().collect();
    let mut all_ok = true;

    println!("-- Class 1: period == 0.0 (NaN via rem_euclid(x, 0.0)) --");
    for (dashes, offset) in [
        (vec![0.0, 0.0], 0.0),
        (vec![-3.0, 3.0], 0.0),
        (vec![3.0, -3.0], 0.0),
        (vec![0.0, 0.0], 5.0),
        (vec![0.0, 0.0, 0.0, 0.0], 0.0),
        (vec![0.0], 0.0), // odd length, doubled by SVG rule -> period 0.0 too
    ] {
        let result = run_dash_with_timeout(dashes.clone(), offset, Duration::from_secs(5));
        match result {
            Some(path) => {
                let els: Vec<PathEl> = path.clone().into_iter().collect();
                let silently_undashed = els == expected_undashed;
                println!(
                    "dashes={dashes:?} offset={offset} -> {path:?}  (dash pattern silently ignored, rendered as solid line: {silently_undashed})"
                );
                if !silently_undashed {
                    println!("  UNEXPECTED: did not collapse to a silent solid line for this candidate's own repro purposes");
                }
            }
            None => {
                println!("dashes={dashes:?} offset={offset} -> TIMED OUT (unexpected for this class)");
                all_ok = false;
            }
        }
    }

    println!();
    println!("-- Class 2: period < 0.0 (search loop diverges forever) --");
    for (dashes, offset) in [(vec![-5.0, -5.0], 0.0), (vec![-5.0, -5.0], 2.0)] {
        let result = run_dash_with_timeout(dashes.clone(), offset, Duration::from_secs(5));
        match result {
            Some(path) => {
                println!("dashes={dashes:?} offset={offset} -> completed: {path:?}");
            }
            None => {
                println!(
                    "dashes={dashes:?} offset={offset} -> HANG DETECTED: dash() did not return within 5s (infinite loop in dash_iter's init search loop)"
                );
                all_ok = false;
            }
        }
    }

    println!();
    if all_ok {
        println!("RESULT: dash() handled every degenerate (non-positive-period) pattern above without hanging or silently ignoring the pattern via NaN - fix appears to be applied.");
    } else {
        println!("RESULT: BUG REPRODUCED - dash() either silently ignored a degenerate dash pattern via NaN, or hung forever on a negative-period pattern.");
        std::process::exit(1);
    }
}
