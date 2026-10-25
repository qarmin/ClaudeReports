# difflib/src/differ.rs:294:13

- crate: https://github.com/DimaKudosh/difflib @ f035fb8e656f27119e23eca9d5b996df14f3885e
- kind: panic
- location: `difflib/src/differ.rs:294:13`
- last API call: `difflib::differ::Differ::restore`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Second parameter must be 1 or 2
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
difflib = { git = "https://github.com/DimaKudosh/difflib", rev = "f035fb8e656f27119e23eca9d5b996df14f3885e" }
```

`src/main.rs`:

```rust
use difflib::differ::Differ;

fn main() {
    let _ = Differ::restore(&[], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_difflib_src_differ_rs_294_13_914803`).

## Output

```
thread 'main' (594254) panicked at /home/rafal/.cargo/git/checkouts/difflib-ef03ab080519207e/f035fb8/src/differ.rs:294:13:
Second parameter must be 1 or 2
stack backtrace:
   0: std::panicking::begin_panic::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:751:5
   1: <difflib::differ::Differ>::restore
             at /home/rafal/.cargo/git/checkouts/difflib-ef03ab080519207e/f035fb8/src/differ.rs:294:13
   2: repro_difflib_src_differ_rs_294_13_914803::main
             at ./src/main.rs:4:13
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
