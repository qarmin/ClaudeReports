# nalgebra/src/base/dimension.rs:115:14

- crate: https://github.com/dimforge/nalgebra @ b2466e6c4070b06240d929895473c9141fe24720
- kind: panic
- location: `nalgebra/src/base/dimension.rs:115:14`
- last API call: `nalgebra::base::dimension::Dyn::add<usize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
nalgebra = { git = "https://github.com/dimforge/nalgebra", rev = "b2466e6c4070b06240d929895473c9141fe24720" }
```

`src/main.rs`:

```rust
use nalgebra::base::dimension::Dyn;

fn main() {
    let _ = <nalgebra::base::dimension::Dyn as ::core::ops::Add<usize>>::add(Dyn(1usize), usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_nalgebra_src_base_dimension_rs_115_14_41e789`).

## Output

```
thread 'main' (566660) panicked at /home/rafal/.cargo/git/checkouts/nalgebra-7064e04a3e04dd80/b2466e6/src/base/dimension.rs:115:14:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <nalgebra::base::dimension::Dyn as core::ops::arith::Add<usize>>::add
             at /home/rafal/.cargo/git/checkouts/nalgebra-7064e04a3e04dd80/b2466e6/src/base/dimension.rs:115:14
   4: repro_nalgebra_src_base_dimension_rs_115_14_41e789::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
