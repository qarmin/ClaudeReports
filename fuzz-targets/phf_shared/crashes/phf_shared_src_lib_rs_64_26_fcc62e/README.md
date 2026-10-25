# phf_shared/src/lib.rs:64:26

- crate: https://github.com/rust-phf/rust-phf @ c15adb08f43f3dadbb725b04dbf7f88ebec7d1de
- kind: panic
- location: `phf_shared/src/lib.rs:64:26`
- last API call: `phf_shared::get_index`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to calculate the remainder with a divisor of zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
phf_shared = { git = "https://github.com/rust-phf/rust-phf", rev = "c15adb08f43f3dadbb725b04dbf7f88ebec7d1de" }
```

`src/main.rs`:

```rust
fn main() {
    let hashes = phf_shared::hash::<u16>(&0u16, &0u64);
    let _ = phf_shared::get_index(&hashes, &[], 1usize);
}
```

Run with `cargo run` in this directory (package `repro_phf_shared_src_lib_rs_64_26_fcc62e`).

## Output

```
thread 'main' (491228) panicked at /home/rafal/.cargo/git/checkouts/rust-phf-ee73cbe656439081/c15adb0/phf_shared/src/lib.rs:64:26:
attempt to calculate the remainder with a divisor of zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_rem_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: phf_shared::get_index
             at /home/rafal/.cargo/git/checkouts/rust-phf-ee73cbe656439081/c15adb0/phf_shared/src/lib.rs:64:26
   4: repro_phf_shared_src_lib_rs_64_26_fcc62e::main
             at ./src/main.rs:3:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
