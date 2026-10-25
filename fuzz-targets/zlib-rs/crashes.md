# zlib-rs - crashes

Source: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Deflate::new(0i32, false, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/deflate.rs:137:9:
assertion `left == right` failed
  left: StreamError
 right: Ok
backtrace:
  &lt;zlib_rs::deflate::DeflateStream&gt;::new (zlib-rs/src/deflate.rs:137:9)
  &lt;zlib_rs::stable::Deflate&gt;::new_with_config (zlib-rs/src/stable.rs:319:20)
  &lt;zlib_rs::stable::Deflate&gt;::new (zlib-rs/src/stable.rs:308:9)
  repro_zlib_rs_src_deflate_rs_137_9_ba5d10::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut z_stream = zlib_rs::c_api::z_stream::default();
let _ = zlib_rs::inflate::back_init(&amp;mut z_stream,
    InflateConfig { window_bits: 1000i32 }, Window::empty());</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/inflate/infback.rs:18:16:
attempt to shift left with overflow
backtrace:
  zlib_rs::inflate::infback::back_init (zlib-rs/src/inflate/infback.rs:18:16)
  repro_zlib_rs_src_inflate_infback_rs_18_16_0ac9c8::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut crc32_fold = Crc32Fold::new();
crc32_fold.fold(&amp;vec![], 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/crc32/pclmulqdq.rs:252:9:
assertion failed: src.len() &gt;= 31 || !first
backtrace:
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold_help::&lt;false&gt; (zlib-rs/src/crc32/pclmulqdq.rs:252:9)
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold (zlib-rs/src/crc32/pclmulqdq.rs:43:23)
  &lt;zlib_rs::crc32::Crc32Fold&gt;::fold (zlib-rs/src/crc32.rs:78:47)
  repro_zlib_rs_src_crc32_pclmulqdq_rs_252_9_c2b5b3::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut inflate_stream = InflateStream::new(InflateConfig::default());
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);
let _ = zlib_rs::inflate::sync_point(&amp;mut inflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::inflate::sync_point (src/inflate.rs:2538)
  repro_sigsegv_in_zlib_rs_inflate_sync_point_4be173::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = deflate_stream.bits_used();</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::DeflateStream::bits_used (src/deflate.rs:130)
  repro_sigsegv_in_zlib_rs_deflate_deflatestream_bits_used_eec98c::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::default());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::end (src/deflate.rs:729)
  repro_sigsegv_in_zlib_rs_deflate_end_019187::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ReturnCode::from(10i32);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/lib.rs:242:21:
invalid return code 10
backtrace:
  &lt;zlib_rs::ReturnCode as core::convert::From&lt;i32&gt;&gt;::from (zlib-rs/src/lib.rs:242:21)
  repro_zlib_rs_src_lib_rs_242_21_04037d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut crc32_fold = Crc32Fold::new();
crc32_fold.fold_copy(&amp;mut vec![], &amp;vec![32u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/crc32/pclmulqdq.rs:260:13:
assertion `left == right` failed: dst and src must be the same length
  left: 0
 right: 1
backtrace:
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold_help::&lt;true&gt; (zlib-rs/src/crc32/pclmulqdq.rs:260:13)
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold_copy (zlib-rs/src/crc32/pclmulqdq.rs:47:23)
  &lt;zlib_rs::crc32::Crc32Fold&gt;::fold_copy (zlib-rs/src/crc32.rs:105:39)
  repro_zlib_rs_src_crc32_pclmulqdq_rs_260_13_a5a021::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let window = Window::empty();
let _ = window.buffer_size();</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/inflate/window.rs:41:9:
assertion failed: self.buf.len().is_power_of_two()
backtrace:
  &lt;zlib_rs::inflate::window::Window&gt;::buffer_size (zlib-rs/src/inflate/window.rs:41:9)
  repro_zlib_rs_src_inflate_window_rs_41_9_d4a7c1::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = deflate_stream.pending();</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::DeflateStream::pending (src/deflate.rs:123)
  repro_sigsegv_in_zlib_rs_deflate_deflatestream_pending_ae8f40::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = zlib_rs::deflate::tune(&amp;mut deflate_stream, 0usize, 0usize, 0usize,
    0usize);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::tune (src/deflate.rs:824)
  repro_sigsegv_in_zlib_rs_deflate_tune_9ccc65::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut inflate_stream = InflateStream::new(
    InflateConfig { window_bits: 0i32 });
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);
let _ = zlib_rs::inflate::prime(&amp;mut inflate_stream, 1i32, 0i32);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::inflate::prime (src/inflate.rs:2165)
  repro_sigsegv_in_zlib_rs_inflate_prime_48f33b::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut crc32_fold = Crc32Fold::new();
crc32_fold.fold(&amp;vec![0u8, 0u8 /* x29 */, 0u8], 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/crc32/pclmulqdq.rs:257:9:
assertion failed: src.len() &gt;= 64 || !first
backtrace:
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold_help::&lt;false&gt; (zlib-rs/src/crc32/pclmulqdq.rs:257:9)
  &lt;zlib_rs::crc32::pclmulqdq::Accumulator&gt;::fold (zlib-rs/src/crc32/pclmulqdq.rs:43:23)
  &lt;zlib_rs::crc32::Crc32Fold&gt;::fold (zlib-rs/src/crc32.rs:78:47)
  repro_zlib_rs_src_crc32_pclmulqdq_rs_257_9_bd5935::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::default());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = zlib_rs::deflate::reset(&amp;mut deflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::pending::Pending::reset_keep (src/deflate/pending.rs:19)
  zlib_rs::deflate::reset_keep (src/deflate.rs:763)
  zlib_rs::deflate::reset (src/deflate.rs:746)
  repro_sigsegv_in_zlib_rs_deflate_reset_97214f::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut inflate_stream = InflateStream::new(InflateConfig::default());
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);
let _ = zlib_rs::inflate::codes_used(&amp;inflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::inflate::codes_used (src/inflate.rs:2373)
  repro_sigsegv_in_zlib_rs_inflate_codes_used_3ba793::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let inflate_config = InflateConfig::default();
let mut inflate_stream = InflateStream::new(inflate_config);
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::inflate::end (src/inflate.rs:2650)
  repro_sigsegv_in_zlib_rs_inflate_end_f58c23::main (src/main.rs:8)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::default());
let _ = zlib_rs::deflate::end(&amp;mut deflate_stream);
let _ = zlib_rs::deflate::set_dictionary(&amp;mut deflate_stream, &amp;vec![]);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::deflate::set_dictionary (src/deflate.rs:502)
  repro_sigsegv_in_zlib_rs_deflate_set_dictionary_946fb5::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
let _ = zlib_rs::deflate::prime(&amp;mut deflate_stream, 10000i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at zlib-rs/src/deflate.rs:568:5:
zlib only supports up to 16 bits here
backtrace:
  zlib_rs::deflate::prime (zlib-rs/src/deflate.rs:568:5)
  repro_zlib_rs_src_deflate_rs_568_5_b13c32::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut inflate_stream = InflateStream::new(InflateConfig::default());
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);
let _ = zlib_rs::compress_slice(&amp;mut vec![0u8], &amp;vec![],
    DeflateConfig::default());
let _ = zlib_rs::inflate::end(&amp;mut inflate_stream);</pre></td><td valign="top"><pre>SIGSEGV: Program received signal SIGSEGV, Segmentation fault
backtrace:
  zlib_rs::inflate::end (src/inflate.rs:2650)
  repro_sigsegv_in_zlib_rs_compress_slice_da7cbb::main (src/main.rs:9)</pre></td></tr>
<tr><td valign="top"><pre>let _ = zlib_rs::crc32::crc32_combine_op(0u32, 0u32, 0u32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  zlib_rs::crc32::combine::multmodp (src/crc32/combine.rs:38)
  zlib_rs::crc32::combine::crc32_combine_op (zlib-rs/src/crc32/combine.rs:14)
  repro_timeout_in_zlib_rs_crc32_crc32_combine_op_82506d::main (src/main.rs:2)</pre></td></tr>
</table>
