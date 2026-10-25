# triple_accel/src/levenshtein.rs:69:13

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: panic
- location: `triple_accel/src/levenshtein.rs:69:13`
- last API call: `triple_accel::levenshtein::levenshtein_search_simd_with_opts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: cost <= self.start_gap_cost + self.gap_cost
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
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&vec![66u8], &vec![], 0u32, SearchType::Best, EditCosts::new(100u8, 63u8, 1u8, Some(100u8)), false);
}
```

Run with `cargo run` in this directory (package `repro_triple_accel_src_levenshtein_rs_69_13_25f6f7`).

## Output

```
thread 'main' (1260701) panicked at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:69:13:
assertion failed: cost <= self.start_gap_cost + self.gap_cost
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <triple_accel::levenshtein::EditCosts>::check_search
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:69:13
   4: triple_accel::levenshtein::levenshtein_search_simd_with_opts
             at /home/rafal/.cargo/git/checkouts/triple_accel-1b19cbcdace966e9/0f2119a/src/levenshtein.rs:1965:11
   5: repro_triple_accel_src_levenshtein_rs_69_13_25f6f7::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
