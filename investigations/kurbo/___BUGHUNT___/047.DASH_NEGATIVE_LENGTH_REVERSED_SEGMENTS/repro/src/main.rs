// Standalone reproduction for candidate 047: DASH_NEGATIVE_LENGTH_REVERSED_SEGMENTS.
//
// kurbo::dash() (kurbo/src/stroke.rs) does not validate that dash-pattern entries are
// non-negative. `dash_iter`'s one-time setup loop (lines ~747-751) corrects a negative
// FIRST entry when searching for the initial offset position, but `DashIterator::step`'s
// main loop (lines ~834-873) has no equivalent correction: `self.dash_remaining =
// self.dashes[self.dash_ix]` (line ~863) re-reads a raw, unclamped dash entry every time the
// index wraps back to a negative slot. A negative `dash_remaining` then satisfies
// `self.dash_remaining < self.seg_remaining` on (almost) every subsequent step, and
// `seg.inv_arclen(self.dash_remaining, ...)` is asked for the parametrization at a
// *negative* arc-length target, producing a `t1` that does not correspond to the intended
// forward sub-segment - so the emitted dash segments run backwards.
//
// This uses kurbo's real public API (`kurbo::dash`) on a simple straight line, driven with
// a single negative entry in an otherwise ordinary dash pattern - exactly the input a real
// caller (e.g. an SVG `stroke-dasharray` consumer, which permits negative-looking numbers in
// its own parsing unless the caller separately validates them) could hand to `dash()`.
//
// Exit code convention: 0 = did not reproduce (every LineTo point comes after its
// preceding MoveTo point along +x, i.e. forward), non-zero = reproduced (a MoveTo/LineTo
// pair runs backwards).

use kurbo::{dash, BezPath, Line, PathEl, Point};

fn main() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    let mut path = BezPath::new();
    path.move_to(line.p0);
    path.line_to(line.p1);

    let dashes = [-5.0, 10.0];
    let result: Vec<PathEl> = dash(path.iter(), 0.0, &dashes).collect();

    println!("dash pattern: {:?}", dashes);
    println!("num path elements emitted: {}", result.len());
    for el in &result {
        println!("{el:?}");
    }

    // Walk MoveTo/LineTo pairs and check that each LineTo's x coordinate is >= the
    // preceding MoveTo's x coordinate (this line only ever moves in +x, so any dash
    // segment running backwards is a direct bug signature).
    let mut reproduced = false;
    let mut pending_move: Option<Point> = None;
    for el in &result {
        match *el {
            PathEl::MoveTo(p) => pending_move = Some(p),
            PathEl::LineTo(p) => {
                if let Some(m) = pending_move.take() {
                    if p.x < m.x {
                        println!(
                            "REPRODUCED: backwards segment MoveTo({:?}) -> LineTo({:?}) (x decreases)",
                            m, p
                        );
                        reproduced = true;
                    }
                }
            }
            _ => {}
        }
    }

    if reproduced {
        eprintln!("REPRODUCED - dash() emitted at least one backwards sub-segment for a negative dash entry");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all emitted sub-segments run forward (+x)");
    }
}
