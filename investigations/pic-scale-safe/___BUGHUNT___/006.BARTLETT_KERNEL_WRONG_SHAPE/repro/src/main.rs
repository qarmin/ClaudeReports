// Standalone reproduction for candidate 006.BARTLETT_KERNEL_WRONG_SHAPE.
//
// `crate::math::bartlett::bartlett` below is the real project function (included verbatim from
// `../clone/src/math/bartlett.rs`, see that file's comment), called here at f64 exactly like
// `sampler.rs`/`compute_weights.rs` call it in the real resize pipeline: a non-negative distance
// from the destination sample's centre, in source-pixel units, over the kernel's declared support
// `min_kernel_size == 2.0` (see `ResamplingFilter::new(bartlett, 2f32)` in `sampler.rs`) - i.e.
// `x` in `[0, 2]` at a scale factor of 1 (`compute_weights.rs` always passes `dx.abs()`, so `x`
// is never actually negative in the real pipeline).
//
// `reference_triangle` is an independent, from-scratch transcription of the textbook triangular
// (Bartlett) window normalized to this same support - it is NOT copied from the project.
//
// Exit code convention: 0 = did not reproduce (kernel matches the textbook triangular shape),
// non-zero = reproduced (kernel shape is wrong).

mod math {
    pub(crate) mod bartlett;
    pub(crate) mod consts;
}

use math::bartlett::bartlett;

const RADIUS: f64 = 2.0;

fn reference_triangle(x: f64) -> f64 {
    let x = x.abs();
    if x >= RADIUS {
        0.0
    } else {
        1.0 - x / RADIUS
    }
}

fn main() {
    let mut failures: Vec<String> = Vec::new();

    // 1. A window function must peak at its own centre (x = 0). This is the core of the bug:
    // the shipped code returns `2*0 = 0` at the centre instead of the maximum weight.
    let center = bartlett::<f64>(0.0);
    println!("bartlett(0) = {center}");
    if (center - 1.0).abs() > 1e-9 {
        failures.push(format!(
            "wrong value at centre: bartlett(0) = {center}, want 1.0 (peak weight)"
        ));
    }

    // 2. Differential test against the textbook triangular window over the kernel's actual
    // support (x in [0, 2], matching min_kernel_size == 2.0 in sampler.rs).
    let mut max_abs_err = 0.0f64;
    let mut max_abs_err_x = 0.0f64;
    for i in 0..=2000 {
        let x = i as f64 / 1000.0;
        let got = bartlett::<f64>(x);
        let want = reference_triangle(x);
        let err = (got - want).abs();
        if err > max_abs_err {
            max_abs_err = err;
            max_abs_err_x = x;
        }
    }
    println!(
        "max |bartlett(x) - reference_triangle(x)| over x in [0,2] = {max_abs_err:e} at x = {max_abs_err_x}"
    );
    if max_abs_err > 1e-9 {
        failures.push(format!(
            "does not match textbook triangular window: max abs err {max_abs_err:e} at x={max_abs_err_x} (tolerance 1e-9)"
        ));
    }

    // 3. A triangular window must be monotonically non-increasing away from the centre, and
    // never negative. The shipped code rises from 0 at x=0 to 2 at x=1, then jumps down and goes
    // as low as -2 at x=2.
    let mut prev = f64::INFINITY;
    let mut min_seen = f64::INFINITY;
    let mut monotonic = true;
    for i in 0..=2000 {
        let x = i as f64 / 1000.0;
        let w = bartlett::<f64>(x);
        min_seen = min_seen.min(w);
        if w > prev + 1e-9 {
            monotonic = false;
        }
        prev = w;
    }
    println!("min value over x in [0,2] = {min_seen}, monotonic non-increasing = {monotonic}");
    if min_seen < -1e-9 {
        failures.push(format!(
            "negative tap(s) found, most negative = {min_seen} (a triangular window must stay >= 0)"
        ));
    }
    if !monotonic {
        failures.push("not monotonically non-increasing away from the centre".to_string());
    }

    // 4. Continuity check right at the branch boundary x=1, where the shipped code has a jump
    // discontinuity of size 2 (2*1 = 2 approaching from below, 2-2*1 = 0 approaching from above).
    let left = bartlett::<f64>(1.0 - 1e-6);
    let right = bartlett::<f64>(1.0 + 1e-6);
    println!("bartlett(1-eps) = {left}, bartlett(1+eps) = {right}");
    if (left - right).abs() > 1e-3 {
        failures.push(format!(
            "discontinuous at x=1: left={left}, right={right}, jump={}",
            (left - right).abs()
        ));
    }

    if failures.is_empty() {
        println!("did not reproduce - bartlett() matches the textbook triangular window shape");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - {} check(s) failed:", failures.len());
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
