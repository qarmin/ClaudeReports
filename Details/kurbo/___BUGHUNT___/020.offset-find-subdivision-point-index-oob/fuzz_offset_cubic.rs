use kurbo::offset::offset_cubic;
use kurbo::{BezPath, CubicBez, Point};
use rand::Rng;
use std::panic;

fn main() {
    let mut rng = rand::rng();
    let iters: u64 = std::env::var("ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20_000_000u64);
    let mut found = 0u64;
    for i in 0..iters {
        // Bias toward S-shaped cubics (which have an inflection point) and
        // extreme/adversarial magnitudes, with tiny tolerance to force deep
        // recursion (MAX_DEPTH) in offset_cubic's subdivision logic.
        let scale: f64 = *[1.0, 1e-3, 1e3, 1e6, 1e-6, 1e9, 1e-9]
            .get(rng.random_range(0..7))
            .unwrap();
        let p0 = Point::new(0.0, 0.0);
        let p1 = Point::new(
            rng.random_range(-1.0..1.0) * scale,
            rng.random_range(-1.0..1.0) * scale,
        );
        let p2 = Point::new(
            rng.random_range(-1.0..1.0) * scale,
            rng.random_range(-1.0..1.0) * scale,
        );
        let p3 = Point::new(
            rng.random_range(-1.0..1.0) * scale,
            rng.random_range(-1.0..1.0) * scale,
        );
        let c = CubicBez::new(p0, p1, p2, p3);
        let d: f64 = rng.random_range(-10.0..10.0) * scale;
        let tolerance: f64 = *[1e-3, 1e-6, 1e-9, 1e-12].get(rng.random_range(0..4)).unwrap();

        let mut result = BezPath::new();
        let res = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            offset_cubic(c, d, tolerance, &mut result);
        }));
        if let Err(e) = res {
            found += 1;
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "<unknown panic payload>".to_string()
            };
            eprintln!(
                "PANIC at iter {i}: {msg}\n  c = CubicBez::new(Point::new({:?}, {:?}), Point::new({:?}, {:?}), Point::new({:?}, {:?}), Point::new({:?}, {:?}));\n  d = {d:?}; tolerance = {tolerance:?};",
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
            );
            if msg.contains("index out of bounds") {
                eprintln!("*** MATCHED index-out-of-bounds pattern, stopping ***");
                std::process::exit(1);
            }
            if found > 20 {
                eprintln!("Too many distinct panics, stopping early.");
                std::process::exit(2);
            }
        }
    }
    eprintln!("Done, {iters} iterations, {found} panics found (none index-oob).");
}
