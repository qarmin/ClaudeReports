// Candidate 035: kurbo/src/expand.rs `two_point_linear` (called from `do_cubic` when
// `try_one_point` returns `None`) evaluates `cx.q.eval(t).to_vec2().normalize()` at t=0.35 and
// t=0.65 with no guard against a zero-length derivative vector, unlike its sibling
// `try_one_point`, which explicitly bails out (`if tan_hypot2 < 1e-12 { return None; }`) before
// normalizing the analogous t=0.5 sample.
//
// This program constructs cubic Beziers with a genuine cusp (an exact (0,0) derivative vector,
// independently verified via the crate's own public `CubicBez::deriv()`/`ParamCurve::eval`) at
// many non-midpoint parameter locations, feeds them through the real, public
// `expand_path_signed` entry point, and checks whether any NaN reaches the emitted output. This
// is a single-participant repro (a path handed to a path-expansion function is the genuine
// external input; there is no second participant to fake here - see CONFIRMATION.md section 11).
//
// Result: this does NOT reproduce - see RESULT.md for the root-cause explanation. In short:
// whenever `cx.q.eval(t)` is exactly the zero vector at one sample point, `normalize()` does
// produce a NaN Vec2 for that sample's `utan`, but every downstream quantity derived from it
// (`a_n`, `b_n`, `c_n`) also becomes NaN, which makes `two_point_linear`'s own `det` (which
// combines `a_n`/`b_n` from *both* samples) NaN too - and the existing check
// `det.abs().partial_cmp(&1e-12) != Some(Ordering::Greater)` explicitly treats a NaN determinant
// as failure (see its own comment: "Consider both near-zero and NaN determinants to be
// failures."), so `two_point_linear` returns `None` and `do_cubic` safely falls back to
// `do_line(p3)` instead of ever emitting the NaN.

use kurbo::{BezPath, CubicBez, Diagonal2, Join, ParamCurve, ParamCurveDeriv, Point, Vec2};

fn coeffs(t: f64) -> (f64, f64, f64) {
    let mt = 1.0 - t;
    (mt * mt, mt * 2.0 * t, t * t)
}

/// Build a cubic Bezier (p0,p1,p2,p3) whose derivative curve is *solved* to have a zero vector at
/// `cusp_t`, given fixed (non-parallel) endpoint tangent directions `q0`/`q2`. Due to floating
/// point rounding this doesn't always land on a bit-exact (0,0) at `cusp_t` - `run_case` checks
/// and reports that separately, only counting bit-exact cases as genuine NaN-triggering cusps.
fn build_cubic_with_cusp(cusp_t: f64, q0: Vec2, q2: Vec2) -> [Point; 4] {
    let (c, d, e) = coeffs(cusp_t);
    let q1 = -(q0 * c + q2 * e) / d;
    let p0 = Point::new(0.0, 0.0);
    let p1 = p0 + q0 / 3.0;
    let p2 = p1 + q1 / 3.0;
    let p3 = p2 + q2 / 3.0;
    [p0, p1, p2, p3]
}

fn has_nan(path: &BezPath) -> bool {
    path.elements().iter().any(|el| {
        let pts: Vec<Point> = match el {
            kurbo::PathEl::MoveTo(p) => vec![*p],
            kurbo::PathEl::LineTo(p) => vec![*p],
            kurbo::PathEl::QuadTo(p1, p2) => vec![*p1, *p2],
            kurbo::PathEl::CurveTo(p1, p2, p3) => vec![*p1, *p2, *p3],
            kurbo::PathEl::ClosePath => vec![],
        };
        pts.iter().any(|p| p.x.is_nan() || p.y.is_nan())
    })
}

/// Returns `(is_exact_cusp, nan_in_output)`.
fn run_case(cusp_t: f64, q0: Vec2, q2: Vec2, verbose: bool) -> (bool, bool) {
    let [p0, p1, p2, p3] = build_cubic_with_cusp(cusp_t, q0, q2);
    if !p1.x.is_finite() || !p2.x.is_finite() || !p1.y.is_finite() || !p2.y.is_finite() {
        return (false, false);
    }

    // Independently verify (via the crate's own public CubicBez/deriv API, not by re-deriving the
    // math ourselves) whether the derivative is really the exact zero vector at cusp_t - i.e.
    // whether this construction actually landed on a genuine cusp at the target t, which is the
    // only condition under which `Vec2::normalize()` produces NaN (a magnitude of merely 1e-17,
    // for example, still normalizes to a well-defined unit vector, no NaN).
    let c = CubicBez::new(p0, p1, p2, p3);
    let deriv_at_cusp = c.deriv().eval(cusp_t).to_vec2();
    let is_exact_cusp = deriv_at_cusp == Vec2::ZERO;

    let mut path = BezPath::new();
    path.move_to(p0);
    path.curve_to(p1, p2, p3);
    path.line_to(p3 + Vec2::new(0.0, 5.0));
    path.line_to(p0 + Vec2::new(0.0, 5.0));
    path.close_path();

    let expanded = kurbo::expand_path_signed(&path, Diagonal2::new(1.0, 1.0), Join::Bevel, 4.0, 1e-3);
    let nan = has_nan(&expanded);
    if verbose {
        println!(
            "cusp_t={cusp_t:<6} q0={q0:?} q2={q2:?} exact_cusp={is_exact_cusp} -> NaN in output: {nan}"
        );
        if verbose && is_exact_cusp {
            println!("  expanded = {expanded:?}");
        }
    }
    (is_exact_cusp, nan)
}

fn main() {
    println!("== Single, headline case: exact cusp at t=0.35 (collinear tangents) ==");
    let (is_exact, nan0) = run_case(0.35, Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0), true);
    assert!(is_exact, "headline case failed to produce an exact cusp");
    let mut any_nan = nan0;

    println!();
    println!("== Broader sweep: ~1000 candidate cusp locations x 4 tangent-direction pairs ==");
    let mut ts: Vec<f64> = Vec::new();
    let mut t: f64 = 0.02;
    while t < 0.98 {
        if (t - 0.5).abs() > 1e-6 {
            ts.push(t);
        }
        t += 0.001;
    }
    let dir_pairs = [
        (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)),
        (Vec2::new(1.0, 0.3), Vec2::new(-0.5, 1.0)),
        (Vec2::new(2.0, -1.0), Vec2::new(1.0, 2.0)),
        (Vec2::new(0.1, 1.0), Vec2::new(1.0, -0.2)),
    ];
    let mut n_tested = 0;
    let mut n_exact_cusps = 0;
    let mut n_nan = 0;
    for (q0, q2) in dir_pairs {
        for &cusp_t in &ts {
            let (is_exact, nan) = run_case(cusp_t, q0, q2, false);
            n_tested += 1;
            if is_exact {
                n_exact_cusps += 1;
            }
            if nan {
                n_nan += 1;
                any_nan = true;
                println!("  NaN found at cusp_t={cusp_t} q0={q0:?} q2={q2:?}");
            }
        }
    }
    println!(
        "tested {n_tested} locations ({n_exact_cusps} were genuine bit-exact-zero-derivative \
         cusps, i.e. cases where normalize() really did compute 0.0/0.0 = NaN internally), \
         NaN reached the output in {n_nan} of them"
    );

    if any_nan {
        eprintln!("REPRODUCED - expand_path_signed emitted NaN control point(s)");
        std::process::exit(1);
    } else {
        println!();
        println!("did not reproduce - no NaN in any output, across all cases tried");
    }
}
