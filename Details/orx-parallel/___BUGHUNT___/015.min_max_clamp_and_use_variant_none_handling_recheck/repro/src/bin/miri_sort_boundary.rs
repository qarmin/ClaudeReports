// Miri escalation (approach 3): a much smaller boundary sweep of the same
// `par_experimental_sort` real public entry point, sized to actually finish under Miri's
// interpreter overhead, to catch any Stacked/Tree-Borrows violation in the unsafe raw-pointer
// merge (`MergeSubTask`/`CopyChunkTask`) that a plain debug-assertions build wouldn't detect.
use orx_parallel::*;

fn main() {
    for len in 0usize..=20 {
        for num_threads in [2usize, 3, 4] {
            let mut data: Vec<i64> = (0..len as i64).rev().collect();
            let mut expected = data.clone();
            expected.sort_unstable();

            let mut runner = Runner::fixed();
            let params = Params {
                num_threads: num_threads.into(),
                ..Default::default()
            };
            par_experimental_sort(&mut data, &mut runner, params);
            assert_eq!(data, expected, "len={len} num_threads={num_threads}");
        }
    }
    println!("OK under miri: len=0..=20, num_threads in [2,3,4]");
}
