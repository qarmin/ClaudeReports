# usvg/src/tree/mod.rs:511:9

- crate: https://github.com/linebender/resvg @ 978775b5f2d31f35b370e0ac4059038be117101c
- kind: panic
- location: `usvg/src/tree/mod.rs:511:9`
- last API call: `usvg::StrokeMiterlimit::from<f32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: n.is_finite()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
resvg = { git = "https://github.com/linebender/resvg", rev = "978775b5f2d31f35b370e0ac4059038be117101c" }
usvg = { git = "https://github.com/linebender/resvg", rev = "978775b5f2d31f35b370e0ac4059038be117101c" }
```

`src/main.rs`:

```rust
use usvg::StrokeMiterlimit;

fn main() {
    let _ = StrokeMiterlimit::from(f32::NAN);
}
```

Run with `cargo run` in this directory (package `repro_usvg_src_tree_mod_rs_511_9_61e5b6`).

## Output

```
thread 'main' (1373693) panicked at /home/rafal/.cargo/git/checkouts/resvg-2970f6cc9c85afc6/978775b/crates/usvg/src/tree/mod.rs:511:9:
assertion failed: n.is_finite()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <usvg::tree::StrokeMiterlimit>::new
             at /home/rafal/.cargo/git/checkouts/resvg-2970f6cc9c85afc6/978775b/crates/usvg/src/tree/mod.rs:511:9
   4: <usvg::tree::StrokeMiterlimit as core::convert::From<f32>>::from
             at /home/rafal/.cargo/git/checkouts/resvg-2970f6cc9c85afc6/978775b/crates/usvg/src/tree/mod.rs:536:9
   5: repro_usvg_src_tree_mod_rs_511_9_61e5b6::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
