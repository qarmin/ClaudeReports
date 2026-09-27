// Repro for candidate 006: `min_dist_param`'s own doc comment claims support for "arbitrary
// Bezier curves up to and including degree 255", but `n`/`m` are stored as `u8` and `2 * n`/
// `2 * m` (the loop bounds used throughout `S`/`C_rk`/`A_r`/`basis_function`) is itself computed
// in `u8` arithmetic - which overflows for any `n > 127`, well before the documented degree-255
// boundary.
//
// `min_dist_param` is `pub(crate)` inside kurbo, so it cannot be called from an external crate's
// public-API surface at all (its only real caller, `PathSeg::min_dist`, always passes n,m <= 3,
// since `PathSeg` only has Line/Quad/Cubic variants). To still exercise the *real, unmodified*
// function - not a reimplementation or a hand-approximated stand-in - this repro pulls in the
// literal `mindist.rs` source file from `./clone` (created by `just clone`, and patched in place
// by `just apply`) via `#[path]`, so `cargo run` always compiles and calls whatever code is
// actually sitting in the clone at the time - pristine before the fix, patched after.
//
// The input itself is not faked/hand-approximated either: it's 129 real, valid Bezier control
// points (a degree-128 curve), exactly the kind of input the function's own doc comment says it
// accepts.

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
    let bez1: Vec<Vec2> = (0..=128).map(|i| Vec2::new(f64::from(i), 0.0)).collect();
    let bez2: Vec<Vec2> = vec![Vec2::new(0.0, 1.0), Vec2::new(1.0, 2.0)];
    assert_eq!(bez1.len(), 129, "bez1 must be a degree-128 Bezier curve");

    println!(
        "calling min_dist_param with a degree-{} Bezier curve (n = {}), \
         per the function's own doc comment this should be supported up to degree 255",
        bez1.len() - 1,
        bez1.len() - 1
    );

    let result = std::panic::catch_unwind(|| {
        mindist::min_dist_param(&bez1, &bez2, (0.0, 1.0), (0.0, 1.0), 0.001, None)
    });

    match result {
        Ok(value) => {
            println!("did not reproduce - min_dist_param completed normally: {value:?}");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!("REPRODUCED - min_dist_param panicked: {msg}");
            std::process::exit(1);
        }
    }
}
