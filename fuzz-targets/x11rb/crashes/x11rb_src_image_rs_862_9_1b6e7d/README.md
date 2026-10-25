# x11rb/src/image.rs:862:9

- crate: https://github.com/psychon/x11rb @ 86c4252fe023fddd523f0ac3c7c07c40ba266ce0
- kind: panic
- location: `x11rb/src/image.rs:862:9`
- last API call: `x11rb::image::Image::put_pixel`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: x < self.width
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
x11rb = { git = "https://github.com/psychon/x11rb", rev = "86c4252fe023fddd523f0ac3c7c07c40ba266ce0", features = ["all-extensions", "allow-unsafe-code", "as-raw-xcb-connection", "composite", "cursor", "damage", "dbe", "dl-libxcb", "dpms", "dri2", "dri3", "extra-traits", "glx", "image", "libc", "libloading", "once_cell", "present", "randr", "raw-window-handle", "record", "render", "request-parsing", "res", "resource_manager", "screensaver", "shape", "shm", "sync", "tracing", "xcursor", "xevie", "xf86dri", "xf86vidmode", "xfixes", "xinerama", "xinput", "xkb", "xprint", "xselinux", "xtest", "xv", "xvmc"] }
```

`src/main.rs`:

```rust
use x11rb::image::BitsPerPixel;
use x11rb::image::Image;
use x11rb::image::ImageOrder;
use x11rb::image::ScanlinePad;

fn main() {
    let image = Image::allocate(0u16, 0u16, ScanlinePad::Pad8, 0u8, BitsPerPixel::B8, ImageOrder::MsbFirst);
    let image2 = image.clone().into_owned();
    let mut image3 = image2.clone().into_owned();
    image3.put_pixel(0u16, 0u16, 1u32);
}
```

Run with `cargo run` in this directory (package `repro_x11rb_src_image_rs_862_9_1b6e7d`).

## Output

```
thread 'main' (327914) panicked at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb/src/image.rs:862:9:
assertion failed: x < self.width
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <x11rb::image::Image>::put_pixel
             at /home/rafal/.cargo/git/checkouts/x11rb-80c70fee2abdbba1/86c4252/x11rb/src/image.rs:862:9
   4: repro_x11rb_src_image_rs_862_9_1b6e7d::main
             at ./src/main.rs:10:12
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
