# epaint/src/tessellator.rs:648:48

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/tessellator.rs:648:48`
- last API call: `epaint::tessellator::path::add_circle_quadrant`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13035834257690667831

## Message

```
range start index 32000 out of range for slice of length 129
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use emath::Pos2;

fn main() {
    epaint::tessellator::path::add_circle_quadrant(&mut vec![], Pos2::NAN, 1000.0f32, 1000.0f32);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_tessellator_rs_648_48_e50533`).

## Output

```
thread 'main' (173271) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:648:48:
range start index 32000 out of range for slice of length 129
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeInclusive<usize> as core::slice::index::SliceIndex<[emath::vec2::Vec2]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:689:9
   4: <[emath::vec2::Vec2] as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <[emath::vec2::Vec2; 129] as core::ops::index::Index<core::ops::range::RangeInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:391:9
   6: epaint::tessellator::path::add_circle_quadrant
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:648:48
   7: repro_epaint_src_tessellator_rs_648_48_e50533::main
             at ./src/main.rs:4:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
