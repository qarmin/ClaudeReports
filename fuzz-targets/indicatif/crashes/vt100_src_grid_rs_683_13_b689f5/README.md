# vt100/src/grid.rs:683:13

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: panic
- location: `vt100/src/grid.rs:683:13`
- source: https://docs.rs/crate/vt100/0.16.2/source/src/grid.rs#683
- last API call: `indicatif::InMemoryTerm::write_str`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
indicatif = { git = "https://github.com/console-rs/indicatif", rev = "6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9", features = ["futures", "improved_unicode", "in_memory", "rayon", "tokio", "unicode-segmentation", "unicode-width", "vt100", "wasmbind"] }
```

`src/main.rs`:

```rust
use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::write_str(&InMemoryTerm::new(1u16, 2u16), "👍🏽");
}
```

Run with `cargo run` in this directory (package `repro_vt100_src_grid_rs_683_13_b689f5`).

## Output

```
thread 'main' (371394) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/grid.rs:683:13:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <vt100::grid::Grid>::col_wrap
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/grid.rs:683:13
   4: <vt100::screen::Screen>::text
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/screen.rs:745:25
   5: <vt100::perform::WrappedScreen as vte::Perform>::print
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/perform.rs:38:25
   6: <vte::Parser<1024>>::ground_dispatch::<vt100::perform::WrappedScreen>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vte-0.15.0/src/lib.rs:726:32
   7: <vte::Parser<1024>>::advance_ground::<vt100::perform::WrappedScreen>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vte-0.15.0/src/lib.rs:611:17
   8: <vte::Parser<1024>>::advance::<vt100::perform::WrappedScreen>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vte-0.15.0/src/lib.rs:119:44
   9: <vt100::parser::Parser>::process
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/parser.rs:49:21
  10: <vt100::parser::Parser as core::io::write::Write>::write
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vt100-0.16.2/src/parser.rs:89:14
  11: <vt100::parser::Parser as core::io::write::Write>::write_all
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/io/write.rs:216:24
  12: <indicatif::in_memory::InMemoryTermState>::write_str
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/in_memory.rs:213:21
  13: <indicatif::in_memory::InMemoryTerm as indicatif::term_like::TermLike>::write_str
             at /home/rafal/.cargo/git/checkouts/indicatif-a1328a59f5b1ae17/6132d8d/src/in_memory.rs:179:15
  14: repro_vt100_src_grid_rs_683_13_b689f5::main
             at ./src/main.rs:4:13
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
