# epaint/src/tessellator.rs:954:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/tessellator.rs:954:9`
- last API call: `epaint::tessellator::Path::fill_with_uv`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7809134214557046431

## Message

```
assertion `left == right` failed: Mixing different `texture_id` in the same 
  left: Managed(0)
 right: Managed(1)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use epaint::Mesh;
use epaint::TextureId;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    let mut mesh = Mesh::from(Mesh { indices: vec![0u32], vertices: vec![], texture_id: TextureId::default() });
    path.fill_with_uv(0.0f32, Color32::from_rgba_unmultiplied_const(0u8, 0u8, 0u8, 1u8), TextureId::Managed(1u64), |a| a, &mut mesh);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_tessellator_rs_954_9_7e59b8`).

## Output

```
thread 'main' (201015) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:954:9:
assertion `left == right` failed: Mixing different `texture_id` in the same 
  left: Managed(0)
 right: Managed(1)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<epaint::TextureId, epaint::TextureId>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: epaint::tessellator::fill_closed_path_with_uv::<repro_epaint_src_tessellator_rs_954_9_7e59b8::main::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:954:9
   5: <epaint::tessellator::Path>::fill_with_uv::<repro_epaint_src_tessellator_rs_954_9_7e59b8::main::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/tessellator.rs:548:9
   6: repro_epaint_src_tessellator_rs_954_9_7e59b8::main
             at ./src/main.rs:9:10
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
