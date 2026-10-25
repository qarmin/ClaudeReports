# bytesize/src/lib.rs:133:5

- crate: https://github.com/bytesize-rs/bytesize @ 66a3715e33369e99cb428bcd5aee32291f8e0a94
- kind: panic
- location: `bytesize/src/lib.rs:133:5`
- last API call: `bytesize::gib`
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
bytesize = { git = "https://github.com/bytesize-rs/bytesize", rev = "66a3715e33369e99cb428bcd5aee32291f8e0a94" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = bytesize::gib::<u64>(u64::MAX);
}
```

Run with `cargo run` in this directory (package `repro_bytesize_src_lib_rs_133_5_a404ac`).

## Output

```
thread 'main' (352213) panicked at /home/rafal/.cargo/git/checkouts/bytesize-973f88ad5d6c3593/66a3715/src/lib.rs:133:5:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bytesize::gib::<u64>
             at /home/rafal/.cargo/git/checkouts/bytesize-973f88ad5d6c3593/66a3715/src/lib.rs:133:5
   4: repro_bytesize_src_lib_rs_133_5_a404ac::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
