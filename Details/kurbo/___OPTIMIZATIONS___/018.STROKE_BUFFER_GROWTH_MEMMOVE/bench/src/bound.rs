//! Upper bound on candidate 018: how much of the `stroke` scene is the StrokeCtx buffer
//! allocation/growth at all?  Compares stroke() (fresh StrokeCtx, four Vecs grown from zero every
//! call) against stroke_with() reusing one warmed StrokeCtx (zero allocations after the first
//! call).  Anything a reserve-capacity patch could win is a subset of this difference.
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use kurbo::{BezPath, PathEl, Point, Stroke, StrokeCtx, StrokeOpts, stroke, stroke_with};

static N: AtomicU64 = AtomicU64::new(0);
static B: AtomicU64 = AtomicU64::new(0);
struct C;
unsafe impl GlobalAlloc for C {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        N.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(l.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, ns: usize) -> *mut u8 {
        N.fetch_add(1, Ordering::Relaxed);
        B.fetch_add(ns.saturating_sub(l.size()) as u64, Ordering::Relaxed);
        unsafe { System.realloc(p, l, ns) }
    }
}
#[global_allocator]
static A: C = C;

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
    fn f(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn gen_path(subpaths: usize, curves: usize, seed: u64) -> BezPath {
    let mut rng = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    let mut path = BezPath::new();
    for _ in 0..subpaths {
        let mut p = Point::new(rng.f() * 1000.0, rng.f() * 1000.0);
        path.push(PathEl::MoveTo(p));
        for _ in 0..curves {
            let p1 = Point::new(p.x + (rng.f() - 0.5) * 200.0, p.y + (rng.f() - 0.5) * 200.0);
            let p2 = Point::new(p1.x + (rng.f() - 0.5) * 200.0, p1.y + (rng.f() - 0.5) * 200.0);
            let p3 = Point::new(p2.x + (rng.f() - 0.5) * 200.0, p2.y + (rng.f() - 0.5) * 200.0);
            path.push(PathEl::CurveTo(p1, p2, p3));
            p = p3;
        }
    }
    path
}

fn sum(p: &BezPath) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for el in p.elements() {
        let pts: &[Point] = match el {
            PathEl::MoveTo(a) | PathEl::LineTo(a) => &[*a],
            PathEl::QuadTo(a, b) => &[*a, *b],
            PathEl::CurveTo(a, b, c) => &[*a, *b, *c],
            PathEl::ClosePath => &[],
        };
        for q in pts {
            for v in [q.x, q.y] {
                h ^= v.to_bits();
                h = h.wrapping_mul(0x100000001b3);
            }
        }
    }
    h
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn main() {
    let iters: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(40);
    let path = gen_path(100, 8, 2);
    let style = Stroke::new(4.0);
    let opts = StrokeOpts::default();

    let mut ctx = StrokeCtx::default();
    // Warm the reusable context so its four Vecs are already at final capacity.
    for _ in 0..5 {
        stroke_with(path.iter(), &style, &opts, 1e-4, &mut ctx);
        black_box(ctx.output());
    }
    let h_reuse = sum(ctx.output());
    let h_fresh = sum(&stroke(path.iter(), &style, &opts, 1e-4));
    println!("checksum fresh {h_fresh:#x}  reused {h_reuse:#x}  equal={}", h_fresh == h_reuse);

    for _ in 0..5 {
        black_box(stroke(black_box(&path).iter(), black_box(&style), &opts, 1e-4));
    }

    let mut a = Vec::new();
    let mut b = Vec::new();
    let (mut na, mut nb, mut ba, mut bb) = (0u64, 0u64, 0u64, 0u64);
    for _ in 0..iters {
        let n0 = N.load(Ordering::Relaxed);
        let b0 = B.load(Ordering::Relaxed);
        let t = Instant::now();
        let out = stroke(black_box(&path).iter(), black_box(&style), &opts, 1e-4);
        a.push(t.elapsed().as_secs_f64() * 1e3);
        black_box(&out);
        na += N.load(Ordering::Relaxed) - n0;
        ba += B.load(Ordering::Relaxed) - b0;
        drop(out);

        let n0 = N.load(Ordering::Relaxed);
        let b0 = B.load(Ordering::Relaxed);
        let t = Instant::now();
        stroke_with(black_box(&path).iter(), black_box(&style), &opts, 1e-4, black_box(&mut ctx));
        b.push(t.elapsed().as_secs_f64() * 1e3);
        black_box(ctx.output());
        nb += N.load(Ordering::Relaxed) - n0;
        bb += B.load(Ordering::Relaxed) - b0;
    }
    let (ma, mb) = (med(a), med(b));
    println!("stroke()      median {ma:.4} ms   allocs/iter {:.1}  bytes/iter {}", na as f64 / iters as f64, ba / iters as u64);
    println!("stroke_with() median {mb:.4} ms   allocs/iter {:.1}  bytes/iter {}", nb as f64 / iters as f64, bb / iters as u64);
    println!("delta {:+.2}%", 100.0 * (mb / ma - 1.0));
}
