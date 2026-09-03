use kurbo::offset::offset_cubic;
use kurbo::{BezPath, CubicBez, Point};

/// Small deterministic xorshift PRNG so this repro has no external `rand` dependency.
struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn next_f64_01(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next_f64_01() * (hi - lo)
    }
}

fn next_up(x: f64) -> f64 {
    if x == 0.0 {
        return 5e-324;
    }
    let bits = x.to_bits() as i64;
    let bits = if x > 0.0 { bits + 1 } else { bits - 1 };
    f64::from_bits(bits as u64)
}
fn next_down(x: f64) -> f64 {
    -next_up(-x)
}

/// This mirrors, in isolation, exactly the arithmetic of the `while arc_angles[i].abs() <
/// target { target -= arc_angles[i].abs(); i += 1; }` loop in
/// `CubicOffset::find_subdivision_point` (kurbo/src/offset.rs), without needing to construct a
/// real curve that happens to produce a given `arc_angles` triple. Returns Some((i, len)) if the
/// index would go out of bounds (i.e. what the candidate hypothesis predicts), None otherwise.
fn isolated_consume(a: &[f64]) -> Option<(usize, usize)> {
    let n = a.len();
    let mut sum = 0.0f64;
    for &v in a {
        sum += v.abs();
    }
    let mut target = sum * 0.5;
    let mut i = 0usize;
    loop {
        if i >= n {
            return Some((i, n));
        }
        if a[i].abs() < target {
            target -= a[i].abs();
            i += 1;
        } else {
            return None;
        }
    }
}

fn approach2_isolated_arithmetic() -> (u64, u64) {
    let pi = std::f64::consts::PI;
    let base = [
        0.0,
        5e-324,
        1e-300,
        1e-16,
        1e-10,
        pi / 4.0,
        pi / 2.0,
        pi * 0.75,
        pi,
        pi - 1e-16,
        pi - 1e-10,
    ];
    let mut grid: Vec<f64> = Vec::new();
    for &b in &base {
        grid.push(b);
        grid.push(next_up(b));
        grid.push(next_down(b));
    }
    grid.sort_by(|x, y| x.partial_cmp(y).unwrap());
    grid.dedup();

    let mut total = 0u64;
    let mut oob = 0u64;
    let mut check = |combo: &[f64]| {
        total += 1;
        if let Some((i, len)) = isolated_consume(combo) {
            oob += 1;
            if oob <= 5 {
                println!("ISOLATED OOB: combo={combo:?} -> i={i} len={len}");
            }
        }
    };
    // n == 2
    for &a in &grid {
        for &b in &grid {
            check(&[a, b]);
        }
    }
    // n == 3
    for &a in &grid {
        for &b in &grid {
            for &c in &grid {
                check(&[a, b, c]);
            }
        }
    }
    (total, oob)
}

fn approach3_real_fuzz(rng: &mut Rng, iters: u64) -> (u64, u64) {
    let scales = [1.0f64, 1e-3, 1e3, 1e6, 1e-6, 1e9, 1e-9];
    let tolerances = [1e-3f64, 1e-6, 1e-9, 1e-12];
    let mut panics = 0u64;
    for i in 0..iters {
        let scale = scales[(rng.next_u64() as usize) % scales.len()];
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale);
        let p2 = Point::new(rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale);
        let p3 = Point::new(rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale);
        let c = CubicBez::new(p0, p1, p2, p3);
        let d = rng.range(-10.0, 10.0) * scale;
        let tolerance = tolerances[(rng.next_u64() as usize) % tolerances.len()];

        let mut result = BezPath::new();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            offset_cubic(c, d, tolerance, &mut result);
        }));
        if let Err(e) = res {
            panics += 1;
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "<unknown panic payload>".to_string()
            };
            if panics <= 10 {
                println!(
                    "FUZZ PANIC at iter {i}: {msg}\n  c = {c:?}\n  d = {d:?}; tolerance = {tolerance:?};"
                );
            }
        }
    }
    (iters, panics)
}

fn main() {
    println!("=== Approach 1+2: exhaustive/adversarial-grid check of the isolated arc_angles-consumption arithmetic ===");
    let (total, oob) = approach2_isolated_arithmetic();
    println!("isolated grid total={total} oob={oob}");

    println!("\n=== Approach 3: real end-to-end fuzz of the public kurbo::offset::offset_cubic API ===");
    let mut rng = Rng(0x853c49e6748fea9b);
    let iters: u64 = std::env::var("ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000_000);
    let (total, panics) = approach3_real_fuzz(&mut rng, iters);
    println!("fuzz total={total} panics={panics}");

    if oob == 0 && panics == 0 {
        println!("\nRESULT: did not reproduce (isolated_oob=0, fuzz_panics=0)");
    } else {
        println!("\nRESULT: REPRODUCED (isolated_oob={oob}, fuzz_panics={panics})");
        std::process::exit(1);
    }
}
