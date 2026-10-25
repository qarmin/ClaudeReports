# glow/src/lib.rs:1995:14

- crate: https://github.com/grovesNL/glow @ 62250dc64e7e531c280beea251f39aa49ba0b2f1
- kind: panic
- location: `glow/src/lib.rs:1995:14`
- last API call: `glow::bytes_per_type`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsupported pixel type: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
glow = { git = "https://github.com/grovesNL/glow", rev = "62250dc64e7e531c280beea251f39aa49ba0b2f1" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = glow::bytes_per_type(0u32);
}
```

Run with `cargo run` in this directory (package `repro_glow_src_lib_rs_1995_14_0ff307`).

## Output

```
thread 'main' (520264) panicked at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:1995:14:
unsupported pixel type: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: glow::bytes_per_type
             at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:1995:14
   3: repro_glow_src_lib_rs_1995_14_0ff307::main
             at ./src/main.rs:2:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
