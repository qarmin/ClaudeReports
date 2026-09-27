// Appended to polycool/src/yuksel.rs by this candidate's justfile (`just inject`) so the
// adversarial cases can call the real, pub(crate) `find_root` directly from inside the crate
// (it is not reachable from an external dependent crate, since it is not `pub`).
//
// These f/deriv pairs are deliberately hand-constructed to try to trigger the theorized failure
// mode from the candidate write-up: a Newton step that underflows to exactly 0.0 at a point that
// is not the true root, with x_error <= 0 so the only other exits (val_x == 0.0 exactly, or
// x becoming non-finite) are unavailable, hopefully forcing reliance on the interval-bisection
// fallback in a way that might defeat it.

#[cfg(test)]
mod bughunt_034_tests {
    use super::find_root;
    use std::cell::Cell;
    use std::time::Instant;

    fn adversarial_f(target: f64) -> impl Fn(f64) -> f64 {
        move |x: f64| {
            if x < target {
                -1e-300
            } else if x > target {
                1e-300
            } else {
                0.0
            }
        }
    }
    fn adversarial_deriv() -> impl Fn(f64) -> f64 {
        move |_x: f64| 1e300
    }

    fn run_with_budget(
        x_error: f64,
        lower: f64,
        upper: f64,
        target: f64,
        max_iters: u64,
    ) -> (f64, u64, bool) {
        let f = adversarial_f(target);
        let deriv = adversarial_deriv();
        let val_lower = f(lower);
        let val_upper = f(upper);
        let counter = Cell::new(0u64);
        let counting_f = |x: f64| {
            counter.set(counter.get() + 1);
            f(x)
        };
        let start = Instant::now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            find_root(counting_f, deriv, lower, upper, val_lower, val_upper, x_error)
        }));
        let timed_out = start.elapsed().as_secs() > 5;
        let x = match result {
            Ok(v) => v,
            Err(_) => f64::NAN,
        };
        let iters = counter.get();
        (x, iters, timed_out || iters > max_iters)
    }

    #[test]
    fn adversarial_fixed_point_negative_x_error() {
        for &x_error in &[-1.0f64, 0.0, -1e300, f64::NEG_INFINITY] {
            let (x, iters, bad) = run_with_budget(x_error, -1.0, 1.0, 0.31415, 100_000);
            eprintln!("x_error={x_error} -> x={x} iters={iters} bad={bad}");
            assert!(
                !bad,
                "find_root did not terminate quickly for x_error={x_error}: iters={iters}"
            );
        }
    }

    #[test]
    fn adversarial_fixed_point_wide_range() {
        for &x_error in &[-1.0f64, 0.0] {
            let (x, iters, bad) = run_with_budget(x_error, -1e10, 1e10, 12345.6789, 100_000);
            eprintln!("wide x_error={x_error} -> x={x} iters={iters} bad={bad}");
            assert!(
                !bad,
                "find_root did not terminate quickly (wide) for x_error={x_error}: iters={iters}"
            );
        }
    }

    #[test]
    fn adjacent_floats_and_random_targets_negative_x_error() {
        let lower = 1.0f64;
        let upper = f64::from_bits(lower.to_bits() + 1);
        let f = |x: f64| if x < 1.0000000000000002 { -1.0 } else { 1.0 };
        let deriv = |_x: f64| 1e300;
        let val_lower = f(lower);
        let val_upper = f(upper);
        let counter = Cell::new(0u64);
        let counting_f = |x: f64| {
            counter.set(counter.get() + 1);
            f(x)
        };
        let x = find_root(counting_f, deriv, lower, upper, val_lower, val_upper, -1.0);
        eprintln!("adjacent-floats x={x} iters={}", counter.get());
        assert!(counter.get() < 1000);
    }

    #[test]
    fn many_random_targets_zero_deriv_and_negative_x_error() {
        // deriv() == 0 everywhere forces step to be +/-inf every iteration, which stresses the
        // "new_x <= lower || new_x >= upper" bisection-fallback path on every single iteration.
        let mut seed = 0x243F_6A88_85A3_08D3u64;
        let mut rand = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        for trial in 0..500 {
            let target = rand() * 2.0 - 1.0;
            let f = move |x: f64| {
                if x < target {
                    -1.0
                } else if x > target {
                    1.0
                } else {
                    0.0
                }
            };
            let deriv = |_x: f64| 0.0f64;
            let lower = -1.0;
            let upper = 1.0;
            let val_lower = f(lower);
            let val_upper = f(upper);
            let counter = Cell::new(0u64);
            let counting_f = |x: f64| {
                counter.set(counter.get() + 1);
                f(x)
            };
            let x = find_root(counting_f, deriv, lower, upper, val_lower, val_upper, -1.0);
            let iters = counter.get();
            assert!(iters < 2000, "trial {trial} target {target} took {iters} iters, x={x}");
        }
        eprintln!("500 random-target trials with deriv=0 all terminated quickly");
    }
}
