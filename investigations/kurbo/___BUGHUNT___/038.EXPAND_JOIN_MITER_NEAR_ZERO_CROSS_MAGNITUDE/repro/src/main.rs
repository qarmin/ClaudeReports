// Candidate 038: EXPAND_JOIN_MITER_NEAR_ZERO_CROSS_MAGNITUDE
//
// Hypothesis under test: in `ExpandCtx::do_join`'s `Join::Miter` arm (kurbo/src/expand.rs),
// `h = (n - last_n).cross(tan) / self.last_tan.cross(tan)` divides by a cross product whose sign
// is checked (`cross * self.expand.xx < 0.0`) but whose *magnitude* is not - so in the
// near-parallel-tangent limit (tiny angle between consecutive segments) `cross` should approach
// zero while the numerator stays bounded, blowing `h` (and therefore `miter_pt`) up towards
// infinity, with no `is_finite` guard before `self.result.line_to(miter_pt)`.
//
// Both approaches below drive the real, public `kurbo::expand_path_signed` entry point directly
// (no hand-built internal state) - this is a single-process, single-caller API-misbehavior
// finding with no second participant to fake (see CONFIRMATION.md section 11's carve-out, same
// reasoning as candidate 033's repro).

use kurbo::{BezPath, Diagonal2, Join, PathEl, Point, expand_path_signed};
use std::panic::{self, AssertUnwindSafe};

/// Build a 4-point closed path with a controlled corner: from the origin, walk a segment at
/// angle `theta0`, then a second segment at angle `theta1` - the corner between them is what
/// `do_join`'s Miter arm processes. A tiny `theta1 - theta0` is the "near-parallel tangents"
/// case the candidate hypothesizes about.
fn build_path(theta0: f64, theta1: f64) -> BezPath {
    let mut p = BezPath::new();
    let a = Point::new(0.0, 0.0);
    let d0 = Point::new(theta0.cos(), theta0.sin());
    let b = Point::new(a.x + 100.0 * d0.x, a.y + 100.0 * d0.y);
    let d1 = Point::new(theta1.cos(), theta1.sin());
    let c = Point::new(b.x + 100.0 * d1.x, b.y + 100.0 * d1.y);
    p.move_to(a);
    p.line_to(b);
    p.line_to(c);
    p.line_to(Point::new(0.0, 100.0));
    p.close_path();
    p
}

fn has_nonfinite(path: &BezPath) -> bool {
    let bad = |p: &Point| !p.x.is_finite() || !p.y.is_finite();
    for el in path.elements() {
        let hit = match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => bad(p),
            PathEl::QuadTo(p1, p2) => bad(p1) || bad(p2),
            PathEl::CurveTo(p1, p2, p3) => bad(p1) || bad(p2) || bad(p3),
            PathEl::ClosePath => false,
        };
        if hit {
            return true;
        }
    }
    false
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Approach 1: targeted boundary-value sweep. Fixes `theta0 = 0.0` (empirically the direction of
/// highest sensitivity of `Diagonal2::scale_normal`'s output angle to a change in input angle,
/// for `expand = Diagonal2::new(xx, 1.0)` with `xx` large - i.e. exactly where the candidate's
/// hypothesized blow-up should be easiest to trigger) and sweeps the angle gap `theta1 - theta0`
/// geometrically from 1e-1 rad down to 1e-16 rad, both signs, across several anisotropy ratios,
/// via the real `expand_path_signed` API with `Join::Miter`.
fn approach1_targeted_sweep() -> (u32, u32) {
    let miter_limit = 1000.0;
    let tolerance = 1e-12;
    let mut checked = 0u32;
    let mut nonfinite = 0u32;
    for &xx in &[1e3, 1e6, 1e9, 1e12] {
        let expand = Diagonal2::new(xx, 1.0);
        for k in 0..300 {
            let gap_mag = 10f64.powf(-1.0 - 15.0 * (k as f64) / 299.0); // 1e-1 down to 1e-16
            for &sign in &[1.0, -1.0] {
                let gap = sign * gap_mag;
                let path = build_path(0.0, gap);
                let result = panic::catch_unwind(AssertUnwindSafe(|| {
                    expand_path_signed(path.clone(), expand, Join::Miter, miter_limit, tolerance)
                }));
                checked += 1;
                match result {
                    Ok(out) => {
                        if has_nonfinite(&out) {
                            nonfinite += 1;
                            if nonfinite <= 10 {
                                println!(
                                    "APPROACH1 NONFINITE xx={:e} gap={:e}",
                                    xx, gap
                                );
                            }
                        }
                    }
                    Err(_) => {
                        nonfinite += 1;
                        println!("APPROACH1 PANIC xx={:e} gap={:e}", xx, gap);
                    }
                }
            }
        }
    }
    (checked, nonfinite)
}

/// Approach 2: broad randomized sweep. Random corner angle `theta0` over the full circle, random
/// (signed) angle gap log-uniform in [1e-18, 1e-1] rad, random anisotropy `xx` log-uniform in
/// [1, 1e12] with `yy = 1.0` - again through the real public `expand_path_signed` API.
fn approach2_random_sweep(iters: u32) -> (u32, u32) {
    let miter_limit = 1000.0;
    let tolerance = 1e-12;
    let mut state = 0x243F6A8885A308D3u64;
    let mut checked = 0u32;
    let mut nonfinite = 0u32;
    for _ in 0..iters {
        let r1 = (xorshift(&mut state) as f64) / (u64::MAX as f64);
        let r2 = (xorshift(&mut state) as f64) / (u64::MAX as f64);
        let r3 = (xorshift(&mut state) as f64) / (u64::MAX as f64);
        let r4 = (xorshift(&mut state) as f64) / (u64::MAX as f64);
        let theta0 = -std::f64::consts::PI + 2.0 * std::f64::consts::PI * r1;
        let log_gap = -18.0 + 17.0 * r2;
        let gap = if r3 < 0.5 { 1.0 } else { -1.0 } * 10f64.powf(log_gap);
        let xx = 10f64.powf(12.0 * r4);
        let expand = Diagonal2::new(xx, 1.0);
        let path = build_path(theta0, theta0 + gap);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            expand_path_signed(path.clone(), expand, Join::Miter, miter_limit, tolerance)
        }));
        checked += 1;
        match result {
            Ok(out) => {
                if has_nonfinite(&out) {
                    nonfinite += 1;
                    if nonfinite <= 10 {
                        println!(
                            "APPROACH2 NONFINITE theta0={:e} gap={:e} xx={:e}",
                            theta0, gap, xx
                        );
                    }
                }
            }
            Err(_) => {
                nonfinite += 1;
                println!("APPROACH2 PANIC theta0={:e} gap={:e} xx={:e}", theta0, gap, xx);
            }
        }
    }
    (checked, nonfinite)
}

fn main() {
    let iters: u32 = std::env::var("ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000_000);

    println!("=== Approach 1: targeted near-parallel-tangent boundary sweep (public expand_path_signed) ===");
    let (c1, nf1) = approach1_targeted_sweep();
    println!("approach1 total={} nonfinite_or_panic={}", c1, nf1);
    println!();
    println!("=== Approach 2: broad randomized sweep (public expand_path_signed, ITERS={}) ===", iters);
    let (c2, nf2) = approach2_random_sweep(iters);
    println!("approach2 total={} nonfinite_or_panic={}", c2, nf2);
    println!();

    if nf1 > 0 || nf2 > 0 {
        println!("RESULT: REPRODUCED (approach1_hits={}, approach2_hits={})", nf1, nf2);
        std::process::exit(1);
    } else {
        println!("RESULT: did not reproduce (approach1_hits=0, approach2_hits=0)");
        std::process::exit(0);
    }
}
