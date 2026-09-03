// Repro for candidate 030: does `C_rk`'s `upsilon + 1` (kurbo/src/mindist.rs:196) /
// `varsigma + 1` (mindist.rs:204) overflow `u8` when a real caller drives `min_dist_param` with
// a degree-255 Bezier curve (256 control points), the crate's own documented boundary?
//
// `min_dist_param` is `pub(crate)` inside kurbo, so - same as candidate 006 - it cannot be called
// from an external crate's public-API surface at all. To still exercise the *real, unmodified*
// function (not a reimplementation or a hand-approximated stand-in), this repro pulls in the
// literal `mindist.rs` source file from `./clone` (created by `just clone`) via `#[path]`, so
// `cargo run`/`cargo run --release` always compile and call whatever code is actually sitting in
// the clone.
//
// The input is not faked/hand-approximated either: it's 256 real, valid Bezier control points on
// each curve (a genuine degree-255 curve), exactly the boundary the function's own doc comment
// claims to support.
//
// Result (see RESULT.md/output.txt for the full picture, including a closed-form sweep over all
// possible degrees proving this can never be reached through any real call path): in a debug
// build this panics at mindist.rs:176 (`S()`'s own `2 * n` overflow - already-confirmed candidate
// 006), never at the `upsilon + 1`/`varsigma + 1` lines this candidate names. In a release build,
// no panic occurs at all (wraps silently, eventually returns a NaN from an unrelated overflow -
// candidate 019's `choose()` u32 overflow).

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
    // 256 control points on each curve => n = m = 255, exactly the documented degree-255 boundary
    // this candidate's overflow is claimed to require.
    let bez1: Vec<Vec2> = (0..256).map(|i| Vec2::new(f64::from(i), f64::from(i) * 3.0)).collect();
    let bez2: Vec<Vec2> = (0..256)
        .map(|i| Vec2::new(1000.0 + f64::from(i), 500.0 + f64::from(i) * 2.0))
        .collect();
    assert_eq!(bez1.len(), 256, "bez1 must be a degree-255 Bezier curve");
    assert_eq!(bez2.len(), 256, "bez2 must be a degree-255 Bezier curve");

    println!(
        "calling min_dist_param with degree-{}/degree-{} Bezier curves (n = m = 255), \
         the function's own doc comment's documented boundary",
        bez1.len() - 1,
        bez2.len() - 1
    );

    let result = std::panic::catch_unwind(|| {
        mindist::min_dist_param(&bez1, &bez2, (0.0, 1.0), (0.0, 1.0), 0.001, None)
    });

    match result {
        Ok(value) => {
            println!(
                "did not reproduce the candidate's claimed upsilon+1/varsigma+1 overflow - \
                 min_dist_param completed normally (though the result itself may be wrong for \
                 unrelated, already-confirmed reasons - candidate 019's choose() overflow): {value:?}"
            );
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!(
                "panicked (see whether the message/location below matches this candidate's \
                 upsilon+1/varsigma+1 claim at mindist.rs:196/204, or a different, already-known \
                 overflow like candidate 006's mindist.rs:176 `2 * n`): {msg}"
            );
            std::process::exit(1);
        }
    }
}
