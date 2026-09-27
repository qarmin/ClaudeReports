// Standalone reproduction for candidate 036: PolyDyn::roots_between recurses once per degree
// of derivative taken, with no depth cap - only bottoming out once degree() <= 3. A polynomial
// with a large-enough number of coefficients drives that recursion deep enough to overflow the
// native call stack.
//
// This is a genuine stack overflow (process abort via SIGABRT, "has overflowed its stack"), not
// a catchable panic, so there is nothing for `std::panic::catch_unwind` to catch here - the
// process aborting on its own *is* the reproduction. Exit code convention: 0 = did not reproduce
// (bug absent), any other outcome (abort, non-zero exit) = reproduced.

use polycool::PolyDyn;

fn main() {
    let degree: usize = std::env::args()
        .nth(1)
        .map(|s| s.parse().expect("degree must be a usize"))
        .unwrap_or(50_000);

    // `degree` coefficients of 0.0 plus one more gives a polynomial whose `degree()` (see
    // PolyDyn::degree) is exactly `degree`, entirely through the public API - no internal
    // scaffolding needed.
    let coeffs = vec![0.0f64; degree + 1];
    let p = PolyDyn::new(coeffs);
    println!("constructed PolyDyn with degree() = {}", p.degree());

    let roots = p.roots_between(-1.0, 1.0, 1e-6);

    // If we get here, the real end-to-end call into the public API returned normally instead of
    // overflowing the stack - the bug did not reproduce for this degree.
    println!(
        "did not reproduce - roots_between returned normally with {} roots",
        roots.len()
    );
}
