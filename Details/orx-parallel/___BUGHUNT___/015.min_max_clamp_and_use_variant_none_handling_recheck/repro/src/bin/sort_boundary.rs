// Real end-to-end check of issue #165's LOGIC_5/UAF_1 pattern (a `min_split_len`-style clamp in
// a parallel merge routine using `.min(.., 3)` instead of `.max(.., 3)`, letting a tiny
// caller-supplied length flow through to an unsafe `first_unchecked()`-style call on a possibly
// empty/too-short slice). Exercises the crate's real public `par_experimental_sort` entry point
// (the only parallel merge/split routine with `unsafe` currently reachable through the public
// API) at every small boundary length, across many thread counts, under a debug-assertions +
// overflow-checks build so any OOB/UB-adjacent slip is very likely to panic loudly.

use orx_parallel::*;

fn run_one(len: usize, num_threads: usize) {
    // reverse-sorted: worst case for a merge-based sort (every element crosses the split)
    let mut data: Vec<i64> = (0..len as i64).rev().collect();
    let mut expected = data.clone();
    expected.sort_unstable();

    let mut runner = Runner::fixed();
    let params = Params {
        num_threads: num_threads.into(),
        ..Default::default()
    };
    par_experimental_sort(&mut data, &mut runner, params);

    assert_eq!(
        data, expected,
        "par_experimental_sort produced wrong output at len={len} num_threads={num_threads}"
    );
}

fn main() {
    // Small lengths: only the `n < 1024` sequential-fallback path in the current
    // implementation (see src/sort/sort_slice.rs) - still worth checking nothing panics.
    for len in 0usize..=64 {
        for num_threads in [1usize, 2, 3, 4, 8, 16] {
            run_one(len, num_threads);
        }
    }

    // Straddling the current `n < 1024` threshold from both sides: this is where the actual
    // parallel merge (`MergeSubTask`/`find_split`) gets exercised for the first time as `len`
    // crosses it, the real analogue of the historical `min_split_len`-clamp bug's trigger zone.
    for len in 1000usize..=1100 {
        for num_threads in [2usize, 3, 4, 8, 16] {
            run_one(len, num_threads);
        }
    }

    // A bit larger, to exercise multiple levels of the recursive merge/split.
    for len in [2048usize, 4096, 10_000, 50_000] {
        for num_threads in [2usize, 4, 8, 16] {
            run_one(len, num_threads);
        }
    }

    println!(
        "OK: par_experimental_sort correct for len in 0..=64, 1000..=1100, and \
         {{2048,4096,10000,50000}}, num_threads in [1,2,3,4,8,16], no panics/UB observed"
    );
}
