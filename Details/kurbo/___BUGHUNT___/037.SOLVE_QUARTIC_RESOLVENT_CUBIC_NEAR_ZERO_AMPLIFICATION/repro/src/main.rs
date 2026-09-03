// Standalone repro for candidate 037: kurbo::common::solve_quartic silently returns fewer real
// roots than exist (in this case, zero roots for a quartic that provably has 4 real roots), when
// the quartic has two pairs of very-close-but-distinct real roots.
//
// This exercises the crate's real public API (`kurbo::common::solve_quartic`) directly with a
// single, self-contained coefficient input. Per CONFIRMATION.md section 11, this needs no second
// participant to fake: the quartic coefficients ARE the external input, exactly the way a real
// caller (e.g. `CubicBez::tangents_to_point`, the crate's own real geometric caller, which builds
// its `c0..c4` from a curve's control points and a query point and hands them to this same
// function) would hand them to `solve_quartic`.
use kurbo::common::solve_quartic;

/// Build the monic quartic coefficients (c0..c4, with c4 == 1.0) for
/// (x - r0)(x - r1)(x - r2)(x - r3), using plain f64 arithmetic (the same rounding any real
/// caller's own coefficient construction would incur).
fn monic_from_roots(r: [f64; 4]) -> (f64, f64, f64, f64, f64) {
    let a = -(r[0] + r[1] + r[2] + r[3]);
    let b = r[0] * r[1] + r[0] * r[2] + r[0] * r[3] + r[1] * r[2] + r[1] * r[3] + r[2] * r[3];
    let c = -(r[0] * r[1] * r[2] + r[0] * r[1] * r[3] + r[0] * r[2] * r[3] + r[1] * r[2] * r[3]);
    let d = r[0] * r[1] * r[2] * r[3];
    (d, c, b, a, 1.0)
}

fn eval(c0: f64, c1: f64, c2: f64, c3: f64, c4: f64, x: f64) -> f64 {
    (((c4 * x + c3) * x + c2) * x + c1) * x + c0
}

fn main() {
    // Two pairs of real roots, each pair separated by only ~2e-7 - an ordinary
    // "near-tangency"/"near-double-root" scenario, not a contrived extreme value. All 4 values
    // are finite, normal, well within typical curve-geometry magnitude ranges.
    let roots = [
        6.466862860353134,
        6.4668626776083,
        8.551950961608416,
        8.551950778863581,
    ];
    let (c0, c1, c2, c3, c4) = monic_from_roots(roots);

    println!("quartic coefficients: c0={c0:e} c1={c1:e} c2={c2:e} c3={c3:e} c4={c4:e}");
    println!("true roots (by construction): {roots:?}");

    // Neither of solve_quartic's two exact-equality degenerate fast paths (c4 == 0.0 / c0 == 0.0)
    // is hit here - this goes through the general Orellana-De Michele factoring path
    // (factor_quartic_inner / depressed_cubic_dominant) that candidate 037 targets.
    assert_ne!(c4, 0.0);
    assert_ne!(c0, 0.0);

    for run in 1..=5 {
        let found = solve_quartic(c0, c1, c2, c3, c4);
        println!(
            "run {run}: solve_quartic returned {} root(s): {found:?}",
            found.len()
        );
        for &x in &found {
            let resid = eval(c0, c1, c2, c3, c4, x);
            println!("    root {x} residual {resid:e}");
        }
    }

    let final_found = solve_quartic(c0, c1, c2, c3, c4);
    if final_found.len() < 2 {
        println!(
            "BUG REPRODUCED: solve_quartic returned {} root(s) for a quartic that has 4 genuine \
             real roots (two near-double-root pairs) - the real roots were silently dropped.",
            final_found.len()
        );
        std::process::exit(1);
    } else {
        println!(
            "solve_quartic returned {} root(s) - not reproduced this run",
            final_found.len()
        );
    }
}
