# krilla/src/interactive/destination.rs:135:13

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/destination.rs:135:13`
- last API call: `krilla::document::Document::finish`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempted to link to page 2, but document only has 1 pages
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::SerializeSettings;
use krilla::destination::NamedDestination;
use krilla::destination::XyzDestination;
use krilla::document::Document;
use krilla::geom::Point;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let _ = document.register_named_destination(NamedDestination::new("".to_string(), XyzDestination::new(1usize, Point::default())));
    let _ = document.finish();
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_destination_rs_135_13_42d342`).

## Output

```
thread 'main' (250940) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:135:13:
attempted to link to page 2, but document only has 1 pages
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <krilla::interactive::destination::XyzDestination>::serialize::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:135:13
   3: <core::option::Option<&krilla::serialize::PageInfo>>::unwrap_or_else::<<krilla::interactive::destination::XyzDestination>::serialize::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1065:21
   4: <krilla::interactive::destination::XyzDestination>::serialize
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/destination.rs:134:64
   5: <krilla::serialize::SerializeContext>::serialize_xyz_destinations
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:848:18
   6: <krilla::serialize::SerializeContext>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/serialize.rs:481:14
   7: <krilla::document::Document>::finish
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/document.rs:160:31
   8: repro_krilla_src_interactive_destination_rs_135_13_42d342::main
             at ./src/main.rs:10:22
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
