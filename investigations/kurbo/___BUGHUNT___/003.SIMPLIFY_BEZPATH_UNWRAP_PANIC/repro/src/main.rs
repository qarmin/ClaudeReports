// Standalone reproduction for: `simplify_bezpath` panics on a `PathEl` sequence that
// does not start with `MoveTo`.
//
// `simplify_bezpath`'s public signature accepts `impl IntoIterator<Item = PathEl>`, not
// specifically a `BezPath` (whose own "must start with MoveTo" invariant is only enforced
// by a `debug_assert!` on its own builder methods, not on arbitrary `PathEl` sequences fed
// to this function). A caller building a `Vec<PathEl>` by hand (or filtering/transforming
// an existing path's elements) can easily produce a sequence whose first element is a
// `LineTo`/`QuadTo`/`CurveTo` instead of a `MoveTo` - kurbo's own public API offers no type
// that rules this out at compile time.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::simplify::{simplify_bezpath, SimplifyOptions};
use kurbo::{PathEl, Point};

fn main() {
    // The minimal triggering input: a single LineTo with no preceding MoveTo.
    let input = vec![PathEl::LineTo(Point::new(1.0, 1.0))];

    let result = std::panic::catch_unwind(|| {
        simplify_bezpath(input, 1.0, &SimplifyOptions::default())
    });

    match result {
        Ok(value) => {
            println!("did not reproduce - simplify_bezpath completed normally: {value:?}");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - simplify_bezpath panicked: {panic_payload:?}");
            std::process::exit(1);
        }
    }
}
