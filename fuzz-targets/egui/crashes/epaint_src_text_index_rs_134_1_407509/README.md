# epaint/src/text/index.rs:134:1

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/index.rs:134:1`
- last API call: `epaint::text::CharIndex::sub<usize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4274873430003401310

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::text::CharIndex;

fn main() {
    let _ = <epaint::text::CharIndex as ::core::ops::Sub<usize>>::sub(CharIndex::default(), 1usize);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_index_rs_134_1_407509`).

## Output

```
thread 'main' (81020) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/index.rs:134:1:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <epaint::text::index::CharIndex as core::ops::arith::Sub<usize>>::sub
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/index.rs:90:22
   4: repro_epaint_src_text_index_rs_134_1_407509::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
