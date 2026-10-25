# epaint/src/textures.rs:337:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/textures.rs:337:9`
- last API call: `egui::Context::run_ui`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 16823816923639158756

## Message

```
Dropped TexturesDelta with 1 unapplied deltas. Deltas need to be handled. If you want to drop this intentionally call `clear` before dropping.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use egui::Context;
use egui::RawInput;

fn main() {
    let context = Context::default();
    let _ = context.run_ui(RawInput::default(), |_| ());
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_textures_rs_337_9_69210d`).

## Output

```
thread 'main' (16417) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:337:9:
Dropped TexturesDelta with 1 unapplied deltas. Deltas need to be handled. If you want to drop this intentionally call `clear` before dropping.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::textures::TexturesDelta as core::ops::drop::Drop>::drop
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:337:9
   3: core::ptr::drop_glue::<epaint::textures::TexturesDelta>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   4: core::ptr::drop_glue::<egui::data::output::FullOutput>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   5: repro_epaint_src_textures_rs_337_9_69210d::main
             at ./src/main.rs:6:56
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
