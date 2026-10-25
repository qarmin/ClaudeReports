# imgref/src/lib.rs:679:9

- crate: femtovg 0.27.0
- kind: panic
- location: `imgref/src/lib.rs:679:9`
- source: https://docs.rs/crate/imgref/1.12.3/source/src/lib.rs#679
- last API call: `femtovg::renderer::Void::screenshot`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Invalid ImgRef params. Got stride=0 for 0×0; len=0 (needed 0)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
femtovg = { version = "=0.27.0" }
```

`src/main.rs`:

```rust
use femtovg::renderer::Void;

fn main() {
    let mut void = Void;
    let _ = <femtovg::renderer::Void as femtovg::Renderer>::screenshot(&mut void);
}
```

Run with `cargo run` in this directory (package `repro_imgref_src_lib_rs_679_9_6e2bf6`).

## Output

```
thread 'main' (232886) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/imgref-1.12.3/src/lib.rs:679:9:
Invalid ImgRef params. Got stride=0 for 0×0; len=0 (needed 0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: imgref::imgref_invalid_size
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/imgref-1.12.3/src/lib.rs:687:5
   3: <imgref::Img<alloc::vec::Vec<rgb::formats::rgba::Rgba<u8>>>>::new_stride
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/imgref-1.12.3/src/lib.rs:654:13
   4: <imgref::Img<alloc::vec::Vec<rgb::formats::rgba::Rgba<u8>>>>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/imgref-1.12.3/src/lib.rs:679:9
   5: <femtovg::renderer::void::Void as femtovg::renderer::Renderer>::screenshot
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/renderer/void.rs:65:12
   6: repro_imgref_src_lib_rs_679_9_6e2bf6::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
