# rust/src/poparser.rs:849:18

- crate: https://github.com/mondeja/rspolib @ 12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8
- kind: panic
- location: `rust/src/poparser.rs:849:18`
- last API call: `rspolib::pofile`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
byte range starts at 1 but ends at 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rspolib = { git = "https://github.com/mondeja/rspolib", rev = "12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8" }
```

`src/main.rs`:

```rust
use rspolib::FileOptions;

fn main() {
    let file_options = FileOptions { path_or_content: "\"".to_string(), wrapwidth: 0usize, byte_content: None };
    let _ = rspolib::pofile::<rspolib::FileOptions>(file_options.clone());
}
```

Run with `cargo run` in this directory (package `repro_rust_src_poparser_rs_849_18_93f10a`).

## Output

```
thread 'main' (1364984) panicked at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:849:18:
byte range starts at 1 but ends at 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::str::slice_error_fail_rt
   3: core::str::slice_error_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/str/mod.rs:70:5
   4: <core::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:335:21
   5: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<str>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/traits.rs:248:34
   6: <alloc::string::String as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2839:15
   7: rspolib::poparser::maybe_raise_unescaped_double_quote_found_error
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:849:18
   8: <rspolib::poparser::POFileParser>::parse_line
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:381:13
   9: <rspolib::poparser::POFileParser>::parse_with_handler
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:262:14
  10: <rspolib::poparser::POFileParser>::parse_content
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:253:14
  11: <rspolib::poparser::POFileParser>::parse
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:235:18
  12: rspolib::file::pofile::pofile::<rspolib::file::FileOptions>
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/file/pofile.rs:140:12
  13: repro_rust_src_poparser_rs_849_18_93f10a::main
             at ./src/main.rs:5:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
