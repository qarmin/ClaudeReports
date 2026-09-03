//! Standalone reproduction for vello issue #1277: "Large cubic causes crash during flattening".
//!
//! This calls the real, public `vello_common::flatten::fill` entry point (the same function every
//! vello renderer backend calls to turn a filled `BezPath` into line segments) with a cubic Bezier
//! curve whose endpoints sit inside an ordinary viewport but whose control points have extreme
//! coordinate magnitude.
//!
//! Note on faithfulness to the original issue: the exact point values from issue #1277 place the
//! *entire* curve (all four points) above the top of any u16-sized cull bbox, which on current
//! main is always culled away by `flatten()`'s "fully outside the cull bbox" check before it ever
//! reaches the numerically fragile subdivision code - so running that literal repro against
//! current `main` no longer crashes (see `output.txt`, tests `literal_issue_1277`). The class of
//! bug the issue is about - an unbounded output-segment-count estimate for curves with
//! extreme/pathological control points - is still very much present, and is reached by any curve
//! where at least one endpoint is on-screen. That is what `main()` below demonstrates, and is a
//! realistic case (e.g. a document with a normal-looking curve whose control points overshoot by
//! many orders of magnitude, which is exactly the shape of curve issue #1277 was about).

use vello_common::fearless_simd::Level;
use vello_common::flatten::{FlattenCtx, Line, fill};
use vello_common::geometry::RectU16;
use vello_common::kurbo::{Affine, BezPath};

fn run(path: BezPath, label: &str) {
    let mut line_buf: Vec<Line> = Vec::new();
    let mut ctx = FlattenCtx::default();
    println!("--- {label} ---");
    fill(
        Level::baseline(),
        path,
        Affine::IDENTITY,
        &mut line_buf,
        &mut ctx,
        RectU16::new(0, 0, 65535, 65535),
    );
    println!("fill() returned successfully, produced {} lines", line_buf.len());
}

fn literal_issue_1277() -> BezPath {
    // Exactly the points from https://github.com/linebender/vello/issues/1277 (including its
    // `e1`, not `e18`, typo in the last two y-coordinates).
    let mut path = BezPath::new();
    path.move_to((-2.1351245891339424e19, -3.976870325297853e18));
    path.curve_to(
        (-2.1351245891339424e19, -3.976870325297879e18),
        (-2.1350093033635213e19, -3.9766555947095035e1),
        (-2.1348670911586615e19, -3.9763907111049165e1),
    );
    path.close_path();
    path
}

fn on_screen_extreme_control_points() -> BezPath {
    // Endpoints are inside the cull bbox (so the whole-curve "fully outside" cull check does not
    // fire), but the control points are extreme, same class of curve as issue #1277.
    let mut path = BezPath::new();
    path.move_to((10.0, 10.0));
    path.curve_to((1e19, 1e19), (-1e19, 1e19), (20.0, 20.0));
    path.close_path();
    path
}

fn main() {
    // Demonstrates that the exact issue #1277 repro no longer crashes on current main (it gets
    // culled entirely - see the comment above `main` for why).
    run(literal_issue_1277(), "literal issue #1277 repro (now culled away, no crash)");

    // Demonstrates the underlying bug: this still aborts the process with an allocation failure
    // on an unpatched tree, and completes normally once `patch.diff` is applied.
    run(
        on_screen_extreme_control_points(),
        "on-screen endpoints, extreme control points (the actual crash)",
    );

    println!("Done: no crash.");
}
