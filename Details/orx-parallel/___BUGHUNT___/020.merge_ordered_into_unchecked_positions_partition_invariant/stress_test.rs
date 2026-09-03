// Stress test for candidate 020: does merge_ord_into_vec's implicit invariant
// (each ValsAndIdx.positions exactly and non-overlappingly partitions its own
// values buffer) ever get violated through the real, public collect() path?
//
// Strategy: flat_map each input index to a variable number of output elements
// (0, 1, 2, or 3) so per-chunk lengths vary, across many thread-count/chunk-size/
// input-size combinations, then compare the parallel result against a sequential
// reference computed the same way. Any invariant violation in the merge would
// either panic (bounds-checked debug build), be caught by miri as UB, or silently
// produce a wrong/garbled result vector that fails the equality assertion below.

use orx_parallel::*;

fn expand(x: usize) -> std::vec::Vec<usize> {
    match x % 4 {
        0 => std::vec::Vec::new(),
        1 => std::vec![x],
        2 => std::vec![x, x + 1000],
        _ => std::vec![x, x + 1000, x + 2000],
    }
}

#[test]
fn merge_ord_into_vec_stress_infallible() {
    for n in [0usize, 1, 2, 3, 5, 7, 8, 13, 16, 31, 32, 33, 64, 100, 257] {
        for num_threads in [1usize, 2, 3, 4, 8] {
            for chunk_size in [1usize, 2, 3, 5, 8] {
                let expected: std::vec::Vec<usize> =
                    (0..n).flat_map(expand).collect();

                let actual: std::vec::Vec<usize> = (0..n)
                    .par()
                    .num_threads(num_threads)
                    .chunk_size(chunk_size)
                    .flat_map(expand)
                    .collect();

                assert_eq!(
                    actual, expected,
                    "n={n} num_threads={num_threads} chunk_size={chunk_size}"
                );
            }
        }
    }
}

#[test]
fn merge_ord_into_vec_stress_option() {
    // Every element present (no None short-circuit) - still exercises extend_opt's
    // bookkeeping and the merge over multiple chunks/threads with variable lengths.
    for n in [0usize, 1, 4, 9, 30, 65, 130] {
        for num_threads in [1usize, 3, 5] {
            for chunk_size in [1usize, 3, 7] {
                let expected: std::vec::Vec<usize> = (0..n).flat_map(expand).collect();

                let actual: Option<std::vec::Vec<usize>> = (0..n)
                    .par()
                    .map(|x| expand(x))
                    .map(Some)
                    .into_optional()
                    .num_threads(num_threads)
                    .chunk_size(chunk_size)
                    .flat_map(|v| v.into_iter())
                    .collect();

                assert_eq!(
                    actual,
                    Some(expected),
                    "n={n} num_threads={num_threads} chunk_size={chunk_size}"
                );
            }
        }
    }
}
