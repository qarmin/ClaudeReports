// Standalone reproduction for candidate 005.GAUSSIAN_KERNEL_WRONG_SHAPE.
//
// `crate::math::gaussian::gaussian` below is the real project function (included verbatim from
// `../clone/src/math/gaussian.rs`, see that file's comment), called here at f64 exactly like
// `sampler.rs` calls it in the real resize pipeline (distance in source-pixel units, already
// `.abs()`'d by the caller before it ever reaches this function).
//
// `normal_pdf` is an independent, from-scratch transcription of the textbook Gaussian density
// used only as the reference oracle - it is NOT copied from the project.
//
// Exit code convention: 0 = did not reproduce (kernel matches the textbook Gaussian shape),
// non-zero = reproduced (kernel shape is wrong).

mod math {
    pub(crate) mod consts;
    pub(crate) mod gaussian;
}

use math::gaussian::gaussian;

// The kernel builds sigma from an `f32` literal, so the reference must use the same widened
// value (0.3499999940395355) rather than an exact 0.35, or a mismatch would just be f32 rounding
// noise instead of the real bug.
const SIGMA: f64 = 0.35f32 as f64;

fn normal_pdf(x: f64) -> f64 {
    (-0.5 * (x / SIGMA).powi(2)).exp() / (SIGMA * (2.0 * std::f64::consts::PI).sqrt())
}

fn main() {
    let mut failures: Vec<String> = Vec::new();

    // 1. Differential test against the textbook normal density over the kernel's actual
    // support (min_kernel_size == 2.0 in sampler.rs, i.e. x in [0, 2] in source-pixel units).
    let mut max_abs_err = 0.0f64;
    let mut max_abs_err_x = 0.0f64;
    for i in 0..=2000 {
        let x = i as f64 / 1000.0;
        let got = gaussian::<f64>(x);
        let want = normal_pdf(x);
        let err = (got - want).abs();
        if err > max_abs_err {
            max_abs_err = err;
            max_abs_err_x = x;
        }
    }
    println!("max |gaussian(x) - normal_pdf(x)| over x in [0,2] = {max_abs_err:e} at x = {max_abs_err_x}");
    if max_abs_err > 1e-6 {
        failures.push(format!(
            "does not match textbook normal density: max abs err {max_abs_err:e} at x={max_abs_err_x} (tolerance 1e-6)"
        ));
    }

    // 2. A Gaussian is flat (zero slope) at its centre. exp(-x/den) instead of exp(-x*x/den)
    // puts a cusp there instead.
    let h = 1e-6;
    let slope = (gaussian::<f64>(h) - gaussian::<f64>(0.0)) / h;
    println!("slope at x=0 (central difference, h={h}) = {slope}");
    if slope.abs() > 1e-3 {
        failures.push(format!("nonzero slope at centre (cusp, not a bell): {slope}"));
    }

    // 3. w(sigma)/w(0) must equal exp(-1/2) for a properly-scaled Gaussian.
    let ratio_1sigma = gaussian::<f64>(SIGMA) / gaussian::<f64>(0.0);
    let want_1sigma = (-0.5f64).exp();
    println!("w(sigma)/w(0) = {ratio_1sigma}, want {want_1sigma}");
    if (ratio_1sigma - want_1sigma).abs() > 1e-6 {
        failures.push(format!(
            "one-sigma falloff wrong: w(sigma)/w(0) = {ratio_1sigma}, want {want_1sigma}"
        ));
    }

    // 4. Neighbouring taps (one full source pixel away, x=1.0) must carry real, non-negligible
    // weight relative to the centre tap, or the "Gaussian" filter degenerates to nearest-neighbour
    // once weights are normalized in compute_weights.rs.
    let ratio_1px = gaussian::<f64>(1.0) / gaussian::<f64>(0.0);
    println!("w(1)/w(0) = {ratio_1px:e}");
    if ratio_1px <= 1e-3 {
        failures.push(format!(
            "neighbouring tap is negligible: w(1)/w(0) = {ratio_1px:e} (collapses to nearest-neighbour after normalization)"
        ));
    }

    if failures.is_empty() {
        println!("did not reproduce - gaussian() matches the textbook Gaussian shape");
        std::process::exit(0);
    } else {
        eprintln!("REPRODUCED - {} check(s) failed:", failures.len());
        for f in &failures {
            eprintln!("  - {f}");
        }
        std::process::exit(1);
    }
}
