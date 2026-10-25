# epaint/src/texture_atlas.rs:226:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/texture_atlas.rs:226:9`
- last API call: `epaint::TextureAtlas::allocate`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 530575767382216039

## Message

```
Tried to allocate a 65536 wide glyph in a 10000 wide texture atlas
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let mut texture_atlas = TextureAtlas::new([10000usize, 1usize], TextOptions::default());
    let _ = texture_atlas.allocate((65536usize, 0usize));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_texture_atlas_rs_226_9_672381`).

## Output

```
thread 'main' (174207) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/texture_atlas.rs:226:9:
Tried to allocate a 65536 wide glyph in a 10000 wide texture atlas
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::texture_atlas::TextureAtlas>::allocate
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/texture_atlas.rs:226:9
   3: repro_epaint_src_texture_atlas_rs_226_9_672381::main
             at ./src/main.rs:6:27
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
