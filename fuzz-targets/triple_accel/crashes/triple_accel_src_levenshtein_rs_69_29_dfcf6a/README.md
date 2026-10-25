# triple_accel/src/levenshtein.rs:69:29

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: panic
- location: `triple_accel/src/levenshtein.rs:69:29`
- last API call: `triple_accel::levenshtein::levenshtein_search_simd_with_opts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { git = "https://github.com/Daniel-Liu-c0deb0t/triple_accel", rev = "0f2119a6bbc3e6f007bcca0051660408c47ee883" }
```

`src/main.rs`:

```rust
use triple_accel::SearchType;
use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&vec![82u8], &vec![], 0u32, SearchType::Best, EditCosts::new(1u8, 1u8, 255u8, Some(1u8)), false);
}
```

Run with `cargo run` in this directory (package `repro_triple_accel_src_levenshtein_rs_69_29_dfcf6a`).

## Output

```
thread 'main' (1260458) panicked at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:69:29:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <triple_accel::levenshtein::EditCosts>::check_search
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:69:29
   4: triple_accel::levenshtein::levenshtein_search_simd_with_opts
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:1965:11
   5: repro_triple_accel_src_levenshtein_rs_69_29_dfcf6a::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
