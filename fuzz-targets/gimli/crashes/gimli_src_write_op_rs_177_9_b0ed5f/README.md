# gimli/src/write/op.rs:177:9

- crate: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c
- kind: panic
- location: `gimli/src/write/op.rs:177:9`
- last API call: `gimli::write::Expression::set_target`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left != right` failed
  left: 0
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gimli = { git = "https://github.com/gimli-rs/gimli", rev = "64a7a194fb5fe43d474fd9c101555e5a81b1d36c" }
```

`src/main.rs`:

```rust
use gimli::write::Expression;

fn main() {
    let mut expression = Expression::raw(vec![]);
    expression.set_target(0usize, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_gimli_src_write_op_rs_177_9_b0ed5f`).

## Output

```
thread 'main' (431221) panicked at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/op.rs:177:9:
assertion `left != right` failed
  left: 0
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <gimli::write::op::Expression>::set_target
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/write/op.rs:177:9
   5: repro_gimli_src_write_op_rs_177_9_b0ed5f::main
             at ./src/main.rs:5:16
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
