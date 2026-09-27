// Repro for candidate 044 (POLYCOOL_CUBIC_CRITICAL_POINTS_NEAR_ZERO_LEADING_COEFF).
//
// This is a negative-result ("Not reproducible") repro. It runs two escalating checks against
// real public API entry points, both stressing `polycool::Cubic::critical_points`'s `a = 3.0 *
// self.coeffs[3]` near-zero-but-nonzero leading-coefficient path:
//
// 1. `cubic_probe`: directly fuzzes the public `polycool::Cubic::roots_between` (which is exactly
//    what `critical_points`/`first_root` back) against a brute-force sign-change ground truth,
//    with the leading coefficient spanning down to 1e-320.
// 2. `nearest_probe`: drives the real end-to-end `kurbo::CubicBez::nearest` entry point (the
//    actual caller reached via `CubicBez::nearest`'s quintic-derivative recursion the candidate
//    names) on near-degenerate ("nearly quadratic") cubic beziers, comparing its reported nearest
//    distance against a brute-force sampled ground truth.
//
// Both report 0 mismatches across millions of trials; see RESULT.md for the full analysis of why.

use kurbo::{CubicBez, ParamCurve, ParamCurveNearest, Point};
use polycool::Cubic;

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
    fn signed_mag(&mut self, lo_exp: f64, hi_exp: f64) -> f64 {
        let exp = self.range(lo_exp, hi_exp);
        let sign = if self.next_f64() < 0.5 { 1.0 } else { -1.0 };
        sign * 10f64.powf(exp)
    }
}

fn brute_force_root_count(coeffs: [f64; 4], lower: f64, upper: f64, n: usize) -> usize {
    let eval = |x: f64| -> f64 {
        let [c0, c1, c2, c3] = coeffs;
        c0 + c1 * x + c2 * x * x + c3 * x * x * x
    };
    let mut count = 0;
    let mut last = eval(lower);
    for i in 1..=n {
        let x = lower + (upper - lower) * (i as f64 / n as f64);
        let val = eval(x);
        if (last < 0.0) != (val < 0.0) && last != 0.0 && val != 0.0 {
            count += 1;
        }
        last = val;
    }
    count
}

fn cubic_probe(trials: u64) -> u64 {
    let mut rng = Rng(0xdead_beef_cafe_1234);
    let mut mismatches = 0u64;

    for _ in 0..trials {
        let (lower, upper) = (0.0, 1.0);
        let c1 = rng.signed_mag(-20.0, 2.0);
        let c2 = rng.signed_mag(-20.0, 2.0);
        let c0 = rng.signed_mag(-20.0, 2.0);
        // Leading coefficient spans down to 1e-320, directly stressing
        // `a = 3.0 * self.coeffs[3]` in `Cubic::critical_points`.
        let c3 = rng.signed_mag(-320.0, 2.0);

        let coeffs = [c0, c1, c2, c3];
        let poly = Cubic::new(coeffs);
        let roots = poly.roots_between(lower, upper, 1e-12);

        let eval = |x: f64| -> f64 { c0 + c1 * x + c2 * x * x + c3 * x * x * x };
        let scale = c0.abs().max(c1.abs()).max(c2.abs()).max(c3.abs());
        for &r in &roots {
            if !(lower..=upper).contains(&r) {
                println!("OUT OF RANGE root: coeffs={coeffs:?} r={r}");
                mismatches += 1;
            }
            let v = eval(r);
            if v.abs() > 1e-3 * scale {
                println!("SPURIOUS root (not actually a root): coeffs={coeffs:?} r={r} eval={v}");
                mismatches += 1;
            }
        }

        let bf_count = brute_force_root_count(coeffs, lower, upper, 20_000);
        if roots.len() < bf_count {
            mismatches += 1;
            println!(
                "MISSED root(s): coeffs={coeffs:?} found={} brute_force_sign_changes={}",
                roots.len(),
                bf_count
            );
        }
    }
    mismatches
}

fn brute_force_nearest(c: &CubicBez, p: Point, n: usize) -> f64 {
    let mut best = f64::INFINITY;
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let d = (c.eval(t) - p).hypot2();
        if d < best {
            best = d;
        }
    }
    best
}

fn nearest_probe(trials: u64) -> u64 {
    let mut rng = Rng(0x1234_5678_9abc_def1);
    let mut mismatches = 0u64;
    let mut worst_ratio = 1.0_f64;

    for _ in 0..trials {
        let mag = 10f64.powf(rng.range(-2.0, 8.0));
        let p0 = Point::new(rng.range(-mag, mag), rng.range(-mag, mag));
        let p1 = Point::new(rng.range(-mag, mag), rng.range(-mag, mag));
        let p2 = Point::new(rng.range(-mag, mag), rng.range(-mag, mag));

        // p3 chosen so the cubic term q3 = -p0+3p1-3p2+p3 is tiny (nearly
        // quadratic bezier), which is what drives the innermost cubic's
        // leading coefficient `a` in `critical_points` towards zero when
        // `CubicBez::nearest` recurses quintic -> quartic -> cubic.
        let quad_p3_x = p0.x - 3.0 * p1.x + 3.0 * p2.x;
        let quad_p3_y = p0.y - 3.0 * p1.y + 3.0 * p2.y;
        let eps = 10f64.powf(rng.range(-300.0, -1.0));
        let p3 = Point::new(
            quad_p3_x + rng.range(-eps, eps),
            quad_p3_y + rng.range(-eps, eps),
        );

        let c = CubicBez::new(p0, p1, p2, p3);
        let p = Point::new(rng.range(-mag, mag), rng.range(-mag, mag));

        let nearest = c.nearest(p, 1e-9);
        let bf = brute_force_nearest(&c, p, 20_000);

        let ratio = if bf > 1e-12 { nearest.distance_sq / bf } else { 1.0 };
        if ratio > worst_ratio {
            worst_ratio = ratio;
        }
        // A meaningfully worse-than-ground-truth result (not just fp noise)
        // would indicate a missed critical point / wrong root count.
        if ratio > 1.0 + 1e-6 {
            mismatches += 1;
            println!(
                "WORSE-THAN-BRUTE-FORCE nearest: p0={p0:?} p1={p1:?} p2={p2:?} p3={p3:?} p={p:?} nearest.distance_sq={} brute_force={}",
                nearest.distance_sq, bf
            );
        }
    }
    println!("nearest_probe: worst_ratio observed = {worst_ratio}");
    mismatches
}

fn main() {
    let cubic_trials: u64 = std::env::var("CUBIC_TRIALS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(500_000);
    let nearest_trials: u64 = std::env::var("NEAREST_TRIALS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(500_000);

    println!("== cubic_probe ({cubic_trials} trials, directly fuzzing polycool::Cubic::roots_between) ==");
    let m1 = cubic_probe(cubic_trials);
    println!("cubic_probe mismatches: {m1}");

    println!();
    println!("== nearest_probe ({nearest_trials} trials, end-to-end kurbo::CubicBez::nearest) ==");
    let m2 = nearest_probe(nearest_trials);
    println!("nearest_probe mismatches: {m2}");

    println!();
    println!("total mismatches: {}", m1 + m2);
}
