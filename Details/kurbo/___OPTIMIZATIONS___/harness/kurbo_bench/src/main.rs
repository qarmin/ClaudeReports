//! Standalone benchmark harness for kurbo, built for the `___OPTIMIZATIONS___` workspace.
//!
//! Usage:  kurbo_bench <scene> [iters]      (iters default 50; ~10% warmup discarded)
//!         kurbo_bench --list
//!         kurbo_bench --dump <offset|stroke>   (not timed; every output coordinate at full
//!                                               f64 precision, for diffing two builds)
//!
//! Prints one `RESULT` line (median ms + spread), one `CHECK` line (a deterministic checksum of
//! the scene's own output, so a patch that changes results is caught), and one `MEM` line
//! (allocation count/bytes for the timed region, plus peak RSS for the whole process).
//!
//! Design notes:
//! - every scene builds its input ONCE, outside the timed loop;
//! - `black_box` wraps both the input and the result so nothing is optimized away;
//! - the checksum is recomputed every iteration and compared against the first one, so a
//!   nondeterministic scene fails loudly instead of quietly averaging two behaviours.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use kurbo::{
    Affine, BezPath, CubicBez, CurveFitSample, Line, ParamCurve, ParamCurveArclen, ParamCurveDeriv,
    ParamCurveFit, ParamCurveNearest, PathEl, PathSeg, Point, QuadBez, Rect, Shape, Stroke,
    StrokeOpts, Vec2, common, fit_to_bezpath, fit_to_bezpath_opt, flatten, offset::offset_cubic,
    simplify::{SimplifyOptions, simplify_bezpath},
    stroke,
};

/// The offset curve of a cubic, as a `ParamCurveFit` source.
///
/// kurbo's own `CubicOffset` is private, and `fit_to_bezpath` is only reachable from outside the
/// crate through a user-supplied `ParamCurveFit` - so the harness supplies one shaped like the
/// motivating use case (offsetting), rather than reaching for the one public impl
/// (`SimplifyBezPath`), which the `simplify` scene already covers.
struct OffsetCurve {
    c: CubicBez,
    d: f64,
}

impl OffsetCurve {
    #[inline]
    fn eval_deriv(&self, t: f64) -> (Point, Vec2) {
        let d1 = self.c.deriv();
        let d2 = d1.deriv();
        let v1 = d1.eval(t).to_vec2();
        let v2 = d2.eval(t).to_vec2();
        let n = v1.hypot();
        if n < 1e-12 {
            return (self.c.eval(t), v1);
        }
        let perp1 = Vec2::new(v1.y, -v1.x);
        let perp2 = Vec2::new(v2.y, -v2.x);
        let p = self.c.eval(t) + perp1 * (self.d / n);
        let dp = v1 + (perp2 / n - perp1 * (v1.dot(v2) / (n * n * n))) * self.d;
        (p, dp)
    }
}

impl ParamCurveFit for OffsetCurve {
    fn sample_pt_tangent(&self, t: f64, _sign: f64) -> CurveFitSample {
        let (p, tangent) = self.eval_deriv(t);
        CurveFitSample { p, tangent }
    }
    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        self.eval_deriv(t)
    }
    fn break_cusp(&self, _range: std::ops::Range<f64>) -> Option<f64> {
        None
    }
}

// ---------------------------------------------------------------------------
// counting allocator
// ---------------------------------------------------------------------------

static ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(new_size.saturating_sub(layout.size()) as u64, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static A: Counting = Counting;

fn peak_rss_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// checksum: order-sensitive mix of f64 bit patterns
// ---------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct Hash(u64);

impl Hash {
    fn new() -> Self {
        Self(0xcbf29ce484222325)
    }
    #[inline]
    fn f(&mut self, v: f64) {
        // NaN has many bit patterns; canonicalize so a NaN-producing scene is still stable.
        let bits = if v.is_nan() { 0x7ff8_0000_0000_0000 } else { v.to_bits() };
        self.0 ^= bits;
        self.0 = self.0.wrapping_mul(0x100000001b3);
        self.0 ^= self.0 >> 29;
    }
    #[inline]
    fn p(&mut self, p: Point) {
        self.f(p.x);
        self.f(p.y);
    }
    #[inline]
    fn el(&mut self, el: PathEl) {
        match el {
            PathEl::MoveTo(p) => {
                self.f(1.0);
                self.p(p);
            }
            PathEl::LineTo(p) => {
                self.f(2.0);
                self.p(p);
            }
            PathEl::QuadTo(a, b) => {
                self.f(3.0);
                self.p(a);
                self.p(b);
            }
            PathEl::CurveTo(a, b, c) => {
                self.f(4.0);
                self.p(a);
                self.p(b);
                self.p(c);
            }
            PathEl::ClosePath => self.f(5.0),
        }
    }
    fn path(&mut self, p: &BezPath) {
        for el in p.elements() {
            self.el(*el);
        }
    }
}

// ---------------------------------------------------------------------------
// deterministic input generation
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9e3779b97f4a7c15) | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// Uniform in [0, 1).
    fn f(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f() * (hi - lo)
    }
}

/// A path made of `subpaths` closed subpaths of `per` cubic segments each, on a 1000x1000 canvas.
/// Control points wander smoothly so the curves are realistically curvy rather than degenerate.
fn gen_path(subpaths: usize, per: usize, seed: u64) -> BezPath {
    let mut rng = Rng::new(seed);
    let mut path = BezPath::new();
    for _ in 0..subpaths {
        let cx = rng.range(100.0, 900.0);
        let cy = rng.range(100.0, 900.0);
        let r = rng.range(20.0, 120.0);
        let mut prev = Point::new(cx + r, cy);
        path.move_to(prev);
        for i in 0..per {
            let a0 = (i as f64) / (per as f64) * std::f64::consts::TAU;
            let a1 = ((i + 1) as f64) / (per as f64) * std::f64::consts::TAU;
            let rr = r * rng.range(0.6, 1.5);
            let p3 = Point::new(cx + rr * a1.cos(), cy + rr * a1.sin());
            let k = r * 0.55 * rng.range(0.5, 2.0);
            let p1 = Point::new(prev.x - k * a0.sin(), prev.y + k * a0.cos());
            let p2 = Point::new(p3.x + k * a1.sin(), p3.y - k * a1.cos());
            path.curve_to(p1, p2, p3);
            prev = p3;
        }
        path.close_path();
    }
    path
}

/// A path mixing all four element kinds, for scenes where element variety matters.
fn gen_mixed_path(subpaths: usize, per: usize, seed: u64) -> BezPath {
    let mut rng = Rng::new(seed);
    let mut path = BezPath::new();
    for _ in 0..subpaths {
        let mut prev = Point::new(rng.range(0.0, 1000.0), rng.range(0.0, 1000.0));
        path.move_to(prev);
        for i in 0..per {
            let step = 40.0;
            let nx = (prev.x + rng.range(-step, step)).clamp(0.0, 1000.0);
            let ny = (prev.y + rng.range(-step, step)).clamp(0.0, 1000.0);
            let np = Point::new(nx, ny);
            match i % 3 {
                0 => path.line_to(np),
                1 => path.quad_to(
                    Point::new(rng.range(0.0, 1000.0), rng.range(0.0, 1000.0)),
                    np,
                ),
                _ => path.curve_to(
                    Point::new(prev.x + rng.range(-60.0, 60.0), prev.y + rng.range(-60.0, 60.0)),
                    Point::new(nx + rng.range(-60.0, 60.0), ny + rng.range(-60.0, 60.0)),
                    np,
                ),
            }
            prev = np;
        }
        path.close_path();
    }
    path
}

fn gen_cubics(n: usize, seed: u64) -> Vec<CubicBez> {
    let mut rng = Rng::new(seed);
    (0..n)
        .map(|_| {
            let p = |rng: &mut Rng| Point::new(rng.range(0.0, 100.0), rng.range(0.0, 100.0));
            CubicBez::new(p(&mut rng), p(&mut rng), p(&mut rng), p(&mut rng))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// scenes
// ---------------------------------------------------------------------------

/// Each scene returns a boxed closure: one call = one timed iteration, returning a checksum.
type Scene = Box<dyn FnMut() -> u64>;

const SCENES: &[(&str, &str)] = &[
    ("flatten", "BezPath::flatten of a 200x8-cubic path, tolerance 0.25"),
    ("flatten-fine", "same path, tolerance 0.001 (much more output per curve)"),
    ("flatten-small", "20x8-cubic path, tolerance 0.25 (second problem size)"),
    ("stroke", "stroke() expansion of a 100x8-cubic path, width 4, round joins"),
    ("stroke-miter", "same, miter joins + butt caps"),
    ("dash", "stroke() with a 4-entry dash pattern"),
    ("offset", "offset_cubic over 2000 random cubics, d=5, tol=1e-3"),
    ("simplify", "simplify_bezpath of a flattened path, accuracy 1.0"),
    ("arclen", "ParamCurveArclen::arclen over 2000 cubics, accuracy 1e-9"),
    ("arclen-coarse", "same, accuracy 1e-3"),
    ("area", "Shape::area + perimeter over a 200x8-cubic path"),
    ("bbox", "Shape::bounding_box over a 200x8-cubic path"),
    ("affine", "BezPath::apply_affine over a 200x8-cubic path"),
    ("svg-parse", "BezPath::from_svg of a ~1600-element path string"),
    ("svg-write", "BezPath::to_svg of the same path"),
    ("nearest", "ParamCurveNearest::nearest over 2000 cubics, accuracy 1e-6"),
    ("intersect", "PathSeg::intersect_line, 2000 cubics x 4 lines"),
    ("segments", "BezPath::segments iteration + eval(0.5) over a 200x8-cubic path"),
    ("fit", "fit_to_bezpath over 500 offset-curve sources, accuracy 1e-3"),
    ("fit-opt", "fit_to_bezpath_opt over 20 offset-curve sources, accuracy 1e-3"),
    ("solve-cubic", "common::solve_cubic, 200k calls"),
    ("solve-quartic", "common::solve_quartic, 200k calls"),
    ("mixed-flatten", "flatten of a mixed line/quad/cubic path"),
    ("deriv", "ParamCurveDeriv::deriv + eval over 2000 cubics"),
    ("subdivide", "ParamCurve::subsegment over 2000 cubics x 8 ranges"),
];

fn make_scene(name: &str) -> Option<Scene> {
    Some(match name {
        "flatten" => {
            let path = gen_path(200, 8, 1);
            Box::new(move || {
                let mut h = Hash::new();
                flatten(black_box(&path).iter(), black_box(0.25), |el| h.el(el));
                black_box(h.0)
            })
        }
        "flatten-fine" => {
            let path = gen_path(200, 8, 1);
            Box::new(move || {
                let mut h = Hash::new();
                flatten(black_box(&path).iter(), black_box(0.001), |el| h.el(el));
                black_box(h.0)
            })
        }
        "flatten-small" => {
            let path = gen_path(20, 8, 1);
            Box::new(move || {
                let mut h = Hash::new();
                flatten(black_box(&path).iter(), black_box(0.25), |el| h.el(el));
                black_box(h.0)
            })
        }
        "mixed-flatten" => {
            let path = gen_mixed_path(100, 30, 7);
            Box::new(move || {
                let mut h = Hash::new();
                flatten(black_box(&path).iter(), black_box(0.25), |el| h.el(el));
                black_box(h.0)
            })
        }
        "stroke" => {
            let path = gen_path(100, 8, 2);
            let style = Stroke::new(4.0);
            Box::new(move || {
                let out = stroke(black_box(&path).iter(), black_box(&style), &StrokeOpts::default(), 1e-4);
                let mut h = Hash::new();
                h.path(&out);
                black_box(h.0)
            })
        }
        "stroke-miter" => {
            let path = gen_path(100, 8, 2);
            let style = Stroke::new(4.0)
                .with_join(kurbo::Join::Miter)
                .with_caps(kurbo::Cap::Butt)
                .with_miter_limit(6.0);
            Box::new(move || {
                let out = stroke(black_box(&path).iter(), black_box(&style), &StrokeOpts::default(), 1e-4);
                let mut h = Hash::new();
                h.path(&out);
                black_box(h.0)
            })
        }
        "dash" => {
            let path = gen_path(60, 8, 3);
            let style = Stroke::new(3.0).with_dashes(0.0, [8.0, 4.0, 2.0, 4.0]);
            Box::new(move || {
                let out = stroke(black_box(&path).iter(), black_box(&style), &StrokeOpts::default(), 1e-4);
                let mut h = Hash::new();
                h.path(&out);
                black_box(h.0)
            })
        }
        "offset" => {
            let cubics = gen_cubics(2000, 4);
            Box::new(move || {
                let mut out = BezPath::new();
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    out.truncate(0);
                    offset_cubic(*c, 5.0, 1e-3, &mut out);
                    h.path(&out);
                }
                black_box(h.0)
            })
        }
        "simplify" => {
            let src = gen_path(50, 8, 5);
            let mut flat = BezPath::new();
            flatten(src.iter(), 0.1, |el| flat.push(el));
            Box::new(move || {
                let out = simplify_bezpath(
                    black_box(&flat).clone(),
                    black_box(1.0),
                    &SimplifyOptions::default(),
                );
                let mut h = Hash::new();
                h.path(&out);
                black_box(h.0)
            })
        }
        "arclen" => {
            let cubics = gen_cubics(2000, 6);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    h.f(c.arclen(black_box(1e-9)));
                }
                black_box(h.0)
            })
        }
        "arclen-coarse" => {
            let cubics = gen_cubics(2000, 6);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    h.f(c.arclen(black_box(1e-3)));
                }
                black_box(h.0)
            })
        }
        "area" => {
            let path = gen_path(200, 8, 8);
            Box::new(move || {
                let p = black_box(&path);
                let mut h = Hash::new();
                h.f(p.area());
                h.f(p.perimeter(1e-6));
                black_box(h.0)
            })
        }
        "bbox" => {
            let path = gen_path(200, 8, 8);
            Box::new(move || {
                let r: Rect = black_box(&path).bounding_box();
                let mut h = Hash::new();
                h.f(r.x0);
                h.f(r.y0);
                h.f(r.x1);
                h.f(r.y1);
                black_box(h.0)
            })
        }
        "affine" => {
            let path = gen_path(200, 8, 8);
            let a = Affine::rotate(0.3) * Affine::scale(1.7) * Affine::translate(Vec2::new(3.0, -2.0));
            Box::new(move || {
                let mut p = black_box(&path).clone();
                p.apply_affine(black_box(a));
                let mut h = Hash::new();
                h.path(&p);
                black_box(h.0)
            })
        }
        "svg-parse" => {
            let path = gen_path(200, 8, 9);
            let s = path.to_svg();
            Box::new(move || {
                let p = BezPath::from_svg(black_box(&s)).unwrap();
                let mut h = Hash::new();
                h.path(&p);
                black_box(h.0)
            })
        }
        "svg-write" => {
            let path = gen_path(200, 8, 9);
            Box::new(move || {
                let s = black_box(&path).to_svg();
                let mut h = Hash::new();
                h.f(s.len() as f64);
                for b in s.as_bytes() {
                    h.0 = h.0.wrapping_mul(31).wrapping_add(*b as u64);
                }
                black_box(h.0)
            })
        }
        "nearest" => {
            let cubics = gen_cubics(2000, 10);
            let pt = Point::new(50.0, 50.0);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let n = c.nearest(black_box(pt), 1e-6);
                    h.f(n.t);
                    h.f(n.distance_sq);
                }
                black_box(h.0)
            })
        }
        "intersect" => {
            let cubics = gen_cubics(2000, 11);
            let lines = [
                Line::new((0.0, 0.0), (100.0, 100.0)),
                Line::new((0.0, 50.0), (100.0, 50.0)),
                Line::new((50.0, 0.0), (50.0, 100.0)),
                Line::new((0.0, 100.0), (100.0, 0.0)),
            ];
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let seg = PathSeg::Cubic(*c);
                    for l in &lines {
                        for isect in seg.intersect_line(*l) {
                            h.f(isect.segment_t);
                            h.f(isect.line_t);
                        }
                    }
                }
                black_box(h.0)
            })
        }
        "segments" => {
            let path = gen_path(200, 8, 12);
            Box::new(move || {
                let mut h = Hash::new();
                for seg in black_box(&path).segments() {
                    h.p(seg.eval(0.5));
                    h.p(seg.start());
                    h.p(seg.end());
                }
                black_box(h.0)
            })
        }
        "fit" => {
            let cubics = gen_cubics(500, 13);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let src = OffsetCurve { c: *c, d: 3.0 };
                    let out = fit_to_bezpath(&src, 1e-3);
                    h.path(&out);
                }
                black_box(h.0)
            })
        }
        "fit-opt" => {
            // Deliberately small: fit_to_bezpath_opt is ~4.5 ms per source curve, ~100x
            // fit_to_bezpath's cost, so 20 sources is already a ~90 ms iteration.
            let cubics = gen_cubics(20, 13);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let src = OffsetCurve { c: *c, d: 3.0 };
                    let out = fit_to_bezpath_opt(&src, 1e-3);
                    h.path(&out);
                }
                black_box(h.0)
            })
        }
        "solve-cubic" => {
            let mut rng = Rng::new(14);
            let coeffs: Vec<[f64; 4]> = (0..200_000)
                .map(|_| {
                    [
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                    ]
                })
                .collect();
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&coeffs) {
                    for r in common::solve_cubic(c[0], c[1], c[2], c[3]) {
                        h.f(r);
                    }
                }
                black_box(h.0)
            })
        }
        "solve-quartic" => {
            let mut rng = Rng::new(15);
            let coeffs: Vec<[f64; 5]> = (0..200_000)
                .map(|_| {
                    [
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                        rng.range(-10.0, 10.0),
                    ]
                })
                .collect();
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&coeffs) {
                    for r in common::solve_quartic(c[0], c[1], c[2], c[3], c[4]) {
                        h.f(r);
                    }
                }
                black_box(h.0)
            })
        }
        "deriv" => {
            let cubics = gen_cubics(2000, 16);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    let d: QuadBez = c.deriv();
                    for i in 0..8 {
                        h.p(d.eval(i as f64 / 8.0));
                    }
                }
                black_box(h.0)
            })
        }
        "subdivide" => {
            let cubics = gen_cubics(2000, 17);
            Box::new(move || {
                let mut h = Hash::new();
                for c in black_box(&cubics) {
                    for i in 0..8 {
                        let a = i as f64 / 8.0;
                        let sub = c.subsegment(a..(a + 0.125));
                        h.p(sub.p0);
                        h.p(sub.p3);
                    }
                }
                black_box(h.0)
            })
        }
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// driver
// ---------------------------------------------------------------------------

/// Print every coordinate produced by a scene, one per line at full precision, so two builds can
/// be compared numerically (max absolute / relative difference) rather than by checksum alone.
fn dump(what: &str) {
    let emit = |v: f64| println!("{v:.17e}");
    match what {
        "offset" => {
            let cubics = gen_cubics(2000, 4);
            let mut out = BezPath::new();
            for c in &cubics {
                out.truncate(0);
                offset_cubic(*c, 5.0, 1e-3, &mut out);
                for el in out.elements() {
                    match *el {
                        PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                            emit(p.x);
                            emit(p.y);
                        }
                        PathEl::QuadTo(a, b) => {
                            emit(a.x);
                            emit(a.y);
                            emit(b.x);
                            emit(b.y);
                        }
                        PathEl::CurveTo(a, b, c) => {
                            emit(a.x);
                            emit(a.y);
                            emit(b.x);
                            emit(b.y);
                            emit(c.x);
                            emit(c.y);
                        }
                        PathEl::ClosePath => {}
                    }
                }
            }
        }
        "stroke" => {
            let path = gen_path(100, 8, 2);
            let style = Stroke::new(4.0);
            let out = stroke(path.iter(), &style, &StrokeOpts::default(), 1e-4);
            for el in out.elements() {
                match *el {
                    PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                        emit(p.x);
                        emit(p.y);
                    }
                    PathEl::QuadTo(a, b) => {
                        emit(a.x);
                        emit(a.y);
                        emit(b.x);
                        emit(b.y);
                    }
                    PathEl::CurveTo(a, b, c) => {
                        emit(a.x);
                        emit(a.y);
                        emit(b.x);
                        emit(b.y);
                        emit(c.x);
                        emit(c.y);
                    }
                    PathEl::ClosePath => {}
                }
            }
        }
        "arclen" => {
            let cubics = gen_cubics(2000, 6);
            for c in &cubics {
                emit(c.arclen(1e-9));
            }
        }
        "solve-cubic" => {
            let mut rng = Rng::new(14);
            for _ in 0..200_000 {
                let c = [
                    rng.range(-10.0, 10.0),
                    rng.range(-10.0, 10.0),
                    rng.range(-10.0, 10.0),
                    rng.range(-10.0, 10.0),
                ];
                // Fixed-width so the two dumps stay line-aligned even if the root COUNT differs.
                let roots = common::solve_cubic(c[0], c[1], c[2], c[3]);
                for i in 0..3 {
                    emit(roots.get(i).copied().unwrap_or(f64::NAN));
                }
            }
        }
        other => {
            eprintln!("unknown dump target: {other} (try offset, stroke, arclen or solve-cubic)");
            std::process::exit(2);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] == "--list" || args[1] == "-h" || args[1] == "--help" {
        println!("usage: kurbo_bench <scene> [iters]");
        println!("scenes:");
        for (n, d) in SCENES {
            println!("  {n:<16} {d}");
        }
        return;
    }
    // Not a timed scene: dumps every coordinate a scene produces, at full f64 precision, so two
    // builds can be diffed numerically rather than only by checksum. Used to characterize an
    // output-changing candidate (SKILL.md "Correctness" -> characterized difference).
    if args[1] == "--dump" {
        let what = args.get(2).map(|s| s.as_str()).unwrap_or("offset");
        dump(what);
        return;
    }

    let name = args[1].clone();
    let iters: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);

    let Some(mut scene) = make_scene(&name) else {
        eprintln!("unknown scene: {name} (try --list)");
        std::process::exit(2);
    };

    // Warmup: ~10% of iters, minimum 2. Not timed, and its allocations are not counted.
    let warmup = (iters / 10).max(2);
    let mut check = 0u64;
    for _ in 0..warmup {
        check = scene();
    }

    let alloc0 = ALLOC_COUNT.load(Ordering::Relaxed);
    let bytes0 = ALLOC_BYTES.load(Ordering::Relaxed);

    let mut times = Vec::with_capacity(iters);
    for i in 0..iters {
        let t = Instant::now();
        let c = scene();
        let dt = t.elapsed().as_secs_f64() * 1000.0;
        if c != check {
            eprintln!("NONDETERMINISTIC: iteration {i} checksum {c:#018x} != {check:#018x}");
            std::process::exit(3);
        }
        times.push(dt);
    }

    let allocs = ALLOC_COUNT.load(Ordering::Relaxed) - alloc0;
    let bytes = ALLOC_BYTES.load(Ordering::Relaxed) - bytes0;

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = times.len();
    let median = times[n / 2];
    let mean = times.iter().sum::<f64>() / n as f64;
    let p95 = times[(n * 95 / 100).min(n - 1)];

    println!("scene {name} iters {iters} warmup {warmup}");
    println!(
        "RESULT median {median:.4} ms  mean {mean:.4}  p95 {p95:.4}  spread {:.4}..{:.4}",
        times[0],
        times[n - 1]
    );
    println!("CHECK {check:#018x}");
    println!(
        "MEM allocs/iter {:.1}  bytes/iter {:.0}  peak_rss_kb {}",
        allocs as f64 / iters as f64,
        bytes as f64 / iters as f64,
        peak_rss_kb()
    );
}
