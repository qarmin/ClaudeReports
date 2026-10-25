# notify-rust/src/image.rs:45:27

- crate: https://github.com/hoodie/notify-rust @ 7841a687bea2212396a0c269ef1092b78ef3a270
- kind: panic
- location: `notify-rust/src/image.rs:45:27`
- last API call: `notify_rust::Image::from_rgb`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
notify-rust = { git = "https://github.com/hoodie/notify-rust", rev = "7841a687bea2212396a0c269ef1092b78ef3a270", features = ["async", "chrono", "d", "d_vendored", "dbus", "debug_namespace", "env_logger", "image", "images", "images_no_default_features", "lazy_static", "preview-macos-un", "serde", "tokio", "z", "z-with-tokio", "zbus"] }
```

`src/main.rs`:

```rust
use notify_rust::Image;

fn main() {
    let _ = Image::from_rgb(-1000000000i32, -1000000000i32, vec![]);
}
```

Run with `cargo run` in this directory (package `repro_notify_rust_src_image_rs_45_27_fd7f69`).

## Output

```
thread 'main' (545422) panicked at /home/rafal/.cargo/git/checkouts/notify-rust-34ac0504adee4afa/7841a68/src/image.rs:45:27:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <notify_rust::image::Image>::from_raw_data
             at /home/rafal/.cargo/git/checkouts/notify-rust-34ac0504adee4afa/7841a68/src/image.rs:45:27
   4: <notify_rust::image::Image>::from_rgb
             at /home/rafal/.cargo/git/checkouts/notify-rust-34ac0504adee4afa/7841a68/src/image.rs:64:9
   5: repro_notify_rust_src_image_rs_45_27_fd7f69::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
