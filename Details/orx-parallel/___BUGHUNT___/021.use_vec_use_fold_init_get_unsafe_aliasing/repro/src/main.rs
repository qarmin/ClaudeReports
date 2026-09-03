// Real end-to-end trigger: this is a single external caller (any downstream crate) using only
// orx-parallel's real public API (`orx_parallel::{Use, UseVec}`, both re-exported from the crate
// root) exactly as a real user would import and call them. Nothing here is a hand-authored stand-in
// for a second participant - there is only one participant (this program), calling the crate's own
// public, safe-looking `Use::init_get` method twice, in plain safe Rust, with no `unsafe` block
// anywhere in this file.
use orx_parallel::{Use, UseVec};

fn main() {
    let uv = UseVec::new(|_| 0i32);

    // Both calls pass thread_idx = 0. `Use::init_get` is declared as a *safe* fn
    // (`fn init_get(&self, thread_idx: usize) -> &mut Self::Item`) on the crate's public,
    // exported `Use` trait, and `UseVec` implements it by taking only `&self` - so any
    // downstream crate can call it as many times as it likes for the same thread_idx, with no
    // `unsafe` keyword required anywhere in the caller's own code.
    let r1: &mut i32 = uv.init_get(0);
    let r2: &mut i32 = uv.init_get(0);

    *r1 = 111;
    *r2 = 222;

    println!("r1 = {r1}, r2 = {r2}");

    if *r1 == *r2 {
        println!("REPRODUCED: r1 and r2 alias the same memory (both {r1}), from 100% safe code.");
        std::process::exit(1);
    } else {
        println!("did not reproduce: r1 ({r1}) and r2 ({r2}) are distinct");
    }
}
