# triple_accel/src/hamming.rs:136:46

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: panic
- location: `triple_accel/src/hamming.rs:136:46`
- last API call: `triple_accel::hamming::hamming_search_naive`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to divide by zero
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
    let _ = triple_accel::hamming::hamming_search_naive(&vec![], &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_triple_accel_src_hamming_rs_136_46_abc6f7`).

## Output

```
thread 'main' (362643) panicked at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/hamming.rs:136:46:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: triple_accel::hamming::hamming_search_naive_with_opts
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/hamming.rs:136:46
   4: triple_accel::hamming::hamming_search_naive
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/hamming.rs:71:5
   5: repro_triple_accel_src_hamming_rs_136_46_abc6f7::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
