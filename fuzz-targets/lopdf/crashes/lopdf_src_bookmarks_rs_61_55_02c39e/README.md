# lopdf/src/bookmarks.rs:61:55

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: panic
- location: `lopdf/src/bookmarks.rs:61:55`
- last API call: `lopdf::Document::build_outline`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1" }
```

`src/main.rs`:

```rust
use lopdf::Bookmark;
use lopdf::Document;

fn main() {
    let mut document = Document::new_from_prev(&Document::default());
    let _ = document.add_bookmark(Bookmark { children: vec![0u32], title: "".to_string(), format: 0u32, color: [0.0f32, 0.0f32, 0.0f32], page: (0u32, 1u16), id: 0u32 }, None);
    let _ = document.build_outline();
}
```

Run with `cargo run` in this directory (package `repro_lopdf_src_bookmarks_rs_61_55_02c39e`).

## Output

```
thread 'main' (399002) panicked at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/bookmarks.rs:61:55:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&lopdf::bookmarks::Bookmark>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <lopdf::document::Document>::outline_child
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/bookmarks.rs:61:55
   6: <lopdf::document::Document>::outline_child
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/bookmarks.rs:104:55
   7: <lopdf::document::Document>::build_outline
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/bookmarks.rs:133:45
   8: repro_lopdf_src_bookmarks_rs_61_55_02c39e::main
             at ./src/main.rs:7:22
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
