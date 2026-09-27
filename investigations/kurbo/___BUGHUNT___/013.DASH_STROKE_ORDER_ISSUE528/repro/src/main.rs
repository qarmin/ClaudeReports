// Reproduction for kurbo GitHub issue #528: "stroke() function with dashed stroke returns
// first piece last".
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{BezPath, PathEl, Stroke, StrokeOpts, dash, stroke};

/// Extract the x-coordinate right after each `MoveTo` in a flat element stream - i.e. the start
/// point of each dash "on" piece.
fn dash_piece_starts(elements: &[PathEl]) -> Vec<f64> {
    elements
        .iter()
        .filter_map(|el| match el {
            PathEl::MoveTo(p) => Some(p.x),
            _ => None,
        })
        .collect()
}

fn is_sorted(xs: &[f64]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    // Two identical open subpaths (a 21-unit horizontal line each), offset from each other -
    // matches the issue's own report that "it affects every individual subpath" in a multi-
    // subpath input.
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((21.0, 0.0));
    path.move_to((100.0, 0.0));
    path.line_to((121.0, 0.0));

    let dashes = [1.0, 5.0, 2.0, 5.0];

    // --- 1. Public `dash()` iterator directly, the same public API the issue reporter used
    //        when splitting kurbo's own SVG output into individual `<path/>` pieces. ---
    let dashed: Vec<PathEl> = dash(path.clone().into_iter(), 0.0, &dashes).collect();
    println!("dash() output (default, non-stable order):\n  {dashed:?}\n");

    let starts = dash_piece_starts(&dashed);
    println!("MoveTo x-starts, in emitted order: {starts:?}");

    // For each subpath, the position-ordered (correct) sequence of dash-piece starts along a
    // 21-unit line with dashes=[1,5,2,5] and offset 0 is x=0 (first), then x=6, x=13, x=19 - in
    // that increasing order. A caller walking the emitted pieces in order (e.g. to paint a
    // gradient along the path, per the issue) expects each subpath's own starts to be
    // non-decreasing.
    let mut reproduced = false;
    for (i, subpath_starts) in starts.chunks(4).enumerate() {
        let ordered = is_sorted(subpath_starts);
        println!("  subpath {i}: emitted starts = {subpath_starts:?} -- in path order? {ordered}");
        if !ordered {
            reproduced = true;
            let subpath_origin_x = if i == 0 { 0.0 } else { 100.0 };
            let first_emitted = subpath_starts[0];
            let last_emitted = *subpath_starts.last().unwrap();
            println!(
                "    -> subpath {i}: first EMITTED piece starts at x={first_emitted}, last EMITTED piece starts at x={last_emitted} (subpath's true first piece starts at x={subpath_origin_x})"
            );
            assert!(
                (last_emitted - subpath_origin_x).abs() < 1e-9,
                "expected the subpath's true first dash piece (x={subpath_origin_x}) to be the one emitted LAST"
            );
        }
    }

    // --- 2. Full public `stroke()` entry point (the exact function named in the issue title),
    //        using default StrokeOpts - the fill outline's MoveTo order follows the same
    //        (broken) per-subpath ordering as `dash()` above, since `stroke_with` feeds
    //        `dash_iter`'s output straight into `stroke_undashed`. ---
    let style = Stroke::new(1.0).with_dashes(0.0, dashes);
    let stroked = stroke(path.clone(), &style, &StrokeOpts::default(), 0.01);
    let stroked_starts = dash_piece_starts(stroked.elements());
    println!("\nstroke() (default StrokeOpts) fill-outline MoveTo x-starts: {stroked_starts:?}");
    let stroked_subpath0_ordered = is_sorted(&stroked_starts[..stroked_starts.len() / 2]);
    println!("  subpath 0 (of the fill outline) in path order? {stroked_subpath0_ordered}");

    // --- 3. Escape hatch already present in this checkout (added by PR #550, merged into
    //        `main` before this candidate was even opened):
    //        `StrokeOpts::default().stable_dash_order(true)` restores in-path-order emission. ---
    let stable_opts = StrokeOpts::default().stable_dash_order(true);
    let stroked_stable = stroke(path.clone(), &style, &stable_opts, 0.01);
    let stable_starts = dash_piece_starts(stroked_stable.elements());
    println!("\nstroke() with stable_dash_order(true) fill-outline MoveTo x-starts: {stable_starts:?}");
    let stable_overall_ordered = is_sorted(&stable_starts);
    println!(
        "  overall non-decreasing across both subpaths (evidence the opt-in escape hatch already fixes it)? {stable_overall_ordered}"
    );

    println!(
        "\n{}",
        if reproduced {
            "REPRODUCED: default dash order does not follow path order (first piece emitted last)"
        } else {
            "did not reproduce - dash order followed path order"
        }
    );

    if reproduced {
        std::process::exit(1);
    }
}
