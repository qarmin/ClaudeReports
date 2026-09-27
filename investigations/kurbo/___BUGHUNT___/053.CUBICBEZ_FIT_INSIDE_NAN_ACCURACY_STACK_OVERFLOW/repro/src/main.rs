// Standalone reproduction: `CubicBez::approx_spline` with a NaN `accuracy` recurses without
// bound through the private `fit_inside` helper, because every comparison against NaN is false,
// so neither of `fit_inside`'s two base cases ever fires. This is a genuine stack overflow (the
// process aborts), not a panic, so it can't be caught with `catch_unwind` - the exit
// code/abort itself is the signal.
//
// Exit code convention: 0 = did not reproduce (bug absent - the goal after a fix). Before the
// fix, this process never reaches the `println!` at all - it aborts with "stack overflow" printed
// by the Rust runtime itself.

use kurbo::CubicBez;

fn main() {
    let c = CubicBez::new((0.0, 0.0), (0.0, 100.0), (100.0, 100.0), (100.0, 0.0));
    let result = c.approx_spline(f64::NAN);
    println!("did not reproduce - approx_spline(NaN) returned: {:?}", result);
}
