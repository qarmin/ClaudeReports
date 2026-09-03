// Isolated, single-threaded reproduction of the *exact* code shape used by
// `TaskCounts::push` in src/runner/runner_variants/with_diagnostics.rs (verbatim struct/method
// body copied here, since `TaskCounts` itself is a private implementation detail of the crate,
// not part of its public API - this file exists purely to check, in isolation and without any
// threading, whether Miri's aliasing checkers can ever flag this raw-pointer-through-`&self`
// pattern at all, independent of the crate's own orchestration/threading logic exercised by
// `main.rs` in this same package). See RESULT.md ("What was tried", approach 3) for why this
// isolation step was run in addition to the real end-to-end multithreaded repro in `main.rs`.
struct TaskCounts(Vec<Vec<usize>>);

impl TaskCounts {
    fn push(&self, th_idx: usize, chunk_size: usize) {
        let vec = unsafe { &mut *(self.0.as_ptr().add(th_idx) as *mut Vec<usize>) };
        vec.push(chunk_size);
    }
}

fn main() {
    let tc = TaskCounts((0..2).map(|_| Vec::new()).collect());
    let shared: &TaskCounts = &tc;

    // Two reborrows of `&mut Vec<usize>` at the *same* index, both derived from the same shared
    // `&TaskCounts`, sequentially in one thread - the simplest possible instance of the pattern.
    shared.push(0, 1);
    shared.push(0, 2);
    shared.push(1, 3);

    println!("{:?}", tc.0);
}
