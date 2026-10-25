# out of memory in kurbo::expand_path_signed[Circle]

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: out of memory
- last API call: `kurbo::expand_path_signed[Circle]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3518652268828921012

## Message

```
process ended abnormally (out of memory) while calling kurbo::expand_path_signed[Circle]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Triangle;

fn main() {
    let _ = kurbo::expand_path_signed(Triangle::EQUILATERAL.circumscribed_circle(), <kurbo::Diagonal2 as ::core::ops::Neg>::neg(Diagonal2 { xx: 1.0f64, yy: -1.2817015970884436e126f64 }), Join::Round, 0.0f64, 1.0f64);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_kurbo_expand_path_signed_circle_94a80d`).

## Output

```
memory allocation of 3758096384 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::grow_one
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:494:13
   6: <alloc::raw_vec::RawVec<kurbo::bezpath::PathEl>>::grow_one
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:188:29
   7: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::push_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1033:22
   8: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::push
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:996:22
   9: <kurbo::bezpath::BezPath>::push
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:222:16
  10: <kurbo::bezpath::BezPath>::curve_to::<kurbo::point::Point>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/bezpath.rs:267:14
  11: <kurbo::expand::ExpandCtx>::do_join::{closure#0}
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/expand.rs:338:49
  12: <kurbo::arc::Arc>::to_cubic_beziers::<<kurbo::expand::ExpandCtx>::do_join::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/arc.rs:114:13
  13: <kurbo::expand::ExpandCtx>::do_join
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/expand.rs:337:37
  14: <kurbo::expand::ExpandCtx>::do_line
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/expand.rs:198:14
  15: <kurbo::expand::ExpandCtx>::do_cubic
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/expand.rs:279:18
  16: kurbo::expand::expand_path_signed::<kurbo::circle::Circle>
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/expand.rs:176:48
  17: repro_out_of_memory_in_kurbo_expand_path_signed_circle_94a80d::main
             at ./src/main.rs:6:13
  18: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
