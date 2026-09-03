// Real end-to-end trigger: this is a plain compile-time type error inside the orx-parallel
// library's own source (src/infallible/par_runner.rs), gated behind the `experimental` Cargo
// feature. There is no runtime input or second participant involved - simply depending on
// orx-parallel with `features = ["experimental"]` enabled and building it is the genuine trigger,
// exactly as any real downstream crate turning on that feature would hit.
//
// Exit code convention: 0 = did not reproduce (the dependency built fine - the goal after a fix).
// This binary never actually runs when the bug is present, because `cargo build`/`cargo run`
// fails to even compile the `orx-parallel` dependency first (E0308 in its own source).

use orx_parallel::*;

fn main() {
    let sum: usize = (0..100).par().map(|x| x * 2).sum();
    println!("did not reproduce - orx-parallel built fine with `experimental` enabled, sum = {sum}");
}
