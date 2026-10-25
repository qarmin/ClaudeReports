# emath/src/lib.rs:227:5

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `emath/src/lib.rs:227:5`
- last API call: `emath::format_with_decimals_in_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1148560359936472584

## Message

```
max_decimals should be < 100, but got 10000000000000000000
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = emath::format_with_decimals_in_range(0.0f64, 0usize..=10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_emath_src_lib_rs_227_5_ed1dd6`).

## Output

```
thread 'main' (173099) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:227:5:
max_decimals should be < 100, but got 10000000000000000000
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: emath::format_with_decimals_in_range
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/emath/src/lib.rs:227:5
   3: repro_emath_src_lib_rs_227_5_ed1dd6::main
             at ./src/main.rs:2:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
