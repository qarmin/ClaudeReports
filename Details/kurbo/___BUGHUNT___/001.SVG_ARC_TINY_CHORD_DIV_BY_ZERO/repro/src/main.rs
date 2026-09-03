// Standalone reproduction for kurbo's `Arc::from_svg_arc` debug_assert / div-by-zero on a
// tiny-but-nonzero rotated half-chord between an SVG arc's endpoints.
//
// Public API only: `kurbo::BezPath::from_svg`, the exact entry point a real caller parsing an
// untrusted/adversarial SVG path string would use.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panic in debug, or a NaN
// leaking into the parsed path in release).

fn main() {
    // A tiny arc whose end point differs from its start point by only 1e-200: `from != to`, so
    // `SvgArc::is_straight_line()`'s early-out does not trigger, but the rotated half-chord's
    // squared components underflow to exactly 0.0 in f64, hitting the `sum_of_sq != 0.0`
    // invariant `Arc::from_svg_arc` assumes.
    let input = "M0,0 A25,25 0 0,1 1e-200,0";

    let result = std::panic::catch_unwind(|| kurbo::BezPath::from_svg(input));

    match result {
        Ok(Ok(path)) => {
            let has_nan = path.elements().iter().any(|el| {
                use kurbo::PathEl::*;
                match *el {
                    MoveTo(p) | LineTo(p) => p.x.is_nan() || p.y.is_nan(),
                    QuadTo(p1, p2) => {
                        [p1, p2].iter().any(|p| p.x.is_nan() || p.y.is_nan())
                    }
                    CurveTo(p1, p2, p3) => {
                        [p1, p2, p3].iter().any(|p| p.x.is_nan() || p.y.is_nan())
                    }
                    ClosePath => false,
                }
            });
            if has_nan {
                eprintln!("REPRODUCED (release-mode NaN propagation) - parsed path contains NaN: {:?}", path);
                std::process::exit(1);
            } else {
                println!("did not reproduce - parsed cleanly: {:?}", path);
            }
        }
        Ok(Err(e)) => {
            println!("did not reproduce - parse error (not the suspected bug): {:?}", e);
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED (debug_assert panic) - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
