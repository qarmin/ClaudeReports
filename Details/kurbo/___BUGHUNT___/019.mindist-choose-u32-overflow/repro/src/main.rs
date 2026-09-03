// Repro for candidate 019: `mindist.rs`'s `choose(n, k) -> u32` accumulates the binomial
// coefficient C(n, k) in a plain `u32`. `min_dist_param`'s own doc comment claims support for
// "arbitrary Bezier curves up to and including degree 255", but `choose` is called (via
// `basis_function`/`C_rk`/`A_r`) with arguments up to `2 * n` - and C(2n, n) already exceeds
// `u32::MAX` (~4.29e9) starting around n = 16-18 (e.g. C(36, 18) = 9,075,135,300), independently
// of (and at a much lower degree than) the separate `2 * n` u8-overflow issue documented in
// candidate 006 (which only bites above n = 127).
//
// `min_dist_param`/`choose` are `pub(crate)`/private inside kurbo, so they cannot be called from
// an external crate's public-API surface at all (min_dist_param's only real caller,
// `PathSeg::min_dist`, always passes n,m <= 3, since `PathSeg` only has Line/Quad/Cubic
// variants - see candidate 006's RESULT.md for the same reachability caveat, which applies here
// identically). To still exercise the *real, unmodified* function - not a reimplementation or a
// hand-approximated stand-in - this repro pulls in the literal `mindist.rs` source file from
// `./clone` (created by `just clone`, and patched in place by `just apply`) via `#[path]`, so
// `cargo run` always compiles and calls whatever code is actually sitting in the clone at the
// time - pristine before the fix, patched after.
//
// The input itself is not faked/hand-approximated either: it's 19 real, valid Bezier control
// points (a degree-18 curve) against a 2-point line - exactly the kind of input the function's
// own doc comment says it accepts (well within even candidate 006's proposed degree-127 limit).

#[path = "../../clone/kurbo/src/mindist.rs"]
#[allow(
    non_snake_case,
    dead_code,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
mod mindist;

// `mindist.rs` resolves its own `use crate::Vec2;` against *this* crate's root, so re-export the
// real kurbo `Vec2` type under that name here.
pub use kurbo::Vec2;

fn main() {
    // Degree-18 Bezier curve (19 control points): C(2*18, 18) = C(36, 18) = 9,075,135,300,
    // already past u32::MAX (4,294,967,295).
    let bez1: Vec<Vec2> = (0..=18).map(|i| Vec2::new(f64::from(i), 0.0)).collect();
    let bez2: Vec<Vec2> = vec![Vec2::new(0.0, 1.0), Vec2::new(1.0, 2.0)];
    assert_eq!(bez1.len(), 19, "bez1 must be a degree-18 Bezier curve");

    println!(
        "calling min_dist_param with a degree-{} Bezier curve (n = {}); \
         C(2n, n) = C({}, {}) exceeds u32::MAX ({}) well within the function's own documented \
         degree-255 support",
        bez1.len() - 1,
        bez1.len() - 1,
        2 * (bez1.len() - 1),
        bez1.len() - 1,
        u32::MAX,
    );

    let result = std::panic::catch_unwind(|| {
        mindist::min_dist_param(&bez1, &bez2, (0.0, 1.0), (0.0, 1.0), 0.001, None)
    });

    match result {
        Ok(value) => {
            let (distance, ..) = value;
            if distance.is_nan() {
                println!(
                    "REPRODUCED (release-style silent wraparound) - min_dist_param completed \
                     without panicking, but returned a NaN/bogus result due to `choose()`'s u32 \
                     overflow: {value:?}"
                );
                std::process::exit(1);
            } else {
                println!("did not reproduce - min_dist_param completed normally: {value:?}");
            }
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!("REPRODUCED (debug-assertions panic) - min_dist_param panicked: {msg}");
            std::process::exit(1);
        }
    }
}
