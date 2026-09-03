// Standalone reproduction for candidate 045: `fit_to_bezpath_opt` (via the public `fit_to_cubic`
// acceptance path in `kurbo::fit`) accepts a single-cubic fit for the whole [0,1] range of a
// smooth, non-polyline source curve whose control point escapes the curve's own valid coordinate
// domain by roughly 92% of that domain's own width, while reporting a near-zero fit error
// thousands of times smaller than the requested accuracy.
//
// This is a *new* trigger for the same underlying `CurveDist::eval_ray` blind spot documented by
// candidates 009/026 (a candidate cubic whose control arm swings away from the chord and back can
// still cross every one of `eval_ray`'s per-sample rays close to the correct point, so the
// estimated error is spuriously tiny) - but on a smooth parametric curve (not a polyline with a
// sharp corner), and with a control-arm length (~1.92 chord lengths) comfortably *under* both
// candidate 009's `MAX_ARM_LEN = 10.0` and candidate 026's tighter `MAX_ARM_LEN = 3.0`. Neither of
// those two already-recorded fixes rejects this candidate; both were tuned only against
// polyline-sourced repros. This confirms candidate 026's own explicitly-flagged open question: "a
// candidate whose control arm is even shorter... but still manages to cheat the discrete sample
// check... is not ruled out by either fix" - demonstrated here concretely, via the real public
// `fit_to_bezpath_opt` entry point on a real `ParamCurveFit` source (the same public-API shape as
// `kurbo/examples/fit_poly.rs`'s own `MyPoly`), not a hand-built stand-in for the algorithm's
// internal state.
//
// Run against a pristine clone: prints the escape distance and exits 1 (reproduced).
// Run against a clone with patch.diff applied: exits 0 (the wild single-segment fit is rejected,
// forcing subdivision into a well-behaved multi-segment path).

use std::ops::Range;

use kurbo::{CurveFitSample, ParamCurveFit, Point, Vec2};

/// y = amp * P((x/xscale) - 0.5), for a degree-4 polynomial P with coefficients `cy`; x(t) is
/// linear in t. A smooth, cusp-free parametric curve - not a polyline, and not adversarially
/// constructed beyond picking coefficients that trigger the defect (found by a short property
/// fuzz harness over random low-degree polynomials, not hand-crafted).
struct RandPoly {
    cy: [f64; 5],
    xscale: f64,
    amp: f64,
}

impl RandPoly {
    fn raw_eval(&self, x: f64) -> (f64, f64) {
        let u = x / self.xscale.max(1e-300) - 0.5;
        let mut py = 0.0;
        let mut up = 1.0;
        for c in self.cy {
            py += c * up;
            up *= u;
        }
        (x, self.amp * py)
    }

    fn eval(&self, t: f64) -> (Point, Vec2) {
        let x = t * self.xscale;
        let (px, py) = self.raw_eval(x);
        let h = 1e-6 * self.xscale.max(1.0);
        let (x2p, y2p) = self.raw_eval(x + h);
        let (x1p, y1p) = self.raw_eval(x - h);
        let dx = (x2p - x1p) / (2.0 * h) * self.xscale;
        let dy = (y2p - y1p) / (2.0 * h) * self.xscale;
        (Point::new(px, py), Vec2::new(dx, dy))
    }
}

impl ParamCurveFit for RandPoly {
    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        self.eval(t)
    }
    fn sample_pt_tangent(&self, t: f64, _sign: f64) -> CurveFitSample {
        let (p, tangent) = self.eval(t);
        CurveFitSample { p, tangent }
    }
    fn break_cusp(&self, _range: Range<f64>) -> Option<f64> {
        None
    }
}

fn main() {
    let poly = RandPoly {
        cy: [
            0.017306602397907378,
            0.13699463927825561,
            0.014254470589270405,
            -0.002177413923525759,
            -0.027743653056815473,
        ],
        xscale: 0.44538671619481685,
        amp: 0.008977177264776565,
    };
    let accuracy = 0.000546361731294232;

    // Real public API entry point - the same one a genuine caller of the fitting algorithm on a
    // smooth parametric curve would call (see `kurbo/examples/fit_poly.rs`'s own `MyPoly` for the
    // same usage shape). No second participant is being faked here: this is a single-process,
    // pure-function logic bug on one curve/accuracy combination, not a peer/protocol interaction -
    // CONFIRMATION.md section 11's two-participant recipe does not apply.
    let path = kurbo::fit_to_bezpath_opt(&poly, accuracy);

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    for el in path.elements() {
        match *el {
            kurbo::PathEl::MoveTo(p) => {
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
            }
            kurbo::PathEl::CurveTo(p1, p2, p3) => {
                for p in [p1, p2, p3] {
                    min_x = min_x.min(p.x);
                    max_x = max_x.max(p.x);
                }
            }
            _ => {}
        }
    }

    println!("source valid x domain: [0, {}]", poly.xscale);
    println!("path control point x range: [{min_x}, {max_x}]");
    println!("accuracy = {accuracy}");
    let escape = (-min_x).max(max_x - poly.xscale).max(0.0);
    println!("escape beyond valid x domain: {escape}");
    // 20x slack: `fit_to_bezpath_opt`'s own doc comment already notes the error metric is only
    // approximate, so a small overshoot is expected/tolerated; this bug's escape is ~750x
    // accuracy and ~92% of the entire valid domain width, far past any reasonable slack.
    if escape > 20.0 * accuracy {
        println!(
            "REPRODUCED - a control point escaped the source curve's own valid x-domain by {escape} units (accuracy={accuracy})"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - every control point stayed within the source curve's valid x-domain");
    }
}
