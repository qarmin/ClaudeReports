# bytesize - crashes

Source: https://github.com/bytesize-rs/bytesize @ 66a3715e33369e99cb428bcd5aee32291f8e0a94

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ByteSize::eib(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:246:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::eib (bytesize/src/lib.rs:246:18)
  repro_bytesize_src_lib_rs_246_18_c02731::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::tib::&lt;u64&gt;(100000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:143:5:
attempt to multiply with overflow
backtrace:
  bytesize::tib::&lt;u64&gt; (bytesize/src/lib.rs:143:5)
  repro_bytesize_src_lib_rs_143_5_7cf4a9::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;bytesize::ByteSize as ::core::ops::Sub&lt;bytesize::ByteSize&gt;&gt;::sub(
    ByteSize::kib(0u64), ByteSize::kib(1u64));</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:473:18:
attempt to subtract with overflow
backtrace:
  &lt;bytesize::ByteSize as core::ops::arith::Sub&gt;::sub (bytesize/src/lib.rs:473:18)
  repro_bytesize_src_lib_rs_473_18_24cf7a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::gib::&lt;u64&gt;(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:133:5:
attempt to multiply with overflow
backtrace:
  bytesize::gib::&lt;u64&gt; (bytesize/src/lib.rs:133:5)
  repro_bytesize_src_lib_rs_133_5_a404ac::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::mb::&lt;u64&gt;(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:118:5:
attempt to multiply with overflow
backtrace:
  bytesize::mb::&lt;u64&gt; (bytesize/src/lib.rs:118:5)
  repro_bytesize_src_lib_rs_118_5_4cf174::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::pb::&lt;u64&gt;(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:148:5:
attempt to multiply with overflow
backtrace:
  bytesize::pb::&lt;u64&gt; (bytesize/src/lib.rs:148:5)
  repro_bytesize_src_lib_rs_148_5_9937e9::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::pb(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:228:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::pb (bytesize/src/lib.rs:228:18)
  repro_bytesize_src_lib_rs_228_18_946d94::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;bytesize::Unit as ::core::ops::Mul&lt;u64&gt;&gt;::mul(Unit::GigaByte,
    10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/parse.rs:164:13:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::parse::Unit as core::ops::arith::Mul&lt;u64&gt;&gt;::mul (bytesize/src/parse.rs:164:13)
  repro_bytesize_src_parse_rs_164_13_987b77::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::tb::&lt;u64&gt;(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:138:5:
attempt to multiply with overflow
backtrace:
  bytesize::tb::&lt;u64&gt; (bytesize/src/lib.rs:138:5)
  repro_bytesize_src_lib_rs_138_5_b9d952::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::mib(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:198:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::mib (bytesize/src/lib.rs:198:18)
  repro_bytesize_src_lib_rs_198_18_39799d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::eb::&lt;u64&gt;(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:158:5:
attempt to multiply with overflow
backtrace:
  bytesize::eb::&lt;u64&gt; (bytesize/src/lib.rs:158:5)
  repro_bytesize_src_lib_rs_158_5_3d3ab1::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::tib(1000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:222:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::tib (bytesize/src/lib.rs:222:18)
  repro_bytesize_src_lib_rs_222_18_9949e1::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::eib::&lt;u64&gt;(16u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:163:5:
attempt to multiply with overflow
backtrace:
  bytesize::eib::&lt;u64&gt; (bytesize/src/lib.rs:163:5)
  repro_bytesize_src_lib_rs_163_5_3d4528::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::pib::&lt;u64&gt;(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:153:5:
attempt to multiply with overflow
backtrace:
  bytesize::pib::&lt;u64&gt; (bytesize/src/lib.rs:153:5)
  repro_bytesize_src_lib_rs_153_5_d9e099::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;bytesize::ByteSize as ::core::ops::Add&lt;bytesize::ByteSize&gt;&gt;::add(
    ByteSize::mib(1u64), ByteSize::b(u64::MAX));</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:418:18:
attempt to add with overflow
backtrace:
  &lt;bytesize::ByteSize as core::ops::arith::Add&gt;::add (bytesize/src/lib.rs:418:18)
  repro_bytesize_src_lib_rs_418_18_899d13::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::mb(100000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:192:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::mb (bytesize/src/lib.rs:192:18)
  repro_bytesize_src_lib_rs_192_18_f73ce5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::gb(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:204:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::gb (bytesize/src/lib.rs:204:18)
  repro_bytesize_src_lib_rs_204_18_d0552f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::mib::&lt;u64&gt;(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:123:5:
attempt to multiply with overflow
backtrace:
  bytesize::mib::&lt;u64&gt; (bytesize/src/lib.rs:123:5)
  repro_bytesize_src_lib_rs_123_5_33e229::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;bytesize::Unit as ::core::ops::Add&lt;u64&gt;&gt;::add(Unit::TebiByte,
    u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/parse.rs:148:13:
attempt to add with overflow
backtrace:
  &lt;bytesize::parse::Unit as core::ops::arith::Add&lt;u64&gt;&gt;::add (bytesize/src/parse.rs:148:13)
  repro_bytesize_src_parse_rs_148_13_f2044c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::gib(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:210:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::gib (bytesize/src/lib.rs:210:18)
  repro_bytesize_src_lib_rs_210_18_e0c922::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::kb(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:108:5:
attempt to multiply with overflow
backtrace:
  bytesize::kb::&lt;u64&gt; (bytesize/src/lib.rs:108:5)
  repro_bytesize_src_lib_rs_108_5_4cb2d5::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::kib(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:186:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::kib (bytesize/src/lib.rs:186:18)
  repro_bytesize_src_lib_rs_186_18_0688d4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::kb(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:180:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::kb (bytesize/src/lib.rs:180:18)
  repro_bytesize_src_lib_rs_180_18_77daa5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::tb(1000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:216:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::tb (bytesize/src/lib.rs:216:18)
  repro_bytesize_src_lib_rs_216_18_0326ae::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::pib(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:234:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::pib (bytesize/src/lib.rs:234:18)
  repro_bytesize_src_lib_rs_234_18_8acad1::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteSize::eb(1000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:240:18:
attempt to multiply with overflow
backtrace:
  &lt;bytesize::ByteSize&gt;::eb (bytesize/src/lib.rs:240:18)
  repro_bytesize_src_lib_rs_240_18_5bef85::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::gb::&lt;u64&gt;(100000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:128:5:
attempt to multiply with overflow
backtrace:
  bytesize::gb::&lt;u64&gt; (bytesize/src/lib.rs:128:5)
  repro_bytesize_src_lib_rs_128_5_dd5ed9::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = bytesize::kib::&lt;u64&gt;(1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bytesize/src/lib.rs:113:5:
attempt to multiply with overflow
backtrace:
  bytesize::kib::&lt;u64&gt; (bytesize/src/lib.rs:113:5)
  repro_bytesize_src_lib_rs_113_5_1747e1::main (src/main.rs:2:13)</pre></td></tr>
</table>
