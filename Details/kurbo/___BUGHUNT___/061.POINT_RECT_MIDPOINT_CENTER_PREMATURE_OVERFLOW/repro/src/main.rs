// Standalone reproduction for candidate 061: Point::midpoint (kurbo/src/point.rs) and
// Rect::center (kurbo/src/rect.rs) both compute `0.5 * (a + b)`, i.e. they sum the two operands
// before scaling. Two same-sign operands that are each individually well within finite f64 range
// (e.g. 1.5e308 and 1.7e308, both far below f64::MAX ~= 1.7977e308) can sum past f64::MAX before
// the 0.5 scaling ever gets applied, overflowing the intermediate sum to `inf` even though the
// true mathematical midpoint (1.6e308) is trivially representable. This is the same
// "square-then-sqrt"-shaped premature-overflow class already confirmed in candidates 056/059, but
// here the intermediate operation is `+` instead of `*`.
//
// This drives the real, public `Point::midpoint()` and `Rect::center()` entry points directly -
// both pure single-participant numeric computations on caller-supplied coordinates (there is no
// second participant/peer to fake here: the large-magnitude coordinates themselves *are* the
// genuine external input a real caller would supply, e.g. any application building points/rects
// from measured or computed coordinates that happen to be very large).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{Point, Rect};

fn main() {
    let mut reproduced = false;

    // 1. Point::midpoint() on two large-but-representable, same-sign points.
    let p1 = Point::new(1.5e308, 0.0);
    let p2 = Point::new(1.7e308, 0.0);
    let mid = p1.midpoint(p2);
    println!("Point::new(1.5e308, 0.0).midpoint(Point::new(1.7e308, 0.0)) = {mid:?}");
    if !mid.x.is_finite() {
        eprintln!("REPRODUCED: Point::midpoint returned {mid:?} instead of (1.6e308, 0.0)");
        reproduced = true;
    } else if (mid.x - 1.6e308).abs() > 1e293 {
        eprintln!("WRONG OUTPUT: Point::midpoint returned {mid:?}, expected ~(1.6e308, 0.0)");
        reproduced = true;
    }

    // 2. Rect::center() on a rect built from the same large, same-sign coordinates.
    let r = Rect::new(1.5e308, 0.0, 1.7e308, 0.0);
    let center = r.center();
    println!("Rect::new(1.5e308, 0.0, 1.7e308, 0.0).center() = {center:?}");
    if !center.x.is_finite() {
        eprintln!("REPRODUCED: Rect::center returned {center:?} instead of (1.6e308, 0.0)");
        reproduced = true;
    }

    // 3. Boundary sweep: show the premature-overflow point sits below f64::MAX for same-sign
    // operands (not simply "the value is genuinely too large to represent").
    for v in [1e150, 1e153, 8.9e307, 9e307, 1e308, 1.5e308, 1.7e308, f64::MAX] {
        let a = Point::new(v, 0.0);
        let b = Point::new(v, 0.0);
        let m = a.midpoint(b);
        println!("midpoint({v:e}, {v:e}) = {m:?} (finite: {})", m.x.is_finite());
    }
    println!("f64::MAX = {}", f64::MAX);

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - Point::midpoint()/Rect::center() stayed correct and finite");
    }
}
