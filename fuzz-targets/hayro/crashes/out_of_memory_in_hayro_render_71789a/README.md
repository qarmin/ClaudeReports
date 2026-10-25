# out of memory in hayro::render

- crate: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65
- kind: out of memory
- last API call: `hayro::render`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling hayro::render
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
    let pdf = Pdf::new_with_password(PdfData::from(include_bytes!("data_0.bin").to_vec()), "%s%n{}").unwrap();
    let pages = pdf.pages();
    let page = pages.get(1usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings { force_image_interpolation: false }, &PixmapSettings::default());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_hayro_render_71789a`).

## Output

```
memory allocation of 2348810240 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<kurbo::bezpath::PathEl>>::extend_trusted::<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4128:18
  10: <alloc::vec::Vec<kurbo::bezpath::PathEl> as alloc::vec::spec_extend::SpecExtend<kurbo::bezpath::PathEl, core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
  11: <alloc::vec::Vec<kurbo::bezpath::PathEl> as core::iter::traits::collect::Extend<kurbo::bezpath::PathEl>>::extend::<&kurbo::bezpath::BezPath>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4064:9
  12: <kurbo::bezpath::BezPath as core::iter::traits::collect::Extend<kurbo::bezpath::PathEl>>::extend::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/bezpath.rs:550:16
  13: <kurbo::stroke::StrokeCtx>::finish
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/stroke.rs:406:21
  14: kurbo::stroke::stroke_undashed::<kurbo::stroke::DashIterator<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<kurbo::bezpath::PathEl>>>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/stroke.rs:315:21
  15: kurbo::stroke::stroke_with::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/kurbo-0.13.1/src/stroke.rs:297:9
  16: vello_common::flatten::expand_stroke::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/flatten.rs:256:5
  17: vello_common::flatten::stroke::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/flatten.rs:238:5
  18: <vello_common::strip_generator::StripGenerator>::generate_stroked_path::<&kurbo::bezpath::BezPath>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/strip_generator.rs:157:9
  19: <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/dispatch/single_threaded.rs:331:33
  20: <vello_common::viewport::ViewportState>::with_generator_and_clip::<(), <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_common-0.3.0/src/viewport.rs:62:9
  21: <vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher>::stroke_path
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/dispatch/single_threaded.rs:330:14
  22: <vello_cpu::render::RenderContext>::stroke_path::{closure#0}
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:322:28
  23: <vello_cpu::render::RenderContext>::with_optional_filter::<<vello_cpu::render::RenderContext>::stroke_path::{closure#0}>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:931:13
  24: <vello_cpu::render::RenderContext>::stroke_path
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/vello_cpu-0.3.0/src/render.rs:315:14
  25: <hayro::Renderer>::stroke_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/path.rs:49:18
  26: <hayro::Renderer>::draw_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/path.rs:82:17
  27: <hayro::Renderer as hayro_interpret::device::Device>::draw_path
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:158:9
  28: hayro_interpret::interpret::path::stroke_path_impl::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/path.rs:115:16
  29: hayro_interpret::interpret::path::stroke_path::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/path.rs:18:5
  30: hayro_interpret::interpret::interpret::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/mod.rs:394:17
  31: hayro_interpret::interpret::interpret_page::<hayro::Renderer>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-interpret/src/interpret/mod.rs:147:5
  32: hayro::render_into
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:301:5
  33: hayro::render
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro/src/lib.rs:246:5
  34: repro_out_of_memory_in_hayro_render_71789a::main
             at ./src/main.rs:13:13
  35: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
