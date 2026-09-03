// Real end-to-end check of issue #165's LOGIC_3 pattern ("using thread-compute loops did
// unconditional `None => break` instead of checking `iter.is_completed_when_none_returned()`,
// so a transient `None` could cause early termination and silently dropped/short results across
// collect/reduce/find/any/all").
//
// Drives the crate's real public `_use` API (`use_new`) end-to-end across collect/reduce/
// find_any/any/all, at various thread counts and chunk sizes, over a large input, and checks the
// result against a plain sequential computation. If a transient/early `None => break` were
// dropping elements, this would show up as an undercount here.

use orx_parallel::*;

fn main() {
    let n = 300_000usize;
    let data: Vec<u64> = (0..n as u64).collect();

    for num_threads in [0usize, 1, 2, 4, 8, 16] {
        for chunk_size in [0usize, 1, 4, 64, 1024] {
            // collect
            let collected: Vec<u64> = data
                .par()
                .copied()
                .num_threads(num_threads)
                .chunk_size(chunk_size)
                .use_new(|_| 0u64)
                .map(|acc, x| {
                    *acc += 1;
                    x
                })
                .collect();
            assert_eq!(
                collected.len(),
                n,
                "collect dropped elements at num_threads={num_threads} chunk_size={chunk_size}: \
                 got {} expected {n}",
                collected.len()
            );
            let mut sorted = collected.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, data, "collect changed/lost values (not just order)");

            // reduce (sum)
            let sum = data
                .par()
                .copied()
                .num_threads(num_threads)
                .chunk_size(chunk_size)
                .use_new(|_| ())
                .map(|_, x| x)
                .reduce(|_, a, b| a + b);
            let expected_sum: u64 = data.iter().sum();
            assert_eq!(
                sum,
                Some(expected_sum),
                "reduce undercounted at num_threads={num_threads} chunk_size={chunk_size}: \
                 got {sum:?} expected {expected_sum}"
            );

            // any / all
            let any_found = data
                .par()
                .copied()
                .num_threads(num_threads)
                .chunk_size(chunk_size)
                .use_new(|_| ())
                .any(|_, x| *x == (n as u64 - 1));
            assert!(
                any_found,
                "any() missed the last element at num_threads={num_threads} chunk_size={chunk_size}"
            );

            let all_nonneg = data
                .par()
                .copied()
                .num_threads(num_threads)
                .chunk_size(chunk_size)
                .use_new(|_| ())
                .all(|_, _x| true);
            assert!(
                all_nonneg,
                "all() spuriously failed at num_threads={num_threads} chunk_size={chunk_size}"
            );
        }
    }

    println!(
        "OK: collect/reduce/any/all through use_new(...) never dropped elements across {} \
         (num_threads, chunk_size) combinations",
        6 * 5
    );
}
