// Standalone reproduction for candidate 050:
// Quadratic::positive_discriminant_roots returns a non-finite bound for the
// degenerate a == 0 (linear-equation) case, unlike its sibling Quadratic::roots
// which filters non-finite values out.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use polycool::Quadratic;

fn main() {
    // 2x + 1 = 0  (a == 0, so this is really a linear equation with one root, x = -0.5)
    let q = Quadratic::new([1.0, 2.0, 0.0]);

    let roots = q.roots();
    println!("Quadratic::roots() = {roots:?}");

    let pdr = q.positive_discriminant_roots();
    println!("Quadratic::positive_discriminant_roots() = {pdr:?}");

    match pdr {
        Some((lo, hi)) if !lo.is_finite() || !hi.is_finite() => {
            eprintln!(
                "REPRODUCED - positive_discriminant_roots() returned a non-finite bound: ({lo}, {hi})"
            );
            std::process::exit(1);
        }
        Some((lo, hi)) => {
            println!("did not reproduce - both bounds finite: ({lo}, {hi})");
        }
        None => {
            println!("did not reproduce - positive_discriminant_roots() returned None");
        }
    }
}
