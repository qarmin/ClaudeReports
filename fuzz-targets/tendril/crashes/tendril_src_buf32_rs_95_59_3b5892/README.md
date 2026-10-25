# tendril/src/buf32.rs:95:59

- crate: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45
- kind: panic
- location: `tendril/src/buf32.rs:95:59`
- last API call: `tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>::with_capacity`
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
    let _ = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_tendril_src_buf32_rs_95_59_3b5892`).

## Output

```
thread 'main' (1270841) panicked at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/buf32.rs:95:59:
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
   5: <tendril::buf32::Buf32<tendril::tendril::Header<tendril::tendril::NonAtomic>>>::grow
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/buf32.rs:95:59
   6: <tendril::tendril::Tendril<tendril::fmt::Bytes>>::make_owned_with_capacity
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:1077:13
   7: <tendril::tendril::Tendril<tendril::fmt::Bytes>>::with_capacity
             at /home/rafal/.cargo/git/checkouts/html5ever-cfe40d3508193bd4/7760920/tendril/src/tendril.rs:574:19
   8: repro_tendril_src_buf32_rs_95_59_3b5892::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
