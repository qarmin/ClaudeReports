// Real, end-to-end reproduction of orx-parallel's par_experimental_sort panic-safety hole.
//
// A `T: Ord` implementation that panics mid-comparison is a perfectly legal, user-supplied Ord
// impl - the crate's public API accepts any `T: Ord + Send`, it never promises the comparator
// can't panic. This repro shows that when it does panic partway through a merge pass, it
// corrupts the CALLER's own slice on unwind: some elements end up duplicated at two addresses,
// others vanish entirely - because the internal merge implementation moves elements between the
// caller's slice and a scratch buffer via raw `copy_nonoverlapping`, with no panic-safety guard
// around the `T::lt`/`T::le` comparisons driving those copies.
//
// This calls the real, public `par_experimental_sort` entry point end-to-end with no fakery.
// The "second participant" here is the comparator; a comparator that panics after N calls is
// the standard technique for panic-safety testing (the same one the Rust standard library's own
// sort test suite uses) - there is no separate caller/peer being impersonated, so
// CONFIRMATION.md section 11 does not apply here (this is a single-process, single-entry-point
// repro, the same shape as handing a malformed file to a parser's own public `from_reader`).
//
// Expected outcome when run against the pristine (buggy) tree: the process aborts with a real
// double-free reported by the system allocator (glibc "double free or corruption" / "corrupted
// size vs. prev_size", SIGABRT, exit code 134) - because the duplicated id's two physical copies
// in the corrupted slice both own a bitwise copy of the very same `Box` pointer, and dropping the
// slice for real (which this repro does deliberately, after printing its diagnostics) drops that
// allocation twice.
//
// Expected outcome against the patched tree: the comparator still panics (that part is
// unavoidable and correct - a user's comparator is allowed to panic), but the caller's slice is
// left as a valid permutation of the original elements, so it drops cleanly with no crash.

use orx_parallel::{Params, Runner, par_experimental_sort};
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static ARMED: AtomicBool = AtomicBool::new(false);

// Empirically found (see ../RESULT.md) to land inside an odd merge pass (dst == the caller's own
// slice) for this exact n/seed/thread-count combination on the machine this was investigated on -
// comfortably inside that pass's window, not near a phase boundary, so it reproduces on every run
// regardless of exact thread scheduling. Overridable via env vars to try other points in the same
// bug class without editing/rebuilding.
fn threshold() -> usize {
    std::env::var("REPRO_THRESHOLD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_731_348)
}
fn n_elems() -> usize {
    std::env::var("REPRO_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(200_000)
}

struct Elem {
    key: i32,
    id: usize,
    // A real heap allocation: if the same logical element ends up duplicated at two addresses in
    // the caller's slice (both holding a bitwise copy of the same `Box` pointer), dropping the
    // slice for real drops the same allocation twice - a genuine double-free, not just a
    // bookkeeping mismatch.
    _payload: Box<u8>,
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Elem {}
impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, other: &Self) -> bool {
        maybe_panic();
        self.key < other.key
    }
    fn le(&self, other: &Self) -> bool {
        maybe_panic();
        self.key <= other.key
    }
}
impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        maybe_panic();
        self.key.cmp(&other.key)
    }
}

#[inline]
fn maybe_panic() {
    if ARMED.load(Ordering::Relaxed) {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        if n == threshold() {
            panic!("planted panic in user Ord impl at comparison {n}");
        }
    }
}

fn make_data(n: usize) -> Vec<Elem> {
    // xorshift64: deterministic, fixed seed, no external randomness needed.
    let mut state: u64 = 0x9E3779B97F4A7C15;
    (0..n)
        .map(|id| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let key = (state as i32).wrapping_mul(2654435761u32 as i32);
            Elem {
                key,
                id,
                _payload: Box::new((id % 256) as u8),
            }
        })
        .collect()
}

/// Returns (duplicated_id_count, missing_id_count) - both zero iff `data` is still exactly a
/// permutation of ids 0..n (i.e. the original element multiset, with none doubled or dropped).
fn check_ids(data: &[Elem], n: usize) -> (usize, usize) {
    let mut seen = vec![0u32; n];
    for e in data {
        if e.id < n {
            seen[e.id] += 1;
        }
    }
    let duplicated = seen.iter().filter(|&&c| c > 1).count();
    let missing = seen.iter().filter(|&&c| c == 0).count();
    (duplicated, missing)
}

fn main() {
    // Silence the default panic hook's own backtrace noise; we print our own diagnostics.
    std::panic::set_hook(Box::new(|info| {
        println!("(panic hook) {info}");
    }));

    let n = n_elems();
    let mut data = make_data(n);
    let mut runner = Runner::fixed();

    ARMED.store(true, Ordering::Relaxed);
    COUNTER.store(0, Ordering::Relaxed);
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        par_experimental_sort(&mut data, &mut runner, Params::default());
    }));
    ARMED.store(false, Ordering::Relaxed);

    match result {
        Ok(()) => {
            println!("RESULT: no panic occurred (sort completed normally) - not the bug scenario");
            std::process::exit(2);
        }
        Err(_) => {
            let (duplicated, missing) = check_ids(&data, n);
            println!(
                "RESULT: sort panicked mid-merge; caller's slice now has {duplicated} duplicated id(s) and {missing} missing id(s) out of {n}"
            );
            if duplicated == 0 && missing == 0 {
                println!("RESULT: slice is still a valid permutation - bug NOT reproduced this run");
                std::process::exit(3);
            }
            println!(
                "RESULT: BUG REPRODUCED - the caller's slice is corrupted (not a valid permutation of the original elements)"
            );
            println!(
                "Now dropping the corrupted slice for real: a duplicated id means two Elem \
                 instances at two different addresses hold a bitwise copy of the same Box \
                 pointer - dropping both frees the same heap allocation twice."
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();
            drop(data); // expected to abort the process with a real double-free from the allocator
            println!("RESULT: UNEXPECTED - process did not abort on drop");
            std::process::exit(4);
        }
    }
}
