// Standalone reproduction for candidate 054: `kurbo::common::solve_quadratic`'s
// overflow-recovery branch (the `!arg.is_finite()` case) silently returns a bogus
// single root instead of the two real roots, when `sc1` (== c1/c2) is zero or small
// while `sc0` (== c0/c2) is large enough in magnitude that `4. * sc0` overflows.
//
// Uses kurbo's public API directly (`kurbo::common::solve_quadratic` is `pub fn`) -
// this is a pure math function with no second participant/peer involved, so calling
// it directly through its documented public entry point is the real trigger path
// (CONFIRMATION.md section 11 concerns a different situation - one participant
// reacting to another's wire input - which does not apply to a self-contained
// numeric routine like this one).
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use kurbo::common::solve_quadratic;

fn main() {
    // c0 + c1*x + c2*x^2 = 0, with c1 == 0.0 and c0 == -1e308, c2 == 1.0.
    // Mathematically: x^2 = 1e308, so the two real roots are +-1e154.
    let c0: f64 = -1e308;
    let c1: f64 = 0.0;
    let c2: f64 = 1.0;

    let roots = solve_quadratic(c0, c1, c2);
    println!("solve_quadratic({c0:e}, {c1:e}, {c2:e}) = {roots:?}");

    let expected_magnitude = (-c0 / c2).sqrt();
    println!("expected two real roots near +-{expected_magnitude:e}");

    let mut bogus = false;
    if roots.len() != 2 {
        println!(
            "WRONG ROOT COUNT: got {} root(s), expected 2",
            roots.len()
        );
        bogus = true;
    }
    for r in &roots {
        let residual = c0 + c1 * r + c2 * r * r;
        // The residual should be tiny relative to c0's own magnitude (~1e308) if `r`
        // is really close to a root. If it's still off by orders of magnitude
        // relative to c0, this root is not a real solution at all.
        let rel_residual = (residual / c0).abs();
        println!("  root {r:e}: residual {residual:e} (relative to c0: {rel_residual:e})");
        if rel_residual > 1e-6 {
            println!("    -> NOT a valid root of the original equation");
            bogus = true;
        }
    }

    if bogus {
        eprintln!("REPRODUCED - solve_quadratic silently dropped/mangled a real root");
        std::process::exit(1);
    } else {
        println!("did not reproduce - both real roots recovered correctly");
    }
}
