# triple_accel/src/levenshtein.rs:51:13

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: panic
- location: `triple_accel/src/levenshtein.rs:51:13`
- last API call: `triple_accel::levenshtein::EditCosts::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: (cost >> 1) < gap_cost
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { git = "https://github.com/Daniel-Liu-c0deb0t/triple_accel", rev = "0f2119a6bbc3e6f007bcca0051660408c47ee883" }
```

`src/main.rs`:

```rust
use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = EditCosts::new(100u8, 1u8, 0u8, Some(10u8));
}
```

Run with `cargo run` in this directory (package `repro_triple_accel_src_levenshtein_rs_51_13_2f47fc`).

## Output

```
thread 'main' (1260288) panicked at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:51:13:
assertion failed: (cost >> 1) < gap_cost
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <triple_accel::levenshtein::EditCosts>::new
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:51:13
   4: repro_triple_accel_src_levenshtein_rs_51_13_2f47fc::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
