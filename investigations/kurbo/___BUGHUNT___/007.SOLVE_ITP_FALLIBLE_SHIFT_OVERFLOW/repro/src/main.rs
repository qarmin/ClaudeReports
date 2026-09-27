// Standalone reproduction for candidate 007 (SOLVE_ITP_FALLIBLE_SHIFT_OVERFLOW).
//
// Drives kurbo's real public entry point (`ParamCurveArclen::inv_arclen`, called through
// `CubicBez`) with two concrete curves that are each real, single-artifact geometric input - no
// second participant is being faked here (see CONFIRMATION.md section 11: a `CubicBez` handed
// straight to a public method is the genuine external input, same as a caller would construct).
//
// Case A ("shift-overflow"): a curve long enough that `nmax` in
// `common::solve_itp_fallible` exceeds 63, so `1u64 << nmax` overflows a `u64` shift.
//   - debug build (overflow checks on): panics immediately ("attempt to shift left with
//     overflow").
//   - release build: the shift silently wraps (`nmax & 63` on x86_64), producing a bogus
//     `scaled_epsilon` that is a factor of 2^64 too small, and the ITP loop's bracket then never
//     shrinks below `2 * epsilon` - `inv_arclen` spins forever.
//
// Case B ("float-resolution-hang"): a shorter curve where `nmax` stays under 64 (no shift
// overflow at all, so this reproduces identically in debug and release), but `epsilon` is still
// finer than the spacing between adjacent `f64` values near the root. The bracket [a, b] shrinks
// until `a`/`b` are neighbouring floats, `xitp` keeps rounding back onto one of the endpoints,
// `a = xitp` (or `b = xitp`) becomes a no-op, and the `while b - a > 2.0 * epsilon` loop can never
// exit. This is the independent bug PR #600 calls "bug one" - PR #588's narrower fix (which only
// replaces the `u64` shift with `f64::exp2`) does not touch it.
//
// Because a hung `inv_arclen` call never returns (an unbounded spin, not a crash), each case runs
// on its own thread with a watchdog timeout on the main thread, so this repro itself always
// terminates with a clear, real captured result instead of needing an external `timeout(1)`
// wrapper to be readable.
//
// Exit code convention: 0 = neither case reproduced. Non-zero = at least one case reproduced
// (panicked or hung past the watchdog deadline) - the real, currently-affected behavior on the
// pristine tree before `patch.diff`, and the expected clean run after it.

use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc;
use std::time::Duration;

use kurbo::{CubicBez, ParamCurveArclen, Point};

const WATCHDOG: Duration = Duration::from_secs(5);

enum Outcome {
    Ok(f64),
    Panicked(String),
}

fn run_with_watchdog(name: &str, f: impl FnOnce() -> f64 + Send + 'static) -> bool {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = panic::catch_unwind(AssertUnwindSafe(f));
        let outcome = match result {
            Ok(t) => Outcome::Ok(t),
            Err(payload) => {
                let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<non-string panic payload>".to_string()
                };
                Outcome::Panicked(msg)
            }
        };
        // Best-effort: if the watchdog already gave up and dropped `rx`, this send fails and the
        // (still-hung, in the release/no-timeout-inside-kurbo case) thread is simply abandoned
        // when the process exits.
        let _ = tx.send(outcome);
    });

    match rx.recv_timeout(WATCHDOG) {
        Ok(Outcome::Ok(t)) => {
            println!("[{name}] did not reproduce - inv_arclen returned t = {t} within {WATCHDOG:?}");
            false
        }
        Ok(Outcome::Panicked(msg)) => {
            println!("[{name}] REPRODUCED - panicked: {msg}");
            true
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            println!(
                "[{name}] REPRODUCED - inv_arclen did not return within {WATCHDOG:?} (spinning, matches the release-build hang)"
            );
            true
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            println!("[{name}] REPRODUCED - worker thread died without a result");
            true
        }
    }
}

fn case_a_shift_overflow() -> bool {
    let c = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(1e12, 0.0),
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
    );
    let arclen = c.arclen(1e-9);
    println!("[case-a-shift-overflow] arclen = {arclen}");
    run_with_watchdog("case-a-shift-overflow", move || {
        let half = arclen * 0.5;
        c.inv_arclen(half, 1e-9)
    })
}

fn case_b_float_resolution_hang() -> bool {
    let c = CubicBez::new((0., 0.), (5e7, 0.), (5e7, 1e7), (1e8, 1e7));
    let arclen = c.arclen(1e-9);
    println!("[case-b-float-resolution-hang] arclen = {arclen}");
    run_with_watchdog("case-b-float-resolution-hang", move || {
        c.inv_arclen(arclen * 0.171, 1e-9)
    })
}

fn main() {
    let a_reproduced = case_a_shift_overflow();
    let b_reproduced = case_b_float_resolution_hang();

    if a_reproduced || b_reproduced {
        eprintln!(
            "REPRODUCED overall (case-a: {a_reproduced}, case-b: {b_reproduced})"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - both cases completed normally");
    }
}
