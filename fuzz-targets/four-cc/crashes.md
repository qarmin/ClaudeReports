# four-cc - crashes

Source: https://github.com/dholroyd/four-cc @ c6a750a03f3260e3fac15643a6e0efe4fd3cae0e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;four_cc::FourCC as ::core::str::FromStr&gt;::from_str("aa");</pre></td><td valign="top"><pre>thread 'main' panicked at four-cc/src/lib.rs:129:33:
index out of bounds: the len is 2 but the index is 2
backtrace:
  &lt;four_cc::FourCC as core::convert::From&lt;&amp;[u8]&gt;&gt;::from (four-cc/src/lib.rs:129:33)
  &lt;&amp;[u8] as core::convert::Into&lt;four_cc::FourCC&gt;&gt;::into (core/src/convert/mod.rs:781:9)
  &lt;four_cc::FourCC as core::str::traits::FromStr&gt;::from_str (four-cc/src/lib.rs:227:25)
  repro_four_cc_src_lib_rs_129_33_a948c8::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;four_cc::FourCC as ::core::str::FromStr&gt;::from_str("a");</pre></td><td valign="top"><pre>thread 'main' panicked at four-cc/src/lib.rs:129:25:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;four_cc::FourCC as core::convert::From&lt;&amp;[u8]&gt;&gt;::from (four-cc/src/lib.rs:129:25)
  &lt;&amp;[u8] as core::convert::Into&lt;four_cc::FourCC&gt;&gt;::into (core/src/convert/mod.rs:781:9)
  &lt;four_cc::FourCC as core::str::traits::FromStr&gt;::from_str (four-cc/src/lib.rs:227:25)
  repro_four_cc_src_lib_rs_129_25_22b39a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;four_cc::FourCC as ::core::str::FromStr&gt;::from_str("\u{feff}");</pre></td><td valign="top"><pre>thread 'main' panicked at four-cc/src/lib.rs:129:41:
index out of bounds: the len is 3 but the index is 3
backtrace:
  &lt;four_cc::FourCC as core::convert::From&lt;&amp;[u8]&gt;&gt;::from (four-cc/src/lib.rs:129:41)
  &lt;&amp;[u8] as core::convert::Into&lt;four_cc::FourCC&gt;&gt;::into (core/src/convert/mod.rs:781:9)
  &lt;four_cc::FourCC as core::str::traits::FromStr&gt;::from_str (four-cc/src/lib.rs:227:25)
  repro_four_cc_src_lib_rs_129_41_dc95bb::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;four_cc::FourCC as ::core::str::FromStr&gt;::from_str("");</pre></td><td valign="top"><pre>thread 'main' panicked at four-cc/src/lib.rs:129:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;four_cc::FourCC as core::convert::From&lt;&amp;[u8]&gt;&gt;::from (four-cc/src/lib.rs:129:17)
  &lt;&amp;[u8] as core::convert::Into&lt;four_cc::FourCC&gt;&gt;::into (core/src/convert/mod.rs:781:9)
  &lt;four_cc::FourCC as core::str::traits::FromStr&gt;::from_str (four-cc/src/lib.rs:227:25)
  repro_four_cc_src_lib_rs_129_17_390afb::main (src/main.rs:2:13)</pre></td></tr>
</table>
