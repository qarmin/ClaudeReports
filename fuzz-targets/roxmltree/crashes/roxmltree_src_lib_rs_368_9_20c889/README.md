# roxmltree/src/lib.rs:368:9

- crate: https://github.com/RazrFalcon/roxmltree @ 3f5c78dea57662b43a2f41961397b1f5ffe27166
- kind: panic
- location: `roxmltree/src/lib.rs:368:9`
- last API call: `roxmltree::NodeId::from<u32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: id < u32::MAX
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
roxmltree = { git = "https://github.com/RazrFalcon/roxmltree", rev = "3f5c78dea57662b43a2f41961397b1f5ffe27166" }
```

`src/main.rs`:

```rust
use roxmltree::NodeId;

fn main() {
    let _ = NodeId::from(u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_roxmltree_src_lib_rs_368_9_20c889`).

## Output

```
thread 'main' (546614) panicked at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/lib.rs:368:9:
assertion failed: id < u32::MAX
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <roxmltree::NodeId>::new
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/lib.rs:368:9
   4: <roxmltree::NodeId as core::convert::From<u32>>::from
             at /home/rafal/.cargo/git/checkouts/roxmltree-65eeae7aeec35eb5/3f5c78d/src/lib.rs:390:9
   5: repro_roxmltree_src_lib_rs_368_9_20c889::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
