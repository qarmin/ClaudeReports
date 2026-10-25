# getopts/src/lib.rs:699:5

- crate: https://github.com/rust-lang/getopts @ c0cdd215f184c920adca2aba7a2086d38f468119
- kind: panic
- location: `getopts/src/lib.rs:699:5`
- last API call: `getopts::Options::optflag`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
the short_name (first argument) should be a single character, or an empty string for none
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
getopts = { git = "https://github.com/rust-lang/getopts", rev = "c0cdd215f184c920adca2aba7a2086d38f468119" }
```

`src/main.rs`:

```rust
use getopts::Fail;
use getopts::Options;

fn main() {
    let mut options = Options::default();
    let fail = Fail::OptionDuplicated("".to_string());
    let string = fail.to_string();
    let _ = options.optflag("م", &string, "");
}
```

Run with `cargo run` in this directory (package `repro_getopts_src_lib_rs_699_5_9a8b55`).

## Output

```
thread 'main' (1375971) panicked at /home/rafal/.cargo/git/checkouts/getopts-30557b485d6e4d34/c0cdd21/src/lib.rs:699:5:
the short_name (first argument) should be a single character, or an empty string for none
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: getopts::validate_names
             at /home/rafal/.cargo/git/checkouts/getopts-30557b485d6e4d34/c0cdd21/src/lib.rs:699:5
   3: <getopts::Options>::optflag
             at /home/rafal/.cargo/git/checkouts/getopts-30557b485d6e4d34/c0cdd21/src/lib.rs:219:9
   4: repro_getopts_src_lib_rs_699_5_9a8b55::main
             at ./src/main.rs:8:21
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
