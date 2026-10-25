# vello_common/src/tile.rs:296:13

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/tile.rs:296:13`
- last API call: `vello_common::tile::Tile::new_clamped`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13401317486988940068

## Message

```
Max. number of lines per path exceeded.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::tile::Tile;

fn main() {
    let _ = Tile::new_clamped(0u16, 0u16, 1000000000u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_tile_rs_296_13_07081d`).

## Output

```
thread 'main' (1143114) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/tile.rs:296:13:
Max. number of lines per path exceeded.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <vello_common::tile::Tile>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/tile.rs:296:13
   3: <vello_common::tile::Tile>::new_clamped
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/tile.rs:277:9
   4: repro_vello_common_src_tile_rs_296_13_07081d::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
