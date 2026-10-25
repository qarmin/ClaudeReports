# krilla/src/interactive/destination.rs:137:17

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/destination.rs:137:17`
- last API call: `krilla::document::Document::finish`
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
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::destination::NamedDestination;
use krilla::destination::XyzDestination;
use krilla::document::Document;
use krilla::geom::Point;

fn main() {
    let mut document = Document::new();
    let _ = document.register_named_destination(NamedDestination::new("".to_string(), XyzDestination::new(usize::MAX, Point { x: 0.0f32, y: 0.0f32 })));
    let _ = document.finish();
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_destination_rs_137_17_4e2eb1`).

## Output

```
thread 'main' (251351) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:137:17:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <krilla::interactive::destination::XyzDestination>::serialize::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:137:17
   4: <core::option::Option<&krilla::serialize::PageInfo>>::unwrap_or_else::<<krilla::interactive::destination::XyzDestination>::serialize::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:21
   5: <krilla::interactive::destination::XyzDestination>::serialize
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:134:64
   6: <krilla::serialize::SerializeContext>::serialize_xyz_destinations
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:848:18
   7: <krilla::serialize::SerializeContext>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:481:14
   8: <krilla::document::Document>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/document.rs:160:31
   9: repro_krilla_src_interactive_destination_rs_137_17_4e2eb1::main
             at ./src/main.rs:9:22
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
