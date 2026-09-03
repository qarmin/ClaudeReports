// Standalone reproduction for issue #519: kurbo::stroke() can produce a path containing NaN
// coordinates for a stroke width large relative to the path's own size.
//
// Uses only kurbo's public API (BezPath::from_svg, kurbo::stroke, BezPath::is_nan).
//
// Exit code convention: 0 = did not reproduce (no NaN in the output - the goal after a fix),
// non-zero = reproduced (stroked path contains NaN).

use kurbo::{BezPath, Stroke, StrokeOpts};

fn main() {
    // Exact path from https://github.com/linebender/kurbo/issues/519 - a tiny, near-degenerate
    // triangular subpath (each cubic segment has a repeated control point at one end).
    let path = BezPath::from_svg(
        "M47.5884,316.827 C47.5884,316.827 47.4321,317.6239 47.4321,317.6239 \
         C46.7915,317.7801 46.7915,317.2176 46.7915,317.2176 Z",
    )
    .expect("valid SVG path data");

    let mut any_nan = false;
    for width in [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 15.0, 20.0, 50.0, 100.0, 1000.0] {
        let stroke = Stroke {
            width,
            ..Stroke::default()
        };
        let expanded = kurbo::stroke(&path, &stroke, &StrokeOpts::default(), 0.25);
        let is_nan = expanded.is_nan();
        println!("width={width} is_nan={is_nan}");
        if is_nan {
            any_nan = true;
        }
    }

    if any_nan {
        eprintln!("REPRODUCED - kurbo::stroke() produced a path containing NaN coordinates");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all widths produced a finite (non-NaN) stroked path");
    }
}
