// Standalone reproduction for candidate 057: PathSeg::tangents()'s Quad-arm fallback
// (`kurbo/src/bezpath.rs`) can itself be exactly zero. `stroke_undashed` (kurbo/src/stroke.rs)
// only guards a QuadTo with the exact-equality check `p1 != p0 || p2 != p0` before calling
// `PathSeg::Quad(q).tangents()` and dividing by `tan0.hypot()` in `do_join`. A quad where `p1` is
// a few ULPs away from `p0` (so the guard passes and the preferred direction `p1 - p0` is below
// the internal `EPS` threshold) but `p2 == p0` exactly (so the fallback direction `p2 - p0` is
// also exactly zero) yields a zero tangent, and `0.5 * width / 0.0.hypot()` produces `inf`, which
// poisons the rest of the stroked path with `NaN`.
//
// This drives the real, public `kurbo::stroke()` entry point directly on a hand-built `BezPath` -
// the genuine external input a real caller would supply (an ordinary, spec-legal `BezPath`
// containing a numerically near-degenerate quadratic segment). There is no second participant to
// fake here: a `BezPath` is a single external artifact, exactly like a malformed file handed to a
// parser's own public `open_file`/`from_reader` entry point (see CONFIRMATION.md section 11).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{BezPath, Stroke};

fn has_nan(path: &BezPath) -> bool {
    format!("{path:?}").contains("NaN")
}

fn main() {
    let stroke = Stroke::new(2.0);
    let mut reproduced = false;

    // Case A (the exact repro from the candidate report): p1 a tiny nonzero offset from p0,
    // p2 == p0 exactly.
    let mut path_a = BezPath::new();
    path_a.move_to((0.0, 0.0));
    path_a.quad_to((1e-7, 0.0), (0.0, 0.0));
    path_a.line_to((10.0, 10.0));
    let stroked_a = kurbo::stroke(&path_a, &stroke, &Default::default(), 1e-3);
    println!("case A (p1 tiny offset, p2==p0): {stroked_a:?}");
    if has_nan(&stroked_a) {
        eprintln!("REPRODUCED (case A): stroked path contains NaN");
        reproduced = true;
    }

    // Case B: same bug class, different magnitude/axis/sign, to check the fix isn't narrowly
    // tailored to the exact triggering input (CONFIRMATION.md section 7).
    let mut path_b = BezPath::new();
    path_b.move_to((0.0, 0.0));
    path_b.quad_to((0.0, 1e-8), (0.0, 0.0));
    path_b.line_to((5.0, -5.0));
    let stroked_b = kurbo::stroke(&path_b, &stroke, &Default::default(), 1e-3);
    println!("case B (different axis/magnitude): {stroked_b:?}");
    if has_nan(&stroked_b) {
        eprintln!("REPRODUCED (case B): stroked path contains NaN");
        reproduced = true;
    }

    // Case C: negative offset, different downstream segment direction.
    let mut path_c = BezPath::new();
    path_c.move_to((0.0, 0.0));
    path_c.quad_to((-1e-7, 0.0), (0.0, 0.0));
    path_c.line_to((-10.0, 10.0));
    let stroked_c = kurbo::stroke(&path_c, &stroke, &Default::default(), 1e-3);
    println!("case C (negative offset): {stroked_c:?}");
    if has_nan(&stroked_c) {
        eprintln!("REPRODUCED (case C): stroked path contains NaN");
        reproduced = true;
    }

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - no NaN found in any case");
    }
}
