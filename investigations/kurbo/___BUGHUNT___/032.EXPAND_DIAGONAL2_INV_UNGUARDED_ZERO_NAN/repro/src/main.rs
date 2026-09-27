// Reproduces EXPAND_DIAGONAL2_INV_UNGUARDED_ZERO_NAN: `expand_path`/`expand_path_signed`'s
// public API accepts a `Diagonal2` expansion factor with a zero component. `ExpandCtx::do_quad`
// (and `do_cubic`) call `self.expand.inv()` unconditionally, and `Diagonal2::inv` divides
// `1.0 / xx` / `1.0 / yy` with no guard, so a zero `xx` produces `f64::INFINITY`, which then
// poisons every downstream `Vec2`/`Point` computation into NaN. This uses only kurbo's public
// API (`expand_path`, `Diagonal2`, `Join`, `BezPath`) end-to-end - no internal fields are poked.

use kurbo::{BezPath, Diagonal2, Join, PathEl, expand_path};

fn main() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.quad_to((5.0, 10.0), (10.0, 0.0));

    // xx == 0.0: a "should be positive" (per expand_path's own doc) but not rejected value.
    let expand = Diagonal2::new(0.0, 5.0);
    let expanded = expand_path(path, expand, Join::Miter, 4.0, 1e-3);

    let mut saw_nan = false;
    for el in expanded.elements() {
        let has_nan = match *el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => p.x.is_nan() || p.y.is_nan(),
            PathEl::QuadTo(p1, p2) => {
                p1.x.is_nan() || p1.y.is_nan() || p2.x.is_nan() || p2.y.is_nan()
            }
            PathEl::CurveTo(p1, p2, p3) => {
                p1.x.is_nan()
                    || p1.y.is_nan()
                    || p2.x.is_nan()
                    || p2.y.is_nan()
                    || p3.x.is_nan()
                    || p3.y.is_nan()
            }
            PathEl::ClosePath => false,
        };
        if has_nan {
            saw_nan = true;
        }
        println!("{el:?}");
    }

    if saw_nan {
        eprintln!("REPRODUCED - expand_path returned NaN-poisoned PathEl(s) with no panic/error");
        std::process::exit(1);
    } else {
        println!("did not reproduce - no NaN in output");
    }
}
