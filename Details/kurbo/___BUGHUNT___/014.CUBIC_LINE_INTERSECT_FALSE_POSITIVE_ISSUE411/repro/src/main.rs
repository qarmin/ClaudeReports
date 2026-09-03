// Standalone reproduction for kurbo issue #411: `PathSeg::intersect_line` reports a false-
// positive intersection between a cubic and a line segment that clearly don't cross, when
// negative coordinate values are present.
//
// Uses only kurbo's public API (`CubicBez`, `Line`, `PathSeg::intersect_line`) - the exact
// entry point and coordinates from the upstream issue report
// (https://github.com/linebender/kurbo/issues/411).
//
// Exit code convention: 0 = did not reproduce (no false-positive hit - the goal after a fix),
// non-zero = reproduced (a spurious intersection was reported).

use kurbo::{CubicBez, Line, PathSeg};

fn main() {
    let c: PathSeg = CubicBez::new(
        (452.0, 240.0),
        (462.667, 78.667),
        (480.667, -146.333),
        (506.0, -435.0),
    )
    .into();
    let line = Line::new((385.0, 146.0), (438.0, 243.0));

    let hits = c.intersect_line(line);
    println!("hits = {:?}", hits);

    if hits.is_empty() {
        println!("did not reproduce - no intersection reported, as expected");
    } else {
        eprintln!(
            "REPRODUCED - spurious intersection(s) reported for a cubic and line that do not cross: {:?}",
            hits
        );
        std::process::exit(1);
    }
}
