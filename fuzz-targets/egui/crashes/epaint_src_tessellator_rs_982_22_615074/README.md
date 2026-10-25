# epaint/src/tessellator.rs:982:22

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/tessellator.rs:982:22`
- last API call: `epaint::tessellator::Path::fill_with_uv`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9084672984381623971

## Message

```
attempt to subtract with overflow
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
use egui::Visuals;
use epaint::Mesh;
use epaint::TextureId;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    let mut mesh = Mesh::with_texture(TextureId::default());
    path.fill_with_uv(1.0f32, Visuals::light().weak_text_color(), TextureId::User(0u64), |a| a, &mut mesh);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_tessellator_rs_982_22_615074`).

## Output

```
thread 'main' (201730) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:982:22:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: epaint::tessellator::fill_closed_path_with_uv::<repro_epaint_src_tessellator_rs_982_22_615074::main::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:982:22
   4: <epaint::tessellator::Path>::fill_with_uv::<repro_epaint_src_tessellator_rs_982_22_615074::main::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:548:9
   5: repro_epaint_src_tessellator_rs_982_22_615074::main
             at ./src/main.rs:9:10
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
