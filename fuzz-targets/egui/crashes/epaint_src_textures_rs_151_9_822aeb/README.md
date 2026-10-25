# epaint/src/textures.rs:151:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/textures.rs:151:9`
- last API call: `epaint::textures::TextureMeta::bytes_used`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3219077064864186366

## Message

```
attempt to multiply with overflow
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
use epaint::textures::TextureMeta;
use epaint::textures::TextureOptions;

fn main() {
    let texture_meta = TextureMeta { name: "".to_string(), size: [1000000000usize, 1000000000usize], bytes_per_pixel: 1000usize, retain_count: 0usize, options: TextureOptions::NEAREST_MIRRORED_REPEAT };
    let _ = texture_meta.bytes_used();
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_textures_rs_151_9_822aeb`).

## Output

```
thread 'main' (83805) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:151:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <epaint::textures::TextureMeta>::bytes_used
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/textures.rs:151:9
   4: repro_epaint_src_textures_rs_151_9_822aeb::main
             at ./src/main.rs:6:26
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
