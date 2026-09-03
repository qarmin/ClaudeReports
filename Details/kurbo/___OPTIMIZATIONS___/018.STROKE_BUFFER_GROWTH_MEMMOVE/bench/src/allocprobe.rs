use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::hint::black_box;

use kurbo::{BezPath, PathEl, Point, Stroke, StrokeOpts, stroke};

thread_local! {
    static LOG: RefCell<Vec<(usize, usize)>> = RefCell::new(Vec::new());
    static ON: RefCell<bool> = const { RefCell::new(false) };
}

struct Probe;

fn record(old: usize, new: usize) {
    let on = ON.try_with(|o| *o.borrow()).unwrap_or(false);
    if on {
        // Turn recording off while the log itself may grow, to avoid re-entrancy.
        ON.with(|o| *o.borrow_mut() = false);
        LOG.with(|l| l.borrow_mut().push((old, new)));
        ON.with(|o| *o.borrow_mut() = true);
    }
}

unsafe impl GlobalAlloc for Probe {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(0, layout.size());
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record(layout.size(), new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static A: Probe = Probe;

// Same generator as kurbo_bench's gen_path(100, 8, 2).
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

fn main() {
    let el = core::mem::size_of::<PathEl>();
    println!("size_of::<PathEl>() = {el}");
    let path = gen_path(100, 8, 2);
    println!("input path elements = {}", path.elements().len());
    let style = Stroke::new(4.0);

    // warm up allocator
    for _ in 0..3 {
        black_box(stroke(path.iter(), &style, &StrokeOpts::default(), 1e-4));
    }

    LOG.with(|l| l.borrow_mut().clear());
    ON.with(|o| *o.borrow_mut() = true);
    let out = stroke(path.iter(), &style, &StrokeOpts::default(), 1e-4);
    ON.with(|o| *o.borrow_mut() = false);
    println!("output elements = {}", out.elements().len());

    LOG.with(|l| {
        let log = l.borrow();
        println!("allocations during one stroke() call: {}", log.len());
        let mut bytes_req = 0usize;
        let mut moved = 0usize;
        for (old, new) in log.iter() {
            bytes_req += new.saturating_sub(*old);
            if *old > 0 {
                moved += *old;
            }
            let kind = if *old == 0 { "alloc " } else { "realloc" };
            println!(
                "  {kind} {old:>8} -> {new:>8} bytes  ({} -> {} PathEl)",
                old / el.max(1),
                new / el.max(1)
            );
        }
        println!("total new bytes requested = {bytes_req}");
        println!("bytes potentially memmoved by realloc (worst case) = {moved}");
    });
}
