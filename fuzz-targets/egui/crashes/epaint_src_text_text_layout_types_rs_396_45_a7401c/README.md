# epaint/src/text/text_layout_types.rs:396:45

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/text/text_layout_types.rs:396:45`
- last API call: `epaint::text::VariationCoords::push[[u8; 4]]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 12134377052164617515

## Message

```
Invalid variation axis tag: InvalidByte { pos: 0, byte: 0 }
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
use epaint::text::VariationCoords;

fn main() {
    let mut variation_coords = VariationCoords::default();
    variation_coords.push([0u8, 0u8, 0u8, 0u8], 1.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_text_text_layout_types_rs_396_45_a7401c`).

## Output

```
thread 'main' (84003) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:396:45:
Invalid variation axis tag: InvalidByte { pos: 0, byte: 0 }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<font_types::tag::Tag, font_types::tag::InvalidTag>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <[u8; 4] as epaint::text::text_layout_types::IntoTag>::into_tag
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:396:45
   5: <epaint::text::text_layout_types::VariationCoords>::push::<[u8; 4]>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/text/text_layout_types.rs:439:26
   6: repro_epaint_src_text_text_layout_types_rs_396_45_a7401c::main
             at ./src/main.rs:5:22
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
