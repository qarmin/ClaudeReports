// Supplementary evidence (not part of the XapSync claim itself): the exact same
// Copy + Send-but-!Sync closure raced through the crate's *other*, non-recursive execution path
// ((0..n).into_par().for_each(...)), which never touches `XapSync` at all and already gives each
// real worker thread its own bitwise copy of the closure (no shared `&self` anywhere). It races
// identically to `main.rs`'s recursive/XapSync path -- see RESULT.md for what this implies about
// XapSync's own contribution to the observable symptom.
use orx_parallel::*;

#[derive(Clone, Copy)]
struct Racy(*mut i64);
unsafe impl Send for Racy {}

impl Racy {
    #[inline(always)]
    fn ptr(self) -> *mut i64 {
        self.0
    }
}

fn main() {
    let mut counter: i64 = 0;
    let racy = Racy(&mut counter as *mut i64);
    let n = 200_000usize;

    (0..n)
        .into_par()
        .num_threads(8)
        .for_each(move |_x: usize| unsafe {
            let p = racy.ptr();
            let old = *p;
            std::thread::yield_now();
            *p = old + 1;
        });

    println!("expected={} actual={}", n, counter);
    if counter != n as i64 {
        println!("RACE DETECTED (direct, non-recursive, non-XapSync path): lost updates");
        std::process::exit(1);
    } else {
        println!("no race observed this run (direct path)");
    }
}
