// Standalone reproduction for candidate 007.BARTLETT_HANN_KERNEL_WRONG_SHAPE.
//
// `crate::math::bartlett::bartlett_hann` below is the real project function (included verbatim
// from `../clone/src/math/bartlett.rs`, see that file's comment), called here at f64 exactly
// like `sampler.rs`/`compute_weights.rs` call it in the real resize pipeline: a distance from the
// destination sample's centre, in source-pixel units, over the kernel's declared support
// `min_kernel_size == 2.0` (see `ResamplingFilter::new(bartlett_hann, 2f32)` in `sampler.rs`) -
// i.e. `x` in `[-2, 2]` (the function itself takes `x.abs()` first).
//
// `reference_bartlett_hann` is an independent, from-scratch transcription of the textbook
// Bartlett-Hann window (`0.62 - 0.48*|t-0.5| - 0.38*cos(2*pi*t)` for `t` in `[0,1]`), re-expressed
// over the normalized distance `d = |x|/radius` the same way the sibling `pic-scale` repo's
// already-fixed `bartlett_hann` derives it (see that repo's doc comment/tests) - it is NOT copied
// from either project's fixed code, just derived independently from the textbook formula for this
// repro.
//
// Exit code convention: 0 = did not reproduce (kernel matches the textbook Bartlett-Hann shape),
// non-zero = reproduced (kernel shape is wrong).

mod math {
    pub(crate) mod bartlett;
    pub(crate) mod consts;
}

use math::bartlett::bartlett_hann;

const RADIUS: f64 = 2.0;

fn reference_bartlett_hann(x: f64) -> f64 {
    let x = x.abs();
    if x > RADIUS {
        return 0.0;
    }
    let d = x / RADIUS;
    0.62 - 0.24 * d + 0.38 * (std::f64::consts::PI * d).cos()
}

fn main() {
    let mut failures: Vec<String> = Vec::new();

    // 1. A window function must peak at its own centre (x = 0), value 1.0 for Bartlett-Hann.
    let center = bartlett_hann::<f64>(0.0);
    println!("bartlett_hann(0) = {center}");
    if (center - 1.0).abs() > 1e-9 {
        failures.push(format!(
            "wrong value at centre: bartlett_hann(0) = {center}, want 1.0 (peak weight)"
        ));
    }

    // 2. Differential test against the textbook Bartlett-Hann window over the kernel's actual
    // support (x in [-2, 2], matching min_kernel_size == 2.0 in sampler.rs).
    let mut max_abs_err = 0.0f64;
    let mut max_abs_err_x = 0.0f64;
    for i in -2000..=2000 {
        let x = i as f64 / 1000.0;
        let got = bartlett_hann::<f64>(x);
        let want = reference_bartlett_hann(x);
        let err = (got - want).abs();
        if err > max_abs_err {
            max_abs_err = err;
            max_abs_err_x = x;
        }
    }
    println!(
        "max |bartlett_hann(x) - reference_bartlett_hann(x)| over x in [-2,2] = {max_abs_err:e} at x = {max_abs_err_x}"
    );
    if max_abs_err > 1e-6 {
        failures.push(format!(
            "does not match textbook Bartlett-Hann window: max abs err {max_abs_err:e} at x={max_abs_err_x} (tolerance 1e-6)"
        ));
    }

    // 3. A Bartlett-Hann window must be monotonically non-increasing away from the centre and
    // never negative anywhere on its support. The candidate report suspected "large negative
    // weights near the support edge" - check exactly that.
    let mut prev = f64::INFINITY;
    let mut min_seen = f64::INFINITY;
    let mut min_seen_x = 0.0f64;
    let mut monotonic = true;
    for i in 0..=2000 {
        let x = i as f64 / 1000.0;
        let w = bartlett_hann::<f64>(x);
        if w < min_seen {
            min_seen = w;
            min_seen_x = x;
        }
        if w > prev + 1e-9 {
            monotonic = false;
        }
        prev = w;
    }
    println!("min value over x in [0,2] = {min_seen} at x = {min_seen_x}, monotonic non-increasing = {monotonic}");
    if min_seen < -1e-6 {
        failures.push(format!(
            "negative tap(s) found, most negative = {min_seen} at x={min_seen_x} (a Bartlett-Hann window must stay >= 0)"
        ));
    }
    if !monotonic {
        failures.push("not monotonically non-increasing away from the centre".to_string());
    }

    // 4. Symmetry: w(x) == w(-x).
    let mut max_asym = 0.0f64;
    for i in 0..=200 {
        let x = i as f64 / 100.0;
        let asym = (bartlett_hann::<f64>(x) - bartlett_hann::<f64>(-x)).abs();
        max_asym = max_asym.max(asym);
    }
    println!("max |w(x) - w(-x)| over x in [0,2] = {max_asym:e}");
    if max_asym > 1e-9 {
        failures.push(format!("not symmetric: max |w(x)-w(-x)| = {max_asym:e}"));
    }

    if failures.is_empty() {
        println!("did not reproduce - bartlett_hann() matches the textbook Bartlett-Hann window shape");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - {} check(s) failed:", failures.len());
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
