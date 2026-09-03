// Standalone repro for candidate 052: Rect::inscribed_rect_with_aspect_ratio produces an
// inverted-coordinate Rect (x0 > x1, or y0 > y1) for a negative `aspect_ratio`, via kurbo's own
// real public API - no hand-constructed internal state, no faked second participant (this is a
// single-process, single-participant finding: one public function called with one f64 argument).

use kurbo::Rect;

fn check(outer: Rect, aspect_ratio: f64) {
    let inner = outer.inscribed_rect_with_aspect_ratio(aspect_ratio);
    println!("outer = {outer:?}, aspect_ratio = {aspect_ratio}");
    println!(
        "inner = {inner:?}  width={} height={}",
        inner.width(),
        inner.height()
    );
    assert!(
        inner.x0 <= inner.x1,
        "BUG: inner rect has x0 > x1 (inverted width): {inner:?}"
    );
    assert!(
        inner.y0 <= inner.y1,
        "BUG: inner rect has y0 > y1 (inverted height): {inner:?}"
    );
}

fn main() {
    // Original triggering input: square outer, aspect_ratio = -1 (takes the "shrink x" branch).
    check(Rect::new(0.0, 0.0, 10.0, 10.0), -1.0);
    // A second input in the same class: non-square outer, different negative ratio, still the
    // "shrink x" branch, different gap magnitude.
    check(Rect::new(0.0, 0.0, 20.0, 10.0), -0.5);
    // A third input exercising the other branch ("shrink y", self_aspect.abs() < aspect_ratio.abs()).
    check(Rect::new(0.0, 0.0, 10.0, 20.0), -4.0);
    // Sanity: positive aspect ratios still produce the documented example from the doctest.
    check(Rect::new(0.0, 0.0, 10.0, 20.0), 1.0);
    println!("All checks passed.");
}
