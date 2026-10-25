# tendril/src/tendril.rs:596:60

- crate: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45
- kind: panic
- location: `tendril/src/tendril.rs:596:60`
- last API call: `tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic>::reserve`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
tendril: overflow in buffer arithmetic
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tendril = { git = "https://github.com/servo/html5ever", rev = "7760920edff08e6dfd4b62affee39d8084e9dc45" }
```

`src/main.rs`:

```rust
fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::from_char('a');
    let mut tendril2 = tendril.try_reinterpret::<tendril::fmt::WTF8>().unwrap();
    tendril2.reserve(u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_tendril_src_tendril_rs_596_60_ddf3c0`).

## Output

```
thread 'main' (1270927) panicked at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:596:60:
tendril: overflow in buffer arithmetic
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<u32>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <tendril::tendril::Tendril<tendril::fmt::WTF8>>::force_reserve
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:596:60
   6: <tendril::tendril::Tendril<tendril::fmt::WTF8>>::reserve
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:589:18
   7: repro_tendril_src_tendril_rs_596_60_ddf3c0::main
             at ./src/main.rs:4:14
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
