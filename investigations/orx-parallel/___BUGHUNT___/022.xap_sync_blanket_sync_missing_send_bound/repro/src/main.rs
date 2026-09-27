// Real end-to-end trigger: the actual public `par_recursive(...).map(h).for_each(...)` API,
// executed on a real thread pool (real OS threads via `std::thread::scope`).
//
// `h` below only needs to satisfy `Fn(..) + Copy + Send` -- the bound `ParRec::map` (and every
// sibling combinator) actually requires (see `Xap::map` in src/infallible/xap.rs). Sync is never
// required of a user-supplied closure anywhere in the public API. `Racy` is Copy + Send but
// deliberately *not* Sync (raw pointers are !Sync by default and we never override that), so it
// satisfies the bound while still tripping a real data race if the executor ever shares one
// instance's `&self` across concurrently-running real threads instead of giving each thread its
// own copy.
use orx_parallel::*;

#[derive(Clone, Copy)]
struct Racy(*mut i64);
unsafe impl Send for Racy {}
// Deliberately NOT Sync.

impl Racy {
    // Force capture of the whole struct (not just the `.0` field) so Rust 2021's disjoint-field
    // closure capture doesn't silently bypass `unsafe impl Send for Racy` by re-deriving auto
    // traits straight from the raw-pointer field.
    #[inline(always)]
    fn ptr(self) -> *mut i64 {
        self.0
    }
}

fn main() {
    let mut counter: i64 = 0;
    let racy = Racy(&mut counter as *mut i64);
    let n = 200_000usize;

    par_recursive(0..n, |_: &usize| None::<usize>)
        .num_threads(8)
        .map(move |x: usize| {
            unsafe {
                let p = racy.ptr();
                let old = *p;
                std::thread::yield_now(); // widen the race window
                *p = old + 1;
            }
            x
        })
        .for_each(|_| {});

    println!("expected={} actual={}", n, counter);
    if counter != n as i64 {
        println!("RACE DETECTED: lost updates, counter != expected (recursive/XapSync path)");
        std::process::exit(1);
    } else {
        println!("no race observed this run (recursive/XapSync path)");
    }
}
