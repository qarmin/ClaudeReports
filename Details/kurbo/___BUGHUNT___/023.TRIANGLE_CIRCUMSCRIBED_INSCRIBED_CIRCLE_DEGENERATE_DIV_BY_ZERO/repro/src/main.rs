// Standalone reproduction: Triangle::inscribed_circle / circumscribed_circle silently divide
// by zero (perimeter / signed-area-derived cross product) for a degenerate (collinear or
// coincident-vertex) Triangle, producing NaN/Inf Circle fields with no panic and no
// documentation of the case, unlike e.g. Affine::inverse which documents this explicitly.
//
// Uses kurbo's public API directly (Triangle::new, Shape trait not needed).
//
// Exit code convention: 0 = did not reproduce (all fields finite), non-zero = reproduced
// (a NaN or infinite field was found in inscribed_circle()/circumscribed_circle()'s output).

use kurbo::Triangle;

fn has_non_finite(c: kurbo::Circle) -> bool {
    !c.center.x.is_finite() || !c.center.y.is_finite() || !c.radius.is_finite()
}

fn check(name: &str, t: Triangle) -> bool {
    let inc = t.inscribed_circle();
    let circ = t.circumscribed_circle();
    let bad_inc = has_non_finite(inc);
    let bad_circ = has_non_finite(circ);
    println!(
        "{name}: area={} inscribed={:?} (non-finite={}) circumscribed={:?} (non-finite={})",
        t.area(),
        inc,
        bad_inc,
        circ,
        bad_circ
    );
    bad_inc || bad_circ
}

fn main() {
    let mut reproduced = false;

    // Three distinct but collinear points.
    reproduced |= check(
        "collinear",
        Triangle::new((0.0, 0.0), (1.0, 0.0), (2.0, 0.0)),
    );

    // Three coincident points.
    reproduced |= check(
        "coincident",
        Triangle::new((5.0, 5.0), (5.0, 5.0), (5.0, 5.0)),
    );

    // Two coincident vertices, one distinct.
    reproduced |= check(
        "two-coincident",
        Triangle::new((0.0, 0.0), (0.0, 0.0), (3.0, 4.0)),
    );

    if reproduced {
        eprintln!(
            "REPRODUCED - a degenerate Triangle produced a Circle with a NaN/infinite field"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - all outputs finite");
    }
}
