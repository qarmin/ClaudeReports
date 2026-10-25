# epaint/src/shapes/shape.rs:386:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/shapes/shape.rs:386:9`
- last API call: `epaint::Shape::mesh`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18317522533205533246

## Message

```
Invalid mesh: Mesh {
    indices: [
        1,
    ],
    vertices: [],
    texture_id: Managed(
        0,
    ),
}
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
use epaint::Shape;
use epaint::TextureId;

fn main() {
    let _ = Shape::mesh(std::sync::Arc::new(Mesh { indices: vec![1u32], vertices: vec![], texture_id: TextureId::Managed(0u64) }));
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_shapes_shape_rs_386_9_535444`).

## Output

```
thread 'main' (204728) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/shape.rs:386:9:
Invalid mesh: Mesh {
    indices: [
        1,
    ],
    vertices: [],
    texture_id: Managed(
        0,
    ),
}
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::shapes::shape::Shape>::mesh::<alloc::sync::Arc<epaint::mesh::Mesh>>
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/shape.rs:386:9
   3: repro_epaint_src_shapes_shape_rs_386_9_535444::main
             at ./src/main.rs:6:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
