# wasm-bindgen/src/lib.rs:1227:1

- crate: https://github.com/RReverser/serde-wasm-bindgen @ a16f4ce2468c0491b92b38ad3ad3bfb1c6f9323f
- kind: panic
- location: `wasm-bindgen/src/lib.rs:1227:1`
- source: https://docs.rs/crate/wasm-bindgen/0.2.129/source/src/lib.rs#1227
- last API call: `serde_wasm_bindgen::Error::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
cannot call wasm-bindgen imported functions on non-wasm targets
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
serde-wasm-bindgen = { git = "https://github.com/RReverser/serde-wasm-bindgen", rev = "a16f4ce2468c0491b92b38ad3ad3bfb1c6f9323f" }
```

`src/main.rs`:

```rust
use serde_wasm_bindgen::Error;

fn main() {
    let _ = Error::new::<String>("".to_string());
}
```

Run with `cargo run` in this directory (package `repro_wasm_bindgen_src_lib_rs_1227_1_9bcf70`).

## Output

```
thread 'main' (574247) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:1227:1:
cannot call wasm-bindgen imported functions on non-wasm targets
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: wasm_bindgen::__wbindgen_error_new::__wbg_Error_30c8987f7c2ed4e2
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:1242:8
   3: wasm_bindgen::__wbindgen_error_new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:1242:8
   4: <wasm_bindgen::JsError>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:1857:20
   5: <serde_wasm_bindgen::error::Error>::new::<alloc::string::String>
             at /home/rafal/.cargo/git/checkouts/serde-wasm-bindgen-b1f76ba176c3923f/a16f4ce/src/error.rs:24:15
   6: repro_wasm_bindgen_src_lib_rs_1227_1_9bcf70::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
