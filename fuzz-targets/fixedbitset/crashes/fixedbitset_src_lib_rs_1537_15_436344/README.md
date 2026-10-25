# fixedbitset/src/lib.rs:1537:15

- crate: https://github.com/petgraph/fixedbitset @ 111fb2552130f6f4c06a4a1a2e0de7ea6e28b922
- kind: panic
- location: `fixedbitset/src/lib.rs:1537:15`
- last API call: `fixedbitset::IntoOnes::last_positive_bit_and_unset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fixedbitset = { git = "https://github.com/petgraph/fixedbitset", rev = "111fb2552130f6f4c06a4a1a2e0de7ea6e28b922" }
```

`src/main.rs`:

```rust
use fixedbitset::IntoOnes;

fn main() {
    let _ = IntoOnes::last_positive_bit_and_unset(&mut 0usize);
}
```

Run with `cargo run` in this directory (package `repro_fixedbitset_src_lib_rs_1537_15_436344`).

## Output

```
thread 'main' (1272372) panicked at /home/rafal/.cargo/git/checkouts/fixedbitset-8ed885828547678f/111fb25/src/lib.rs:1537:15:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <fixedbitset::IntoOnes>::last_positive_bit_and_unset
             at /home/rafal/.cargo/git/checkouts/fixedbitset-8ed885828547678f/111fb25/src/lib.rs:1537:15
   4: repro_fixedbitset_src_lib_rs_1537_15_436344::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
