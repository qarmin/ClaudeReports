# libc/src/unix/linux_like/mod.rs:1781:13

- crate: https://github.com/rust-lang/libc @ d490042d3390bb5f023a2cfaf85fa8c5606c04dc
- kind: panic
- location: `libc/src/unix/linux_like/mod.rs:1781:13`
- last API call: `libc::_IO`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: ty <= _IOC_TYPEMASK
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
libc = { git = "https://github.com/rust-lang/libc", rev = "d490042d3390bb5f023a2cfaf85fa8c5606c04dc" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = libc::_IO(1000000000u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_libc_src_unix_linux_like_mod_rs_1781_13_4879f2`).

## Output

```
thread 'main' (1434188) panicked at /home/rafal/.cargo/git/checkouts/libc-6cc2a637dbcd9088/d490042/src/unix/linux_like/mod.rs:1781:13:
assertion failed: ty <= _IOC_TYPEMASK
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: libc::unix::linux_like::_IOC
             at /home/rafal/.cargo/git/checkouts/libc-6cc2a637dbcd9088/d490042/src/unix/linux_like/mod.rs:1781:13
   4: libc::unix::linux_like::_IO
             at /home/rafal/.cargo/git/checkouts/libc-6cc2a637dbcd9088/d490042/src/unix/linux_like/mod.rs:1793:13
   5: repro_libc_src_unix_linux_like_mod_rs_1781_13_4879f2::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
