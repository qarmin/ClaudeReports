# byteorder - crashes

Source: https://github.com/BurntSushi/byteorder @ 5a82625fae462e8ba64cec8146b24a372b4d75c6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_uint(&amp;vec![],
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2096:9:
assertion failed: 1 &lt;= nbytes &amp;&amp; nbytes &lt;= out.len() &amp;&amp; nbytes &lt;= buf.len()
backtrace:
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_uint (byteorder/src/lib.rs:2096:9)
  repro_byteorder_src_lib_rs_2096_9_7ebd93::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u16(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2075:31:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u16 (byteorder/src/lib.rs:2075:31)
  repro_byteorder_src_lib_rs_2075_31_84b64e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigEndian::default();</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1769:9:
BigEndian default
backtrace:
  &lt;byteorder::BigEndian as core::default::Default&gt;::default (byteorder/src/lib.rs:1769:9)
  repro_byteorder_src_lib_rs_1769_9_540cbb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_uint128(&amp;vec![],
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1923:9:
assertion failed: 1 &lt;= nbytes &amp;&amp; nbytes &lt;= out.len() &amp;&amp; nbytes &lt;= buf.len()
backtrace:
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_uint128 (byteorder/src/lib.rs:1923:9)
  repro_byteorder_src_lib_rs_1923_9_d506c8::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LittleEndian::default();</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1799:9:
LittleEndian default
backtrace:
  &lt;byteorder::LittleEndian as core::default::Default&gt;::default (byteorder/src/lib.rs:1799:9)
  repro_byteorder_src_lib_rs_1799_9_a361f0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u64(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1903:31:
range end index 8 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u64 (byteorder/src/lib.rs:1903:31)
  repro_byteorder_src_lib_rs_1903_31_8300ae::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u128(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2090:32:
range end index 16 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u128 (byteorder/src/lib.rs:2090:32)
  repro_byteorder_src_lib_rs_2090_32_024237::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u16_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2171:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u16_into (byteorder/src/lib.rs:1883:9)
  repro_byteorder_src_lib_rs_2171_9_b58750::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u128_into(&amp;vec![66u8],
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1994:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u128_into (byteorder/src/lib.rs:1866:9)
  repro_byteorder_src_lib_rs_1994_9_ed54ff::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u16_into(&amp;vec![0u8],
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2151:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u16_into (byteorder/src/lib.rs:1866:9)
  repro_byteorder_src_lib_rs_2151_9_449a0f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_uint(&amp;mut vec![], 0u64,
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1951:9:
assertion failed: pack_size(n) &lt;= nbytes &amp;&amp; nbytes &lt;= 8
backtrace:
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_uint (byteorder/src/lib.rs:1951:9)
  repro_byteorder_src_lib_rs_1951_9_6adc1f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u16_into(&amp;vec![],
    &amp;mut vec![0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1979:9:
assertion `left == right` failed
  left: 0
 right: 2
backtrace:
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u16_into (byteorder/src/lib.rs:1866:9)
  repro_byteorder_src_lib_rs_1979_9_9c9860::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u128_into(&amp;vec![123u8],
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2166:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::read_u128_into (byteorder/src/lib.rs:1866:9)
  repro_byteorder_src_lib_rs_2166_9_5ac40a::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u16(&amp;mut vec![], 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2111:12:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u16 (byteorder/src/lib.rs:2111:12)
  repro_byteorder_src_lib_rs_2111_12_e4536b::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_u16(&amp;mut vec![], 1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1931:12:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_u16 (byteorder/src/lib.rs:1931:12)
  repro_byteorder_src_lib_rs_1931_12_b9c363::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u64(&amp;mut vec![], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:2121:12:
range end index 8 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;byteorder::LittleEndian as byteorder::ByteOrder&gt;::write_u64 (byteorder/src/lib.rs:2121:12)
  repro_byteorder_src_lib_rs_2121_12_6ca617::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let u128 = &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::read_u128(&amp;vec![73u8,
    73u8, 42u8, 0u8 /* x12 */, 0u8]);
&lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_uint128(&amp;mut vec![], u128,
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder/src/lib.rs:1965:9:
assertion failed: pack_size128(n) &lt;= nbytes &amp;&amp; nbytes &lt;= 16
backtrace:
  &lt;byteorder::BigEndian as byteorder::ByteOrder&gt;::write_uint128 (byteorder/src/lib.rs:1965:9)
  repro_byteorder_src_lib_rs_1965_9_48450c::main (src/main.rs:3:5)</pre></td></tr>
</table>
