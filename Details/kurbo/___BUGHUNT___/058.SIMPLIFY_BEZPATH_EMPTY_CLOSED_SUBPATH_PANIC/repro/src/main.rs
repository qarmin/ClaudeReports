// Standalone reproduction for candidate 058: `simplify_bezpath` panics (debug) / emits a
// malformed `BezPath` not starting with `MoveTo` (release) for a well-formed zero-segment
// closed subpath ("M0,0 Z" - a MoveTo immediately followed by ClosePath with no segments in
// between).
//
// Uses only kurbo's own public API (`kurbo::simplify::simplify_bezpath`, `BezPath`).
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (release-mode malformed
// output) or the process aborts with a panic (debug-mode).

use kurbo::BezPath;
use kurbo::simplify::simplify_bezpath;

fn main() {
    // Case 1: the whole path is just one degenerate empty closed subpath.
    let mut p = BezPath::new();
    p.move_to((0.0, 0.0));
    p.close_path();
    println!("input 1:  {p:?}");

    let result = std::panic::catch_unwind(|| simplify_bezpath(p.clone(), 0.1, &Default::default()));
    match result {
        Err(payload) => {
            eprintln!("REPRODUCED (panic): {payload:?}");
            std::process::exit(1);
        }
        Ok(out) => {
            println!("output 1: {out:?}");
            let starts_with_moveto =
                matches!(out.elements().first(), Some(kurbo::PathEl::MoveTo(_)) | None);
            if !starts_with_moveto {
                eprintln!(
                    "REPRODUCED (malformed): non-empty BezPath does not start with MoveTo: {out:?}"
                );
                std::process::exit(1);
            }
        }
    }

    // Case 2 (related, same root cause): a degenerate empty closed subpath *in the middle* of
    // an otherwise well-formed multi-subpath path. This never hits the debug_assert (the global
    // element vec is non-empty by then), but without the fix it silently emits a second,
    // moveto-less ClosePath into the result - also a malformed BezPath.
    let mut p2 = BezPath::new();
    p2.move_to((0.0, 0.0));
    p2.line_to((1.0, 1.0));
    p2.close_path();
    p2.move_to((2.0, 2.0));
    p2.close_path();
    p2.move_to((3.0, 3.0));
    p2.line_to((4.0, 4.0));
    p2.close_path();
    println!("input 2:  {p2:?}");

    let out2 = simplify_bezpath(p2, 0.1, &Default::default());
    println!("output 2: {out2:?}");

    // A well-formed BezPath never has two ClosePath/subpath-ending elements in a row without an
    // intervening MoveTo, and never has a ClosePath as the very first element of a subpath.
    let mut prev_was_subpath_end = false;
    for el in out2.elements() {
        match el {
            kurbo::PathEl::MoveTo(_) => prev_was_subpath_end = false,
            kurbo::PathEl::ClosePath => {
                if prev_was_subpath_end {
                    eprintln!("REPRODUCED (malformed): ClosePath with no preceding MoveTo in {out2:?}");
                    std::process::exit(1);
                }
                prev_was_subpath_end = true;
            }
            _ => {}
        }
    }

    println!("did not reproduce - both cases produced well-formed output");
}
