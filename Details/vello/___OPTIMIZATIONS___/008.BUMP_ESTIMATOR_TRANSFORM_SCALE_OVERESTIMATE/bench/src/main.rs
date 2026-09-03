//! Standalone bench/measurement harness for `vello_encoding::BumpEstimator`.
//!
//! Modes (first CLI arg):
//!   size   - print `Scene::bump_estimate` totals for several representative
//!            scenes/transforms (accuracy dimension: estimated buffer bytes).
//!   perf   - time building+estimating a large synthetic scene, N iterations
//!            (CPU cost dimension).

use std::time::Instant;

use vello::Scene;
use vello::kurbo::{Affine, BezPath, Cap, Join, PathEl, Point, Stroke};
use vello::peniko::color::palette;

fn star_path(cx: f64, cy: f64, r_outer: f64, r_inner: f64, points: usize) -> BezPath {
    let mut path = BezPath::new();
    for i in 0..points * 2 {
        let r = if i % 2 == 0 { r_outer } else { r_inner };
        let theta = std::f64::consts::PI * i as f64 / points as f64;
        let p = Point::new(cx + r * theta.cos(), cy + r * theta.sin());
        if i == 0 {
            path.push(PathEl::MoveTo(p));
        } else {
            path.push(PathEl::LineTo(p));
        }
    }
    path.push(PathEl::ClosePath);
    path
}

fn wavy_curve_path(x0: f64, y0: f64, len: f64, segments: usize) -> BezPath {
    let mut path = BezPath::new();
    path.push(PathEl::MoveTo(Point::new(x0, y0)));
    let seg_len = len / segments as f64;
    for i in 0..segments {
        let x = x0 + seg_len * i as f64;
        path.push(PathEl::CurveTo(
            Point::new(x + seg_len * 0.33, y0 - 20.0),
            Point::new(x + seg_len * 0.66, y0 + 20.0),
            Point::new(x + seg_len, y0),
        ));
    }
    path
}

/// Builds a scene with a representative mix of stroked stars and wavy stroked curves
/// (round caps/joins included, which is the case PR #1561 tightens the most), all under
/// the given `transform`, and returns its `bump_estimate(None)` memory total in bytes.
fn build_and_estimate(transform: Affine, n_stars: usize, n_curves: usize) -> u32 {
    let mut scene = Scene::new();
    let round_stroke = Stroke {
        width: 6.0,
        join: Join::Round,
        start_cap: Cap::Round,
        end_cap: Cap::Round,
        miter_limit: 4.0,
        dash_pattern: Default::default(),
        dash_offset: 0.0,
    };
    for i in 0..n_stars {
        let cx = (i % 50) as f64 * 60.0;
        let cy = (i / 50) as f64 * 60.0;
        let star = star_path(cx, cy, 20.0, 8.0, 5);
        scene.stroke(&round_stroke, transform, palette::css::BLACK, None, &star);
    }
    for i in 0..n_curves {
        let y0 = i as f64 * 15.0;
        let curve = wavy_curve_path(0.0, y0, 400.0, 8);
        scene.stroke(&round_stroke, transform, palette::css::BLACK, None, &curve);
    }
    scene.bump_estimate(None).total
}

fn run_size_mode() {
    let n_stars = 400;
    let n_curves = 400;
    let cases: &[(&str, Affine)] = &[
        ("identity", Affine::IDENTITY),
        ("scale_2x", Affine::scale(2.0)),
        ("scale_0.5x", Affine::scale(0.5)),
        ("rotate_37deg", Affine::rotate(37.0_f64.to_radians())),
        (
            "rotate_scale_1.7x",
            Affine::rotate(51.0_f64.to_radians()) * Affine::scale(1.7),
        ),
    ];
    println!("case,total_bytes");
    for (name, transform) in cases {
        let total = build_and_estimate(*transform, n_stars, n_curves);
        println!("{name},{total}");
    }
}

fn run_perf_mode(iters: usize) {
    let n_stars = 400;
    let n_curves = 400;
    // Warmup.
    for _ in 0..(iters / 10).max(1) {
        std::hint::black_box(build_and_estimate(
            std::hint::black_box(Affine::IDENTITY),
            n_stars,
            n_curves,
        ));
    }
    let mut samples = Vec::with_capacity(iters);
    for _ in 0..iters {
        let start = Instant::now();
        let total = build_and_estimate(std::hint::black_box(Affine::IDENTITY), n_stars, n_curves);
        std::hint::black_box(total);
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    let min = samples[0];
    let p50 = samples[n / 2];
    let p95 = samples[(n * 95) / 100];
    let max = samples[n - 1];
    let mean = samples.iter().sum::<f64>() / n as f64;
    println!("iters={n} min_ms={min:.4} p50_ms={p50:.4} mean_ms={mean:.4} p95_ms={p95:.4} max_ms={max:.4}");
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "size".to_string());
    match mode.as_str() {
        "size" => run_size_mode(),
        "perf" => {
            let iters: usize = std::env::args()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(200);
            run_perf_mode(iters);
        }
        other => eprintln!("unknown mode: {other}"),
    }
}
