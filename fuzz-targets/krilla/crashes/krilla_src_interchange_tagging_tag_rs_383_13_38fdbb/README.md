# krilla/src/interchange/tagging/tag.rs:383:13

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interchange/tagging/tag.rs:383:13`
- last API call: `krilla::tagging::NaiveRgbColor::new_f32`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
RGB color components must be in the range [0.0, 1.0]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::tagging::NaiveRgbColor;

fn main() {
    let _ = NaiveRgbColor::new_f32(0.0f32, 1.0f32, 716.0f32);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interchange_tagging_tag_rs_383_13_38fdbb`).

## Output

```
thread 'main' (250302) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interchange/tagging/tag.rs:383:13:
RGB color components must be in the range [0.0, 1.0]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <krilla::interchange::tagging::tag::NaiveRgbColor>::new_f32
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interchange/tagging/tag.rs:383:13
   3: repro_krilla_src_interchange_tagging_tag_rs_383_13_38fdbb::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
