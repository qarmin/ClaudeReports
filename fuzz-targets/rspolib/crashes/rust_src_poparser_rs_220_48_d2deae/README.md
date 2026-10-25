# rust/src/poparser.rs:220:48

- crate: https://github.com/mondeja/rspolib @ 12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8
- kind: panic
- location: `rust/src/poparser.rs:220:48`
- last API call: `rspolib::pofile`
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
rspolib = { git = "https://github.com/mondeja/rspolib", rev = "12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8" }
```

`src/main.rs`:

```rust
use rspolib::FileOptions;

fn main() {
    let _ = rspolib::pofile::<rspolib::FileOptions>(FileOptions { path_or_content: "\"\u{5cac8}\"".to_string(), wrapwidth: 0usize, byte_content: None });
}
```

Run with `cargo run` in this directory (package `repro_rust_src_poparser_rs_220_48_d2deae`).

## Output

```
thread 'main' (1365076) panicked at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:220:48:
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
   4: <core::option::Option<&(rspolib::poparser::St, rspolib::poparser::St)>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <rspolib::poparser::POFileParser>::process
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:220:48
   6: <rspolib::poparser::POFileParser>::parse_line
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:388:18
   7: <rspolib::poparser::POFileParser>::parse_with_handler
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:262:14
   8: <rspolib::poparser::POFileParser>::parse_content
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:253:14
   9: <rspolib::poparser::POFileParser>::parse
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/poparser.rs:235:18
  10: rspolib::file::pofile::pofile::<rspolib::file::FileOptions>
             at /home/rafal/.cargo/git/checkouts/rspolib-5d7d9a97a85088df/12343ca/rust/src/file/pofile.rs:140:12
  11: repro_rust_src_poparser_rs_220_48_d2deae::main
             at ./src/main.rs:4:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
