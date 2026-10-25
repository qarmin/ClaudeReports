# triple_accel/src/levenshtein.rs:1421:10

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: panic
- location: `triple_accel/src/levenshtein.rs:1421:10`
- last API call: `triple_accel::rdamerau`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { git = "https://github.com/Daniel-Liu-c0deb0t/triple_accel", rev = "0f2119a6bbc3e6f007bcca0051660408c47ee883" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = triple_accel::rdamerau(&vec![], &include_bytes!("data_0.bin").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_triple_accel_src_levenshtein_rs_1421_10_f2133d`).

## Output

```
thread 'main' (1260758) panicked at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:1421:10:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<(u32, core::option::Option<alloc::vec::Vec<triple_accel::Edit>>)>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: triple_accel::levenshtein::rdamerau
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:1421:10
   6: repro_triple_accel_src_levenshtein_rs_1421_10_f2133d::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
