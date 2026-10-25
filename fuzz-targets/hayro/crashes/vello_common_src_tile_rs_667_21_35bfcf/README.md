# vello_common/src/tile.rs:667:21

- crate: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65
- kind: panic
- location: `vello_common/src/tile.rs:667:21`
- source: https://docs.rs/crate/vello_common/0.3.0/source/src/tile.rs#667
- last API call: `hayro::render`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18350257370851280604

## Message

```
attempt to add with overflow
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
    let pdf = Pdf::new(PdfData::from(include_bytes!("data_0.bin").to_vec())).unwrap();
    let pages = pdf.pages();
    let page = pages.get(0usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings { force_image_interpolation: false }, &PixmapSettings::default());
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_tile_rs_667_21_35bfcf`).

## Output

```
thread 'main' (4061018) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/tile.rs:667:21:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <vello_common::tile::Tiles>::make_tiles_analytic_aa_impl::<fearless_simd::generated::avx512::Avx512>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/tile.rs:667:21
   4: <vello_common::tile::Tiles>::make_tiles_analytic_aa::{closure#3}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/tile.rs:521:39
   5: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::vectorize_avx512::<<vello_common::tile::Tiles>::make_tiles_analytic_aa::{closure#3}, bool>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-0.7.0/src/generated/avx512.rs:388:13
   6: <fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::vectorize::<<vello_common::tile::Tiles>::make_tiles_analytic_aa::{closure#3}, bool>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fearless_simd-0.7.0/src/generated/avx512.rs:390:18
   7: <vello_common::tile::Tiles>::make_tiles_analytic_aa
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/tile.rs:521:9
   8: <vello_common::strip_generator::StripGenerator>::generate_with_clip
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/strip_generator.rs:178:14
   9: <vello_common::strip_generator::StripGenerator>::generate_stroked_path::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/strip_generator.rs:167:14
  10: <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/dispatch/single_threaded.rs:331:33
  11: <vello_common::viewport::ViewportState>::with_generator_and_clip::<(), <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/viewport.rs:62:9
  12: <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/dispatch/single_threaded.rs:330:14
  13: <vello_cpu::render::RenderContext>::stroke_path::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:322:28
  14: <vello_cpu::render::RenderContext>::with_optional_filter::<<vello_cpu::render::RenderContext>::stroke_path::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:931:13
  15: <vello_cpu::render::RenderContext>::stroke_path
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:315:14
  16: <hayro::Renderer>::stroke_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/path.rs:49:18
  17: <hayro::Renderer>::draw_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/path.rs:82:17
  18: <hayro::Renderer as hayro_interpret::device::Device>::draw_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:158:9
  19: hayro_interpret::interpret::path::stroke_path_impl::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/path.rs:115:16
  20: hayro_interpret::interpret::path::stroke_path::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/path.rs:18:5
  21: hayro_interpret::interpret::interpret::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/mod.rs:394:17
  22: hayro_interpret::interpret::interpret_page::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/mod.rs:147:5
  23: hayro::render_into
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:301:5
  24: hayro::render
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:246:5
  25: repro_vello_common_src_tile_rs_667_21_35bfcf::main
             at ./src/main.rs:13:13
  26: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
