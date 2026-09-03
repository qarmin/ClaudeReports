// This is a compile-time bug: enabling `wasm` + `persistent-pool-rayon` on a native (non-wasm32)
// target makes orx-parallel itself fail to build with E0425 "cannot find value
// `build_default_rayon_thread_pool`". There is nothing adversarial to feed at runtime - the whole
// point is that `cargo build`/`cargo run` for this crate does not even reach `main` before the
// fix is applied. Exit code convention (same as SKILL.md's repro template): 0 = did not reproduce
// (built and ran fine), the process never gets to run at all (a `cargo run` build failure) = the
// bug reproduced.
use orx_parallel::ParThreadPool;
use orx_parallel::pool::get_global_pool;

fn main() {
    let pool = get_global_pool();
    println!(
        "did not reproduce - crate compiled and get_global_pool() returned a pool with {} threads",
        pool.max_num_threads()
    );
}
