# epaint/src/mesh.rs:232:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/mesh.rs:232:9`
- last API call: `epaint::Mesh::add_colored_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 822761989001455917

## Message

```
Mesh has an assigned texture
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use emath::Rect;
use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let mut mesh = Mesh::with_texture(TextureId::User(1u64));
    mesh.add_colored_rect(Rect::ZERO, Color32::DARK_BLUE);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_mesh_rs_232_9_5f93ef`).

## Output

```
thread 'main' (201469) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/mesh.rs:232:9:
Mesh has an assigned texture
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::mesh::Mesh>::add_colored_rect
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/mesh.rs:232:9
   3: repro_epaint_src_mesh_rs_232_9_5f93ef::main
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:8:10
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
