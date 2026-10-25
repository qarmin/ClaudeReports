# tendril - crashes

Source: https://github.com/servo/html5ever @ 7760920edff08e6dfd4b62affee39d8084e9dc45

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1316070960u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_7a3a21::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(2010656208u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_c4f1e4::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut incomplete_utf8 = IncompleteUtf8 { buffer: [0u8, 0u8, 1u8, 0u8],
    buffer_len: 10u8 };
let _ = incomplete_utf8.try_to_complete_codepoint(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at tendril/src/utf8_decode.rs:95:45:
range start index 10 out of range for slice of length 4
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;[u8; 4] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/array/mod.rs:403:9)
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_complete_offsets (tendril/src/utf8_decode.rs:95:45)
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_to_complete_codepoint (tendril/src/utf8_decode.rs:141:50)
  repro_tendril_src_utf8_decode_rs_95_45_508fe0::main (src/main.rs:5:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut incomplete_utf8 = IncompleteUtf8 { buffer: [0u8, 0u8, 255u8, 0u8],
    buffer_len: 3u8 };
let _ = incomplete_utf8.try_to_complete_codepoint(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at tendril/src/utf8_decode.rs:108:80:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_complete_offsets (tendril/src/utf8_decode.rs:108:80)
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_to_complete_codepoint (tendril/src/utf8_decode.rs:141:50)
  repro_tendril_src_utf8_decode_rs_108_80_098d87::main (src/main.rs:5:29)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::ASCII,
    tendril::NonAtomic&gt;&gt;::with_capacity(2134126441u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_ascii_tendril_n_4e0124::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt; as ::core::convert::From&lt;tendril::SendTendril&lt;tendril::
    fmt::Bytes&gt;&gt;&gt;::from(&lt;tendril::Tendril&lt;tendril::fmt::ASCII,
    tendril::NonAtomic&gt;&gt;::with_capacity(
    1537703420u32).into_bytes().into_send());</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_8ddf40::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::with_capacity(1537130801u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_cf9d18::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut incomplete_utf8 = IncompleteUtf8 { buffer: [255u8, 0u8, 0u8, 0u8],
    buffer_len: 2u8 };
let _ = incomplete_utf8.try_to_complete_codepoint(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at tendril/src/utf8_decode.rs:116:34:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_complete_offsets (tendril/src/utf8_decode.rs:116:34)
  &lt;tendril::utf8_decode::IncompleteUtf8&gt;::try_to_complete_codepoint (tendril/src/utf8_decode.rs:141:50)
  repro_tendril_src_utf8_decode_rs_116_34_2d9b30::main (src/main.rs:5:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut tendril = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::new();
tendril.reserve(1747292696u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::force_reserve&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:599)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::reserve&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:589)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_4db687::main (src/main.rs:3)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut tendril = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt; as ::core::convert::From&lt;tendril::SendTendril&lt;tendril::
    fmt::Bytes&gt;&gt;&gt;::from(&lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::new().into_send());
tendril.reserve(1620049837u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::force_reserve&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:599)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::reserve&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:589)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_6f5811::main (src/main.rs:3)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1254267382u32);
let _ = tendril.as_bytes();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_ad5773::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::with_capacity(1664946732u32);
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::Bytes&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_bytes_tendril_n_ecc87a::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt; as ::core::convert::From&lt;tendril::SendTendril&lt;tendril::
    fmt::WTF8&gt;&gt;&gt;::from(&lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(
    2006686492u32).into_superset::&lt;tendril::fmt::WTF8&gt;().into_send());</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_be8f5f::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::with_capacity(0u32);
let tendril2 = tendril.try_reinterpret::&lt;tendril::fmt::UTF8&gt;().unwrap();
let mut tendril3 = tendril2.into_superset::&lt;tendril::fmt::WTF8&gt;();
tendril3.reserve(2071774921u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::force_reserve&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:599)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::reserve&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:589)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_478d8b::main (src/main.rs:5)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt; as ::core::convert::From&lt;tendril::SendTendril&lt;tendril::
    fmt::UTF8&gt;&gt;&gt;::from(&lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1776097792u32).into_send());</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_499a27::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1961106048u32);
let _ = tendril.try_as_subset::&lt;tendril::fmt::UTF8&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_1e4eb4::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1170330663u32);
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::Bytes&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_0625ab::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::WTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1957461737u32);
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::ASCII&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::WTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_wtf8_tendril_no_f3cc25::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut tendril = &lt;tendril::Tendril&lt;tendril::fmt::ASCII,
    tendril::NonAtomic&gt;&gt;::with_capacity(0u32);
tendril.reserve(1195885177u32);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::force_reserve&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:599)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::reserve&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:589)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_ascii_tendril_n_844261::main (src/main.rs:3)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1573871419u32);
let _ = tendril.as_superset::&lt;tendril::fmt::WTF8&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_804413::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::with_capacity(
    2004863132u32).try_reinterpret::&lt;tendril::fmt::UTF8&gt;().unwrap();
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::ASCII&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::Bytes, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_75d5c2::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::with_capacity(1160800565u32);
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::UTF8&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::UTF8, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_utf8_tendril_no_acda0a::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::ASCII,
    tendril::NonAtomic&gt;&gt;::with_capacity(1869478941u32);
let _ = tendril.try_reinterpret_view::&lt;tendril::fmt::WTF8&gt;();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483664 bytes failed
backtrace:
  tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;::grow&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt; (tendril/src/buf32.rs:97)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::make_owned_with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:1077)
  tendril::tendril::Tendril&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt;::with_capacity&lt;tendril::fmt::ASCII, tendril::tendril::NonAtomic&gt; (tendril/src/tendril.rs:574)
  repro_out_of_memory_in_tendril_tendril_tendril_fmt_ascii_tendril_n_85e3f5::main (src/main.rs:2)
  (17 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;tendril::Tendril&lt;tendril::fmt::Bytes,
    tendril::NonAtomic&gt;&gt;::with_capacity(u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at tendril/src/buf32.rs:95:59:
tendril: overflow in buffer arithmetic
backtrace:
  &lt;tendril::buf32::Buf32&lt;tendril::tendril::Header&lt;tendril::tendril::NonAtomic&gt;&gt;&gt;::grow (tendril/src/buf32.rs:95:59)
  &lt;tendril::tendril::Tendril&lt;tendril::fmt::Bytes&gt;&gt;::make_owned_with_capacity (tendril/src/tendril.rs:1077:13)
  &lt;tendril::tendril::Tendril&lt;tendril::fmt::Bytes&gt;&gt;::with_capacity (tendril/src/tendril.rs:574:19)
  repro_tendril_src_buf32_rs_95_59_3b5892::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let tendril = &lt;tendril::Tendril&lt;tendril::fmt::UTF8,
    tendril::NonAtomic&gt;&gt;::from_char('a');
let mut tendril2 = tendril.try_reinterpret::&lt;tendril::fmt::WTF8&gt;().unwrap();
tendril2.reserve(u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at tendril/src/tendril.rs:596:60:
tendril: overflow in buffer arithmetic
backtrace:
  &lt;tendril::tendril::Tendril&lt;tendril::fmt::WTF8&gt;&gt;::force_reserve (tendril/src/tendril.rs:596:60)
  &lt;tendril::tendril::Tendril&lt;tendril::fmt::WTF8&gt;&gt;::reserve (tendril/src/tendril.rs:589:18)
  repro_tendril_src_tendril_rs_596_60_ddf3c0::main (src/main.rs:4:14)</pre></td></tr>
</table>
