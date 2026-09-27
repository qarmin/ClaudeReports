// Real end-to-end entry point: the fully public `ParamCurveNearest::nearest(p, accuracy)` on
// `CubicBez`, which passes `accuracy` straight through as `x_error` into
// `polycool::Poly::<N>::roots_between` and eventually into `find_root` (see candidate write-up).
// This exercises the actual caller-reachable path, not a hand-built stand-in for it.
use kurbo::{CubicBez, ParamCurveNearest, Point};
use std::time::{Duration, Instant};

fn check(c: CubicBez, p: Point, accuracy: f64) {
    let start = Instant::now();
    let n = c.nearest(p, accuracy);
    let elapsed = start.elapsed();
    println!("accuracy={accuracy} nearest={n:?} elapsed={elapsed:?}");
    assert!(
        elapsed < Duration::from_secs(3),
        "hang detected for accuracy={accuracy} (took {elapsed:?})"
    );
}

fn main() {
    let curves = [
        CubicBez::new(
            Point::new(0.0, 0.0),
            Point::new(1.0, 5.0),
            Point::new(3.0, -4.0),
            Point::new(4.0, 1.0),
        ),
        CubicBez::new(
            Point::new(-100.0, 50.0),
            Point::new(200.0, -300.0),
            Point::new(-50.0, 400.0),
            Point::new(300.0, 300.0),
        ),
    ];
    let accuracies = [-1.0, 0.0, -1e300, f64::NEG_INFINITY, -f64::MIN_POSITIVE];

    for c in curves {
        for &accuracy in &accuracies {
            check(c, Point::new(2.0, 2.0), accuracy);
        }
    }
    println!("all quick - no hang observed for any non-positive accuracy value tried");
}
