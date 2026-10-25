# allsorts/src/scripts/indic.rs:1270:14

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/scripts/indic.rs:1270:14`
- last API call: `allsorts::scripts::indic::indic2_tag`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6674315305308808390

## Message

```
Expected an Indic1 script tag
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = allsorts::scripts::indic::indic2_tag(0u32);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_scripts_indic_rs_1270_14_625094`).

## Output

```
thread 'main' (24099) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/scripts/indic.rs:1270:14:
Expected an Indic1 script tag
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: allsorts::scripts::indic::indic2_tag
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/scripts/indic.rs:1270:14
   3: repro_allsorts_src_scripts_indic_rs_1270_14_625094::main
             at ./src/main.rs:2:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
