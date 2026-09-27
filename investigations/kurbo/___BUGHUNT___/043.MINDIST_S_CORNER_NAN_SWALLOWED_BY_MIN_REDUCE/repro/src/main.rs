// Repro for candidate 043: does `min_dist_param`'s
// `svalues.iter().map(...).reduce(f64::min)` (kurbo/src/mindist.rs, computing `alpha` from the
// 4 corner evaluations of `S(u,v,bez1,bez2)`) ever silently swallow a single NaN corner (since
// `f64::min(NaN, x) == x`) while the other 3 corners stay finite - which would corrupt the
// branch-and-bound pruning bound `alpha` into a plausible-looking but wrong finite number,
// instead of visibly propagating to a NaN result the way already-confirmed candidates
// 006/019 (mindist.rs `u8`/`u32` overflow at high curve degree) do?
//
// `min_dist_param`/`S`/`D_rk` are private/`pub(crate)` inside kurbo - `S`/`D_rk` are not even
// `pub(crate)` (fully module-private), so they cannot be called directly from outside the
// `mindist` module at all, real caller or not. To still exercise the *real, unmodified* code
// (not a reimplementation), this repro vendors the literal `mindist.rs` from `./clone` (created
// by `just clone`) via `#[path]`, exactly like candidates 006/019/030's own repros, and drives it
// through the one function that *is* reachable from outside the module (`pub(crate)
// min_dist_param`) with the same high-degree-curve inputs already known (candidates 006/019) to
// make `choose()` overflow and produce a NaN `D_rk(r,k)` term somewhere in the summation.
//
// The key structural fact this repro checks: `D_rk(r,k)` (and hence any NaN it produces) does
// NOT depend on `u`/`v` at all - it's a function of `(r,k,bez1,bez2)` only. Since IEEE-754
// multiplication/addition propagate NaN unconditionally (`NaN * 0.0 == NaN`, not `0.0`), a NaN
// `D_rk(r,k)` term poisons **every** `S(u,v)` evaluation identically, regardless of which corner
// `(u,v)` is evaluated at - so all 4 `svalues` entries become NaN together, never a mix of one
// NaN and three finite corners. That means `reduce(f64::min)` never actually gets a chance to
// silently swallow a lone NaN here: either all 4 inputs are NaN (and the reduce correctly
// produces NaN back out, matching candidates 006/019's already-documented behavior) or all 4 are
// finite (and the reduce behaves normally).
//
// This repro exercises the real end-to-end `min_dist_param` entry point (not a hand-rebuilt
// stand-in for its internals) across a spread of curve degrees straddling the known
// `choose()`-overflow threshold (n ~= 16, per candidate 019), and reports, for each one, whether
// the final returned distance is NaN or a plausible finite number - looking specifically for a
// finite-but-wrong result that would indicate a swallowed partial-NaN corruption.

#[path = "../../clone/kurbo/src/mindist.rs"]
#[allow(non_snake_case, dead_code, clippy::all, clippy::pedantic, clippy::nursery)]
mod mindist;

pub use kurbo::Vec2;

fn main() {
    let mut any_finite_after_overflow_threshold = false;
    for n_use in [4u32, 8, 12, 15, 16, 17, 18, 20, 24, 32, 40, 64] {
        let bez1: Vec<Vec2> = (0..=n_use)
            .map(|i| Vec2::new(f64::from(i), f64::from(i) * f64::from(i)))
            .collect();
        let bez2: Vec<Vec2> = (0..=3).map(|i| Vec2::new(1000.0 + f64::from(i), 500.0 + f64::from(i))).collect();

        let result = std::panic::catch_unwind(|| {
            mindist::min_dist_param(&bez1, &bez2, (0.0, 1.0), (0.0, 1.0), 0.001, None)
        });

        match result {
            Ok((alpha, t1, t2)) => {
                println!(
                    "n={n_use:>3}: min_dist_param -> alpha={alpha} t1={t1} t2={t2} \
                     is_nan={}",
                    alpha.is_nan()
                );
                if n_use >= 16 && alpha.is_finite() {
                    // Past the known choose()-overflow threshold (candidate 019: n~=16), a
                    // finite (non-NaN) result here would indicate the reduce silently swallowed
                    // a partial NaN into a wrong-but-plausible answer - the bug this candidate
                    // hypothesizes.
                    any_finite_after_overflow_threshold = true;
                }
            }
            Err(panic_payload) => {
                let msg = panic_payload
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                    .unwrap_or("<non-string panic payload>");
                println!("n={n_use:>3}: panicked (expected in a debug build past the overflow threshold): {msg}");
            }
        }
    }

    if any_finite_after_overflow_threshold {
        eprintln!(
            "REPRODUCED: min_dist_param returned a finite (non-NaN) result at a curve degree \
             past the known choose()-overflow threshold - possible swallowed-NaN corruption."
        );
        std::process::exit(1);
    } else {
        println!(
            "NOT REPRODUCED: every past-threshold run either panicked (debug build) or returned \
             NaN (release build) - never a silently-corrupted finite result."
        );
    }
}
