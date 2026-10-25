# vello_common/src/util.rs:238:60

- crate: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65
- kind: panic
- location: `vello_common/src/util.rs:238:60`
- source: https://docs.rs/crate/vello_common/0.3.0/source/src/util.rs#238
- last API call: `hayro::render`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hayro = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-interpret = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-syntax = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
```

`src/main.rs`:

```rust
use hayro::PixmapSettings;
use hayro::RenderCache;
use hayro::RenderSettings;
use hayro_interpret::InterpreterSettings;
use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let pdf = Pdf::new_with_password(PdfData::from(include_bytes!("data_0.bin").to_vec()), "").unwrap();
    let pages = pdf.pages();
    let page = pages.get(0usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings { force_image_interpolation: false }, &PixmapSettings::default());
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_util_rs_238_60_3fc766`).

## Output

```
thread 'main' (233859) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/util.rs:238:60:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<u16>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <vello_common::geometry::RectU16 as vello_common::util::RectExt>::snap_to_tile_coordinates
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/util.rs:238:60
   6: <vello_cpu::coarse::bucketer::CommandBucketer>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/coarse/bucketer.rs:195:29
   7: <vello_cpu::coarse::bucketer::CommandBucketer>::from_wh
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/coarse/bucketer.rs:188:9
   8: <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/dispatch/single_threaded.rs:55:36
   9: <vello_cpu::render::RenderContext>::new_with
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:216:24
  10: <vello_cpu::render::RenderContext>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:197:9
  11: hayro::render
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:238:19
  12: repro_vello_common_src_util_rs_238_60_3fc766::main
             at ./src/main.rs:13:13
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
