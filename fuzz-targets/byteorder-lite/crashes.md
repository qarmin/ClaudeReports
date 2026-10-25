# byteorder-lite - crashes

Source: https://github.com/image-rs/byteorder-lite @ 7d44da06a48fe10d7e23efd9a932d18a7a2def63

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u128(
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1809:32:
range end index 16 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u128 (byteorder-lite/src/lib.rs:1809:32)
  repro_byteorder_lite_src_lib_rs_1809_32_e4279f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigEndian::default();</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1670:9:
BigEndian default
backtrace:
  &lt;byteorder_lite::BigEndian as core::default::Default&gt;::default (byteorder-lite/src/lib.rs:1670:9)
  repro_byteorder_lite_src_lib_rs_1670_9_137883::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::
    read_uint128(&amp;vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2089:9:
assertion failed: 1 &lt;= nbytes &amp;&amp; nbytes &lt;= out.len() &amp;&amp; nbytes &lt;= buf.len()
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_uint128 (byteorder-lite/src/lib.rs:2089:9)
  repro_byteorder_lite_src_lib_rs_2089_9_c18c4e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u16_into(&amp;[],
    &amp;mut vec![45u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1918:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u16_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1918_9_39d80b::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u32_into(
    &amp;vec![], &amp;mut vec![0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2137:9:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u32_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2137_9_a9b894::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_uint128(
    &amp;vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1824:9:
assertion failed: 1 &lt;= nbytes &amp;&amp; nbytes &lt;= out.len() &amp;&amp; nbytes &lt;= buf.len()
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_uint128 (byteorder-lite/src/lib.rs:1824:9)
  repro_byteorder_lite_src_lib_rs_1824_9_338e95::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LittleEndian::default();</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1700:9:
LittleEndian default
backtrace:
  &lt;byteorder_lite::LittleEndian as core::default::Default&gt;::default (byteorder-lite/src/lib.rs:1700:9)
  repro_byteorder_lite_src_lib_rs_1700_9_0d56bd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u16(
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2060:31:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u16 (byteorder-lite/src/lib.rs:2060:31)
  repro_byteorder_lite_src_lib_rs_2060_31_4c3b73::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_f64_into(&amp;vec![],
    &amp;mut vec![0.0f64]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1913:9:
assertion `left == right` failed
  left: 0
 right: 8
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_f64_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1913_9_7378eb::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u32_into(&amp;vec![],
    &amp;mut vec![0u32]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1873:9:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u32_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1873_9_17dc17::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i64_into(
    &amp;vec![66u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2162:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i64_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2162_9_d62f01::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u32_into(&amp;[],
    &amp;mut vec![73u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2187:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2187_9_ddc99d::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_f32_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2222:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_f32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2222_9_68293e::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i64_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1948:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1948_9_56aafb::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_f32_into(&amp;[],
    &amp;mut vec![60u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1958:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_f32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1958_9_d98947::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_f64_into(&amp;[],
    &amp;mut vec![60u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2227:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_f64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2227_9_cb0d69::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i32_into(
    &amp;[0i32], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2207:9:
assertion `left == right` failed
  left: 4
 right: 0
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2207_9_fe78ea::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u64_into(
    &amp;vec![], &amp;mut vec![0u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2142:9:
assertion `left == right` failed
  left: 0
 right: 8
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u64_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2142_9_f0749f::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i16_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2202:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i16_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2202_9_015232::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_f64_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1963:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_f64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1963_9_84a8df::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i32_into(&amp;vec![],
    &amp;mut vec![0i32]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1893:9:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i32_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1893_9_7abe48::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i128_into(
    &amp;[], &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2217:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i128_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2217_9_ba0911::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u128_into(
    &amp;[], &amp;mut vec![82u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2197:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u128_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2197_9_5a483e::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i64_into(&amp;vec![],
    &amp;mut vec![0i64]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1898:9:
assertion `left == right` failed
  left: 0
 right: 8
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i64_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1898_9_5e363c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i16_into(&amp;vec![],
    &amp;mut vec![0i16]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1888:9:
assertion `left == right` failed
  left: 0
 right: 2
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i16_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1888_9_dd3448::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i32_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1943:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1943_9_1ac324::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i16_into(
    &amp;vec![], &amp;mut vec![0i16]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2152:9:
assertion `left == right` failed
  left: 0
 right: 2
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i16_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2152_9_4747a3::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u64_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1928:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1928_9_666bcf::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u32_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1923:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u32_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1923_9_300f6d::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i16_into(&amp;[],
    &amp;mut vec![82u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1938:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i16_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1938_9_0ec6a8::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u64_into(&amp;[],
    &amp;mut vec![84u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2192:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2192_9_6d042c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i32_into(
    &amp;vec![], &amp;mut vec![0i32]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2157:9:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i32_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2157_9_a01c69::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i128_into(
    &amp;vec![0u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2167:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_i128_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2167_9_e32bd5::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u16_into(
    &amp;vec![0u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1868:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u16_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1868_9_543d86::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u16_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2182:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_u16_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2182_9_7d6e68::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u16_into(
    &amp;vec![], &amp;mut vec![0u16]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2132:9:
assertion `left == right` failed
  left: 0
 right: 2
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u16_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2132_9_7b3514::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u64_into(
    &amp;vec![116u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1878:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u64_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1878_9_4111ad::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u16(&amp;mut vec![],
    0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1832:12:
range end index 2 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u16 (byteorder-lite/src/lib.rs:1832:12)
  repro_byteorder_lite_src_lib_rs_1832_12_68ae33::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u128_into(
    &amp;vec![0u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1883:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u128_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1883_9_2a3449::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i128_into(&amp;[],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1953:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_i128_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1953_9_49a0bd::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i64_into(
    &amp;[0i64], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2212:9:
assertion `left == right` failed
  left: 8
 right: 0
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_i64_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_2212_9_acae6e::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i128_into(
    &amp;vec![45u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1903:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_i128_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_1903_9_c69429::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u128_into(
    &amp;vec![82u8], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2147:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::read_u128_into (byteorder-lite/src/lib.rs:1767:9)
  repro_byteorder_lite_src_lib_rs_2147_9_58d6f2::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u128_into(&amp;[],
    &amp;mut vec![60u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1933:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u128_into (byteorder-lite/src/lib.rs:1784:9)
  repro_byteorder_lite_src_lib_rs_1933_9_d2a840::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let u128 = &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::read_u128(
    &amp;vec![0u8, 0u8, 0u8, 0u8, 79u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 69u8, 101u8]);
&lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_uint128(
    &amp;mut vec![], u128, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:2124:9:
assertion failed: pack_size128(n) &lt;= nbytes &amp;&amp; nbytes &lt;= 16
backtrace:
  &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::write_uint128 (byteorder-lite/src/lib.rs:2124:9)
  repro_byteorder_lite_src_lib_rs_2124_9_8c5139::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let u128 = &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::
    read_u128(&amp;vec![0u8, 80u8, 78u8, 71u8, 0u8 /* x8 */, 73u8, 72u8, 68u8,
    82u8]);
&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u128(
    &amp;mut vec![], u128);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1847:12:
range end index 16 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_u128 (byteorder-lite/src/lib.rs:1847:12)
  repro_byteorder_lite_src_lib_rs_1847_12_689e80::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>let u128 = &lt;byteorder_lite::LittleEndian as byteorder_lite::ByteOrder&gt;::
    read_u128(&amp;vec![58u8, 0u8, 0u8, 0u8, 108u8, 67u8, 0u8, 0u8, 60u8, 92u8, 0u8,
    0u8, 0u8, 76u8, 0u8, 0u8]);
&lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_uint128(
    &amp;mut vec![], u128, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at byteorder-lite/src/lib.rs:1860:9:
assertion failed: pack_size128(n) &lt;= nbytes &amp;&amp; nbytes &lt;= 16
backtrace:
  &lt;byteorder_lite::BigEndian as byteorder_lite::ByteOrder&gt;::write_uint128 (byteorder-lite/src/lib.rs:1860:9)
  repro_byteorder_lite_src_lib_rs_1860_9_795f0e::main (src/main.rs:3:5)</pre></td></tr>
</table>
