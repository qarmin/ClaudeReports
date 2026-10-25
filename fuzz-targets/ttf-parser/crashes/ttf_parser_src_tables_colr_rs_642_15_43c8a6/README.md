# ttf-parser/src/tables/colr.rs:642:15

- crate: https://github.com/harfbuzz/ttf-parser @ 0c7291223fe9e0bc2808f0255cb991066eedd796
- kind: panic
- location: `ttf-parser/src/tables/colr.rs:642:15`
- last API call: `ttf_parser::colr::CompositeMode::parse`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ttf-parser = { git = "https://github.com/harfbuzz/ttf-parser", rev = "0c7291223fe9e0bc2808f0255cb991066eedd796" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <ttf_parser::colr::CompositeMode as ttf_parser::FromData>::parse(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_ttf_parser_src_tables_colr_rs_642_15_43c8a6`).

## Output

```
thread 'main' (457663) panicked at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/tables/colr.rs:642:15:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <ttf_parser::tables::colr::CompositeMode as ttf_parser::parser::FromData>::parse
             at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/tables/colr.rs:642:15
   4: repro_ttf_parser_src_tables_colr_rs_642_15_43c8a6::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
