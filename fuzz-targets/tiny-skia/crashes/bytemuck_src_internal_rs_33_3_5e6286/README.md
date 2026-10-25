# bytemuck/src/internal.rs:33:3

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `bytemuck/src/internal.rs:33:3`
- source: https://docs.rs/crate/bytemuck/1.25.2/source/src/internal.rs#33
- last API call: `tiny_skia::PixmapRef::pixel`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
cast_slice>OutputSliceWouldHaveSlop
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-skia = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
```

`src/main.rs`:

```rust
use tiny_skia::PixmapRef;

fn main() {
    let vec = vec![50u8, 50u8, 50u8, 50u8, 50u8];
    let pixmap_ref = PixmapRef::from_bytes(&vec, 1u32, 1u32).unwrap();
    let pixmap = pixmap_ref.to_owned();
    let pixmap_ref2 = pixmap.as_ref();
    let _ = pixmap_ref2.pixel(0u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_bytemuck_src_internal_rs_33_3_5e6286`).

## Output

```
thread 'main' (21746) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.25.2/src/internal.rs:33:3:
cast_slice>OutputSliceWouldHaveSlop
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: bytemuck::internal::something_went_wrong::<bytemuck::PodCastError>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.25.2/src/internal.rs:33:3
   3: bytemuck::internal::cast_slice::<u8, tiny_skia::color::PremultipliedColorU8>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.25.2/src/internal.rs:257:15
   4: bytemuck::cast_slice::<u8, tiny_skia::color::PremultipliedColorU8>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.25.2/src/lib.rs:413:12
   5: <tiny_skia::pixmap::PixmapRef>::pixels
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/pixmap.rs:377:9
   6: <tiny_skia::pixmap::PixmapRef>::pixel
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/pixmap.rs:372:14
   7: repro_bytemuck_src_internal_rs_33_3_5e6286::main
             at ./src/main.rs:8:25
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
