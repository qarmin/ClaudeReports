# epaint/src/mesh.rs:244:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/mesh.rs:244:9`
- last API call: `epaint::Mesh::split_to_u16`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8795118772119019750

## Message

```
Mesh is invalid
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
use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let mesh = Mesh { indices: vec![0u32], vertices: vec![], texture_id: TextureId::User(0u64) };
    let _ = mesh.clone().split_to_u16();
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_mesh_rs_244_9_dcefff`).

## Output

```
thread 'main' (232412) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/mesh.rs:244:9:
Mesh is invalid
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::mesh::Mesh>::split_to_u16
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/mesh.rs:244:9
   3: repro_epaint_src_mesh_rs_244_9_dcefff::main
             at ./src/main.rs:6:26
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
