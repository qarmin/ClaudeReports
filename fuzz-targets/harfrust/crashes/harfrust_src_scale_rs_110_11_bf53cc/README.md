# harfrust/src/scale.rs:110:11

- crate: https://github.com/harfbuzz/harfrust @ facb5efa7f301e2a27cadb3a43dfde93bba3f1f2
- kind: panic
- location: `harfrust/src/scale.rs:110:11`
- last API call: `harfrust::Scale::scale_y`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
harfrust = { git = "https://github.com/harfbuzz/harfrust", rev = "facb5efa7f301e2a27cadb3a43dfde93bba3f1f2" }
```

`src/main.rs`:

```rust
use harfrust::Scale;

fn main() {
    let scale = Scale::new(Some((0i32, 1000000000i32)), 1i32);
    let _ = scale.scale_y(-1000000000i32);
}
```

Run with `cargo run` in this directory (package `repro_harfrust_src_scale_rs_110_11_bf53cc`).

## Output

```
thread 'main' (481554) panicked at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/scale.rs:110:11:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <harfrust::scale::Scale>::scale_by_mult
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/scale.rs:110:11
   4: <harfrust::scale::Scale>::scale_y
             at /home/rafal/.cargo/git/checkouts/harfrust-525ce3c4e938b123/facb5ef/harfrust/src/scale.rs:64:9
   5: repro_harfrust_src_scale_rs_110_11_bf53cc::main
             at ./src/main.rs:5:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
