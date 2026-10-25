# toml_edit-0.25.17+spec/src/array.rs:306:35

- crate: toml 1.1.8+spec-1.1.0
- kind: panic
- location: `toml_edit-0.25.17+spec/src/array.rs:306:35`
- last API call: `toml_edit::Array::remove`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
removal index (is 0) should be < len (is 0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
toml = { version = "=1.1.8+spec-1.1.0" }
toml_edit = { version = "=0.25.17+spec-1.1.0" }
```

`src/main.rs`:

```rust
use toml_edit::Array;

fn main() {
    let _ = Array::default().remove(0usize);
}
```

Run with `cargo run` in this directory (package `repro_toml_edit_0_25_17_spec_src_array_rs_306_35_9dfd9d`).

## Output

```
thread 'main' (375287) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_edit-0.25.17+spec-1.1.0/src/array.rs:306:35:
removal index (is 0) should be < len (is 0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <alloc::vec::Vec<_, _>>::remove::assert_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/vec/mod.rs:2403:13
   3: <alloc::vec::Vec<toml_edit::item::Item>>::remove
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2408:21
   4: <toml_edit::array::Array>::remove
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_edit-0.25.17+spec-1.1.0/src/array.rs:306:35
   5: repro_toml_edit_0_25_17_spec_src_array_rs_306_35_9dfd9d::main
             at ./src/main.rs:4:30
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
