# hayro - crashes

Source: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new_with_password(PdfData::from(include_bytes!(
    "data_0.bin").to_vec()), "%s%n{}").unwrap();
let pages = pdf.pages();
let page = pages.get(1usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings { force_image_interpolation: false },
    &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>out of memory: memory allocation of 2348810240 bytes failed
backtrace:
  kurbo::bezpath::{impl#4}::extend&lt;&amp;kurbo::bezpath::BezPath&gt; (src/bezpath.rs:550)
  kurbo::stroke::StrokeCtx::finish (src/stroke.rs:406)
  kurbo::stroke::stroke_undashed&lt;kurbo::stroke::DashIterator&lt;core::iter::adapters::cloned::Cloned&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt;&gt; (kurbo/src/stroke.rs:315)
  kurbo::stroke::stroke_with&lt;&amp;kurbo::bezpath::BezPath&gt; (kurbo/src/stroke.rs:297)
  vello_common::flatten::expand_stroke&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/flatten.rs:256)
  vello_common::flatten::stroke&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/flatten.rs:238)
  vello_common::strip_generator::StripGenerator::generate_stroked_path&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/strip_generator.rs:157)
  vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path::{closure#0} (src/dispatch/single_threaded.rs:331)
  vello_common::viewport::ViewportState::with_generator_and_clip&lt;(), vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path::{closure_env#0}&gt; (vello_common/src/viewport.rs:62)
  vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path (src/dispatch/single_threaded.rs:330)
  vello_cpu::render::{impl#3}::stroke_path::{closure#0} (src/render.rs:322)
  vello_cpu::render::RenderContext::with_optional_filter&lt;vello_cpu::render::{impl#3}::stroke_path::{closure_env#0}&gt; (src/render.rs:931)
  vello_cpu::render::RenderContext::stroke_path (src/render.rs:315)
  hayro::Renderer::stroke_path (src/path.rs:49)
  hayro::Renderer::draw_path (src/path.rs:82)
  hayro::{impl#2}::draw_path (src/lib.rs:158)
  hayro_interpret::interpret::path::stroke_path_impl&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/path.rs:115)
  hayro_interpret::interpret::path::stroke_path&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/path.rs:18)
  hayro_interpret::interpret::interpret&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/mod.rs:394)
  hayro_interpret::interpret::interpret_page&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/mod.rs:147)
  hayro::render_into (src/lib.rs:301)
  hayro::render (src/lib.rs:246)
  repro_out_of_memory_in_hayro_render_71789a::main (src/main.rs:13)
  (21 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Pdf::new_with_password(PdfData::from(include_bytes!(
    "data_0.bin").to_vec()), "");</pre></td><td valign="top"><pre>thread 'main' panicked at hayro-syntax/src/object/array.rs:153:18:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;hayro_syntax::object::array::ArrayIter as core::iter::traits::iterator::Iterator&gt;::next (hayro-syntax/src/object/array.rs:153:18)
  hayro_syntax::page::resolve_pages (hayro-syntax/src/page.rs:133:36)
  &lt;hayro_syntax::page::Pages&gt;::new (hayro-syntax/src/page.rs:56:9)
  &lt;hayro_syntax::page::cached::CachedPages&gt;::new::{closure#0} (hayro-syntax/src/page.rs:532:31)
  &lt;hayro_syntax::page::cached::CachedPages&gt;::new (hayro-syntax/src/page.rs:532:18)
  &lt;hayro_syntax::pdf::Pdf&gt;::new_with_password::&lt;hayro_syntax::data::PdfData&gt; (hayro-syntax/src/pdf.rs:61:21)
  repro_hayro_syntax_src_object_array_rs_153_18_dc92d1::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ParseError::UnexpectedEof;
let pdf = Pdf::new(PdfData::from(include_bytes!(
    "document_0.pdf").to_vec())).unwrap();
let pages = pdf.pages();
let page = pages.get(0usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings::default(), &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  kurbo::stroke::dash_iter&lt;core::iter::adapters::cloned::Cloned&lt;core::slice::iter::Iter&lt;kurbo::bezpath::PathEl&gt;&gt;&gt; (kurbo/src/stroke.rs:748)
  kurbo::stroke::stroke_with&lt;&amp;kurbo::bezpath::BezPath&gt; (kurbo/src/stroke.rs:291)
  vello_common::flatten::expand_stroke&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/flatten.rs:256)
  vello_common::flatten::stroke&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/flatten.rs:238)
  vello_common::strip_generator::StripGenerator::generate_stroked_path&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/strip_generator.rs:157)
  vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path::{closure#0} (src/dispatch/single_threaded.rs:331)
  vello_common::viewport::ViewportState::with_generator_and_clip&lt;(), vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path::{closure_env#0}&gt; (vello_common/src/viewport.rs:62)
  vello_cpu::dispatch::single_threaded::{impl#1}::stroke_path (src/dispatch/single_threaded.rs:330)
  vello_cpu::render::{impl#3}::stroke_path::{closure#0} (src/render.rs:322)
  vello_cpu::render::RenderContext::with_optional_filter&lt;vello_cpu::render::{impl#3}::stroke_path::{closure_env#0}&gt; (src/render.rs:931)
  vello_cpu::render::RenderContext::stroke_path (src/render.rs:315)
  hayro::Renderer::stroke_path (src/path.rs:49)
  hayro::Renderer::draw_path (src/path.rs:82)
  hayro::{impl#2}::draw_path (src/lib.rs:158)
  hayro_interpret::interpret::path::stroke_path_impl&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/path.rs:115)
  hayro_interpret::interpret::path::stroke_path&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/path.rs:18)
  hayro_interpret::interpret::interpret&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/mod.rs:310)
  hayro_interpret::interpret::interpret_page&lt;hayro::Renderer&gt; (hayro-interpret/src/interpret/mod.rs:147)
  hayro::render_into (src/lib.rs:301)
  hayro::render (src/lib.rs:246)
  repro_timeout_in_hayro_render_b1f27c::main (src/main.rs:15)</pre></td></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new_with_password(PdfData::from(include_bytes!(
    "data_0.bin").to_vec()), "").unwrap();
let pages = pdf.pages();
let page = pages.get(0usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings { force_image_interpolation: false },
    &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:238:60:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::RectU16 as vello_common::util::RectExt&gt;::snap_to_tile_coordinates (vello_common/src/util.rs:238:60)
  &lt;vello_cpu::coarse::bucketer::CommandBucketer&gt;::new (vello_cpu/src/coarse/bucketer.rs:195:29)
  &lt;vello_cpu::coarse::bucketer::CommandBucketer&gt;::from_wh (vello_cpu/src/coarse/bucketer.rs:188:9)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher&gt;::new (vello_cpu/src/dispatch/single_threaded.rs:55:36)
  &lt;vello_cpu::render::RenderContext&gt;::new_with (vello_cpu/src/render.rs:216:24)
  &lt;vello_cpu::render::RenderContext&gt;::new (vello_cpu/src/render.rs:197:9)
  hayro::render (hayro/src/lib.rs:238:19)
  repro_vello_common_src_util_rs_238_60_3fc766::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new(PdfData::from(include_bytes!(
    "data_0.bin").to_vec())).unwrap();
let pages = pdf.pages();
let page = pages.get(0usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings::default(), &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/util.rs:237:59:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;vello_common::geometry::RectU16 as vello_common::util::RectExt&gt;::snap_to_tile_coordinates (vello_common/src/util.rs:237:59)
  &lt;vello_cpu::coarse::bucketer::CommandBucketer&gt;::new (vello_cpu/src/coarse/bucketer.rs:195:29)
  &lt;vello_cpu::coarse::bucketer::CommandBucketer&gt;::from_wh (vello_cpu/src/coarse/bucketer.rs:188:9)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher&gt;::new (vello_cpu/src/dispatch/single_threaded.rs:55:36)
  &lt;vello_cpu::render::RenderContext&gt;::new_with (vello_cpu/src/render.rs:216:24)
  &lt;vello_cpu::render::RenderContext&gt;::new (vello_cpu/src/render.rs:197:9)
  hayro::render (hayro/src/lib.rs:238:19)
  repro_vello_common_src_util_rs_237_59_b645fa::main (src/main.rs:13:13)</pre></td></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new_with_password(PdfData::from(include_bytes!(
    "data_0.bin").to_vec()), "").unwrap();
let pages = pdf.pages();
let page = pages.get(1usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings { force_image_interpolation: false },
    &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:654:21:
attempt to add with overflow
backtrace:
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa_impl::&lt;fearless_simd::generated::avx512::Avx512&gt; (vello_common/src/tile.rs:654:21)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3} (vello_common/src/tile.rs:521:39)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::vectorize_avx512::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:388:13)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:390:18)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa (vello_common/src/tile.rs:521:9)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_with_clip (vello_common/src/strip_generator.rs:178:14)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_stroked_path::&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/strip_generator.rs:167:14)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0} (vello_cpu/src/dispatch/single_threaded.rs:331:33)
  &lt;vello_common::viewport::ViewportState&gt;::with_generator_and_clip::&lt;(), &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0}&gt; (vello_common/src/viewport.rs:62:9)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path (vello_cpu/src/dispatch/single_threaded.rs:330:14)
  &lt;vello_cpu::render::RenderContext&gt;::stroke_path::{closure#0} (vello_cpu/src/render.rs:322:28)
  &lt;vello_cpu::render::RenderContext&gt;::with_optional_filter::&lt;&lt;vello_cpu::render::RenderContext&gt;::stroke_path::{closure#0}&gt; (vello_cpu/src/render.rs:931:13)</pre></td></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new(PdfData::from(include_bytes!(
    "data_0.bin").to_vec())).unwrap();
let pages = pdf.pages();
let page = pages.get(0usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings { force_image_interpolation: true },
    &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:781:41:
attempt to add with overflow
backtrace:
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa_impl::&lt;fearless_simd::generated::avx512::Avx512&gt;::{closure#1} (vello_common/src/tile.rs:781:41)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa_impl::&lt;fearless_simd::generated::avx512::Avx512&gt; (vello_common/src/tile.rs:916:29)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3} (vello_common/src/tile.rs:521:39)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::vectorize_avx512::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:388:13)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:390:18)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa (vello_common/src/tile.rs:521:9)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_with_clip (vello_common/src/strip_generator.rs:178:14)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_stroked_path::&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/strip_generator.rs:167:14)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0} (vello_cpu/src/dispatch/single_threaded.rs:331:33)
  &lt;vello_common::viewport::ViewportState&gt;::with_generator_and_clip::&lt;(), &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0}&gt; (vello_common/src/viewport.rs:62:9)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path (vello_cpu/src/dispatch/single_threaded.rs:330:14)
  &lt;vello_cpu::render::RenderContext&gt;::stroke_path::{closure#0} (vello_cpu/src/render.rs:322:28)</pre></td></tr>
<tr><td valign="top"><pre>let pdf = Pdf::new(PdfData::from(include_bytes!(
    "data_0.bin").to_vec())).unwrap();
let pages = pdf.pages();
let page = pages.get(0usize).unwrap();
let render_cache = RenderCache::new();
let _ = hayro::render(page, &amp;render_cache, &amp;InterpreterSettings::default(),
    &amp;RenderSettings { force_image_interpolation: false },
    &amp;PixmapSettings::default());</pre></td><td valign="top"><pre>thread 'main' panicked at vello_common/src/tile.rs:667:21:
attempt to add with overflow
backtrace:
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa_impl::&lt;fearless_simd::generated::avx512::Avx512&gt; (vello_common/src/tile.rs:667:21)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3} (vello_common/src/tile.rs:521:39)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::vectorize_avx512::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:388:13)
  &lt;fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd&gt;::vectorize::&lt;&lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa::{closure#3}, bool&gt; (fearless_simd/src/generated/avx512.rs:390:18)
  &lt;vello_common::tile::Tiles&gt;::make_tiles_analytic_aa (vello_common/src/tile.rs:521:9)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_with_clip (vello_common/src/strip_generator.rs:178:14)
  &lt;vello_common::strip_generator::StripGenerator&gt;::generate_stroked_path::&lt;&amp;kurbo::bezpath::BezPath&gt; (vello_common/src/strip_generator.rs:167:14)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0} (vello_cpu/src/dispatch/single_threaded.rs:331:33)
  &lt;vello_common::viewport::ViewportState&gt;::with_generator_and_clip::&lt;(), &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path::{closure#0}&gt; (vello_common/src/viewport.rs:62:9)
  &lt;vello_cpu::dispatch::single_threaded::SingleThreadedDispatcher as vello_cpu::dispatch::Dispatcher&gt;::stroke_path (vello_cpu/src/dispatch/single_threaded.rs:330:14)
  &lt;vello_cpu::render::RenderContext&gt;::stroke_path::{closure#0} (vello_cpu/src/render.rs:322:28)
  &lt;vello_cpu::render::RenderContext&gt;::with_optional_filter::&lt;&lt;vello_cpu::render::RenderContext&gt;::stroke_path::{closure#0}&gt; (vello_cpu/src/render.rs:931:13)</pre></td></tr>
</table>
