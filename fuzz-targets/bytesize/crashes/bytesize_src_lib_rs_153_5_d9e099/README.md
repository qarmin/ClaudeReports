# bytesize/src/lib.rs:153:5

- crate: https://github.com/bytesize-rs/bytesize @ 66a3715e33369e99cb428bcd5aee32291f8e0a94
- kind: panic
- location: `bytesize/src/lib.rs:153:5`
- last API call: `bytesize::pib`
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
    let _ = bytesize::pib::<u64>(10000000000000000000u64);
}
```

Run with `cargo run` in this directory (package `repro_bytesize_src_lib_rs_153_5_d9e099`).

## Output

```
thread 'main' (352603) panicked at /home/rafal/.cargo/git/checkouts/bytesize-973f88ad5d6c3593/66a3715/src/lib.rs:153:5:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: bytesize::pib::<u64>
             at /home/rafal/.cargo/git/checkouts/bytesize-973f88ad5d6c3593/66a3715/src/lib.rs:153:5
   4: repro_bytesize_src_lib_rs_153_5_d9e099::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
