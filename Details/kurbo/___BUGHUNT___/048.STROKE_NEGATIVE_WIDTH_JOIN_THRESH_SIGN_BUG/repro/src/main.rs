// Repro for candidate 048: `stroke_undashed` computes
// `ctx.join_thresh = 2.0 * tolerance / style.width` without taking the absolute value of
// `style.width`. `Stroke::new` accepts any `f64` width, including negative values, with no
// validation anywhere on the path from the public `stroke()`/`stroke_with()` entry points down to
// this line.
//
// For a negative width, `join_thresh` becomes negative. The join-omission test in `do_join` is:
//     if dot <= 0.0 || cross.abs() >= hypot * self.join_thresh { <add a join> }
// With `join_thresh` negative, `hypot * self.join_thresh` is <= 0.0 for any non-negative `hypot`,
// so `cross.abs() >= hypot * self.join_thresh` is satisfied for essentially every join, no matter
// how small the bend angle is. This forces a real join (bevel/miter/round geometry) to be
// inserted at every joint of a negative-width stroke, even at joints so close to straight that a
// positive-width stroke of the same magnitude correctly omits the join as imperceptible.
//
// This is a real, structural, sign-dependent asymmetry in the *number and shape* of path elements
// produced for a stroke, not merely a cosmetic float rounding difference: the join-omission
// decision is conceptually a property of the path's own bend angle at a vertex, and must not
// depend on the sign of `style.width`. This repro demonstrates that asymmetry through the crate's
// own public `stroke()` API on a plain, unremarkable near-straight two-segment path - no
// second participant is being faked here: a `Stroke` with a negative width is a legitimate value
// any caller can construct (`Stroke` has no validation on construction), and `stroke()` is the
// real, single, affected public entry point.
use kurbo::{BezPath, Join, Point, Stroke, StrokeOpts, stroke};

fn make_path(bend: f64) -> BezPath {
    // A two-segment path that bends by a tiny amount at (50, 0) -- far too small an angle for
    // any join to be visually necessary at ordinary strokes.
    let mut p = BezPath::new();
    p.move_to(Point::new(0.0, 0.0));
    p.line_to(Point::new(50.0, 0.0));
    p.line_to(Point::new(120.0, bend));
    p
}

fn element_count(width: f64, tolerance: f64, join: Join, bend: f64) -> usize {
    let path = make_path(bend);
    let style = Stroke::new(width).with_join(join);
    let opts = StrokeOpts::default();
    stroke(path, &style, &opts, tolerance).elements().len()
}

fn main() {
    let mut any_mismatch = false;

    // Sweep several tiny bend angles, join styles, and width magnitudes. In every case, the
    // stroke's own join-omission decision at the one bent vertex must be identical whether the
    // stroke's width is +w or -w -- the join-omission test is defined purely in terms of the
    // angle between the two adjacent tangents (`cross`/`dot`/`hypot`), which does not depend on
    // `style.width` at all except through the buggy `join_thresh` term.
    for bend in [0.0001_f64, 0.0005, 0.002, 0.01] {
        for join in [Join::Miter, Join::Bevel, Join::Round] {
            for w in [0.3_f64, 2.5] {
                let pos = element_count(w, 0.1, join, bend);
                let neg = element_count(-w, 0.1, join, bend);
                let matches = pos == neg;
                println!(
                    "bend={bend:<8} join={join:?} width=+-{w}: elements(+{w})={pos} elements(-{w})={neg} match={matches}"
                );
                if !matches {
                    any_mismatch = true;
                }
            }
        }
    }

    if any_mismatch {
        println!(
            "\nBUG REPRODUCED: stroke() emits a different number of path elements for a \
             negative-width stroke than for the equal-magnitude positive-width stroke on the \
             same path, because `join_thresh` is computed from the signed `style.width` instead \
             of its absolute value, forcing spurious extra joins onto negative-width strokes."
        );
        std::process::exit(1);
    } else {
        println!("\nNo mismatch: element counts are symmetric under width sign flip (bug fixed).");
    }
}
