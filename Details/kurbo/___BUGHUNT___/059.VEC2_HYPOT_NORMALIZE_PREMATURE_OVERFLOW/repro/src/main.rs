// Standalone reproduction for candidate 059: Vec2::hypot (kurbo/src/vec2.rs) computes
// `(x*x + y*y).sqrt()` instead of an overflow-safe hypotenuse. A large-but-perfectly-representable
// component (e.g. x = y = 1e200, well under f64::MAX ~= 1.7977e308) makes the intermediate squaring
// step (`1e200 * 1e200` = `1e400`) overflow to `inf`, even though the true mathematical result
// (~1.41421e200) is trivially representable. `Point::distance` delegates to the same `hypot()`, and
// `Vec2::normalize()` (`self / self.hypot()`) then divides by that `inf`, silently returning the
// zero vector instead of a correct unit vector - a wrong-geometric-result footgun, not just an inf.
//
// This drives the real, public `Vec2::hypot()`, `Vec2::length()`, `Vec2::normalize()`, and
// `Point::distance()` entry points directly - all pure single-participant numeric computations on
// caller-supplied coordinates (there is no second participant/peer to fake here: the large-magnitude
// coordinates themselves *are* the genuine external input a real caller would supply, e.g. any
// application building vectors/points from measured or computed coordinates that happen to be
// very large).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{Point, Vec2};

fn main() {
    let mut reproduced = false;

    // 1. Vec2::hypot() on a large vector: the true magnitude is exactly 1.414213562373095e200
    // (perfectly representable), but the buggy code squares 1e200 to 1e400 first, which overflows
    // to `inf`.
    let v = Vec2::new(1e200, 1e200);
    let h = v.hypot();
    println!("Vec2::new(1e200, 1e200).hypot() = {h}");
    let expected = 1e200_f64.hypot(1e200);
    if !h.is_finite() {
        eprintln!("REPRODUCED: hypot() returned {h} instead of a finite value (~{expected})");
        reproduced = true;
    } else if (h - expected).abs() > expected * 1e-12 {
        eprintln!("WRONG OUTPUT: hypot() returned {h}, expected ~{expected}");
        reproduced = true;
    }

    // 2. Vec2::length() is an alias for hypot() - same bug.
    let len = v.length();
    println!("Vec2::new(1e200, 1e200).length() = {len}");
    if !len.is_finite() {
        eprintln!("REPRODUCED: length() returned {len} instead of a finite value");
        reproduced = true;
    }

    // 3. Point::distance() delegates to the same hypot() - inherits the bug.
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(1e200, 1e200);
    let d = p1.distance(p2);
    println!("Point::distance(origin, (1e200,1e200)) = {d}");
    if !d.is_finite() {
        eprintln!("REPRODUCED: distance() returned {d} instead of a finite value (~{expected})");
        reproduced = true;
    }

    // 4. Vec2::normalize() divides by the (buggy, inf) hypot(), silently returning the zero
    // vector instead of a correct unit vector - a wrong-geometric-result footgun, not just an inf.
    let n = v.normalize();
    println!("Vec2::new(1e200, 1e200).normalize() = {n:?}");
    let expected_component = std::f64::consts::FRAC_1_SQRT_2;
    if (n.x - expected_component).abs() > 1e-9 || (n.y - expected_component).abs() > 1e-9 {
        eprintln!(
            "REPRODUCED: normalize() returned {n:?} instead of a unit vector (~({expected_component}, {expected_component}))"
        );
        reproduced = true;
    }

    // 5. Boundary sweep: show the premature-overflow point sits far below f64::MAX, i.e. this is
    // not simply "the value is genuinely too large to represent".
    for exp in [100_i32, 150, 153, 154, 155, 200, 300] {
        let x: f64 = format!("1e{exp}").parse().unwrap();
        let hv = Vec2::new(x, x).hypot();
        println!("Vec2::new(1e{exp}, 1e{exp}).hypot() = {hv} (finite: {})", hv.is_finite());
    }
    println!("f64::MAX = {}", f64::MAX);

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - hypot()/length()/distance()/normalize() all stayed correct and finite");
    }
}
