# num-integer - crashes

Source: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i16&gt;(0i16, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:418:1:
can't find a root of degree 0!
backtrace:
  &lt;u16 as num_integer::roots::Roots&gt;::nth_root::go (num-integer/src/roots.rs:245:30)
  &lt;u16 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:309:17)
  &lt;i16 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:138:35)
  num_integer::roots::nth_root::&lt;i16&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_418_1_a40799::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::gcd_lcm::&lt;i64&gt;(i64::MIN, 0i64);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/mod.rs:454:5:
attempt to negate with overflow
backtrace:
  &lt;i64&gt;::abs (core/src/num/int_macros.rs:3635:17)
  &lt;i64 as num_integer::Integer&gt;::gcd (num-integer/src/lib.rs:498:36)
  &lt;i64 as num_integer::Integer&gt;::gcd_lcm (num-integer/src/lib.rs:562:32)
  num_integer::gcd_lcm::&lt;i64&gt; (num-integer/src/lib.rs:435:7)
  repro_num_integer_src_lib_rs_435_7_51d2df::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::sqrt::&lt;i8&gt;(-10i8);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:163:1:
the square root of a negative is imaginary
backtrace:
  &lt;i8 as num_integer::roots::Roots&gt;::sqrt (num-integer/src/roots.rs:147:17)
  num_integer::roots::sqrt::&lt;i8&gt; (num-integer/src/roots.rs:115:7)
  repro_num_integer_src_roots_rs_163_1_1e4522::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::gcd::&lt;i8&gt;(i8::MIN, 0i8);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/mod.rs:379:5:
attempt to negate with overflow
backtrace:
  &lt;i8&gt;::abs (core/src/num/int_macros.rs:3635:17)
  &lt;i8 as num_integer::Integer&gt;::gcd (num-integer/src/lib.rs:498:36)
  num_integer::gcd::&lt;i8&gt; (num-integer/src/lib.rs:423:7)
  repro_num_integer_src_lib_rs_498_36_06bf3f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::div_mod_floor::&lt;i32&gt;(0i32, 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:845:1:
attempt to divide by zero
backtrace:
  &lt;i32 as num_integer::Integer&gt;::div_rem (num-integer/src/lib.rs:592:18)
  &lt;i32 as num_integer::Integer&gt;::div_mod_floor (num-integer/src/lib.rs:472:35)
  num_integer::div_mod_floor::&lt;i32&gt; (num-integer/src/lib.rs:411:7)
  repro_num_integer_src_lib_rs_845_1_ca3d69::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::gcd_lcm::&lt;i16&gt;(10i16, i16::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:844:1:
attempt to multiply with overflow
backtrace:
  &lt;i16 as num_integer::Integer&gt;::gcd_lcm (num-integer/src/lib.rs:564:27)
  num_integer::gcd_lcm::&lt;i16&gt; (num-integer/src/lib.rs:435:7)
  repro_num_integer_src_lib_rs_844_1_ee70a7::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i32&gt;(0i32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:419:1:
can't find a root of degree 0!
backtrace:
  &lt;u32 as num_integer::roots::Roots&gt;::nth_root::go (num-integer/src/roots.rs:245:30)
  &lt;u32 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:309:17)
  &lt;i32 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:138:35)
  num_integer::roots::nth_root::&lt;i32&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_419_1_243c58::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i8&gt;(0i8, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:417:1:
can't find a root of degree 0!
backtrace:
  &lt;u8 as num_integer::roots::Roots&gt;::nth_root::go (num-integer/src/roots.rs:245:30)
  &lt;u8 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:309:17)
  &lt;i8 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:138:35)
  num_integer::roots::nth_root::&lt;i8&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_417_1_73d9d5::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::binomial::&lt;i32&gt;(10000i32, 8i32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:1129:5:
attempt to multiply with overflow
backtrace:
  &lt;i32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  num_integer::multiply_and_divide::&lt;i32&gt; (num-integer/src/lib.rs:1129:5)
  num_integer::binomial::&lt;i32&gt; (num-integer/src/lib.rs:1164:13)
  repro_num_integer_src_lib_rs_1129_5_bd728b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i32&gt;(-1i32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:165:1:
even roots of a negative are imaginary
backtrace:
  &lt;i32 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:140:21)
  num_integer::roots::nth_root::&lt;i32&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_165_1_1b1b79::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::div_rem::&lt;i8&gt;(0i8, 0i8);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:843:1:
attempt to divide by zero
backtrace:
  &lt;i8 as num_integer::Integer&gt;::div_rem (num-integer/src/lib.rs:592:18)
  num_integer::div_rem::&lt;i8&gt; (num-integer/src/lib.rs:396:7)
  repro_num_integer_src_lib_rs_843_1_a0c2a4::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::sqrt::&lt;i64&gt;(-1000000000000000000i64);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:166:1:
the square root of a negative is imaginary
backtrace:
  &lt;i64 as num_integer::roots::Roots&gt;::sqrt (num-integer/src/roots.rs:147:17)
  num_integer::roots::sqrt::&lt;i64&gt; (num-integer/src/roots.rs:115:7)
  repro_num_integer_src_roots_rs_166_1_00cf72::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::binomial::&lt;i16&gt;(0i16, i16::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:1155:12:
attempt to subtract with overflow
backtrace:
  &lt;i16 as core::ops::arith::Sub&gt;::sub (core/src/ops/arith.rs:216:45)
  num_integer::binomial::&lt;i16&gt; (num-integer/src/lib.rs:1155:12)
  repro_num_integer_src_lib_rs_1155_12_6281ea::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::lcm::&lt;i64&gt;(i64::MIN, 0i64);</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/mod.rs:454:5:
attempt to negate with overflow
backtrace:
  &lt;i64&gt;::abs (core/src/num/int_macros.rs:3635:17)
  &lt;i64 as num_integer::Integer&gt;::gcd (num-integer/src/lib.rs:498:36)
  &lt;i64 as num_integer::Integer&gt;::gcd_lcm (num-integer/src/lib.rs:562:32)
  &lt;i64 as num_integer::Integer&gt;::lcm (num-integer/src/lib.rs:552:22)
  num_integer::lcm::&lt;i64&gt; (num-integer/src/lib.rs:428:7)
  repro_num_integer_src_lib_rs_552_22_cc4487::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::lcm::&lt;i64&gt;(-1000000000000000000i64, i64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/lib.rs:846:1:
attempt to multiply with overflow
backtrace:
  &lt;i64 as num_integer::Integer&gt;::gcd_lcm (num-integer/src/lib.rs:564:27)
  &lt;i64 as num_integer::Integer&gt;::lcm (num-integer/src/lib.rs:552:22)
  num_integer::lcm::&lt;i64&gt; (num-integer/src/lib.rs:428:7)
  repro_num_integer_src_lib_rs_846_1_a69d85::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i16&gt;(-1i16, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:164:1:
even roots of a negative are imaginary
backtrace:
  &lt;i16 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:140:21)
  num_integer::roots::nth_root::&lt;i16&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_164_1_d5228d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::nth_root::&lt;i64&gt;(0i64, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-integer/src/roots.rs:420:1:
can't find a root of degree 0!
backtrace:
  &lt;u64 as num_integer::roots::Roots&gt;::nth_root::go (num-integer/src/roots.rs:245:30)
  &lt;u64 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:309:17)
  &lt;i64 as num_integer::roots::Roots&gt;::nth_root (num-integer/src/roots.rs:138:35)
  num_integer::roots::nth_root::&lt;i64&gt; (num-integer/src/roots.rs:129:7)
  repro_num_integer_src_roots_rs_420_1_a54c7f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_integer::lcm::&lt;i16&gt;(i16::MIN, i16::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at num-traits/src/sign.rs:75:1:
attempt to negate with overflow
backtrace:
  &lt;i16 as num_traits::sign::Signed&gt;::abs (num-traits/src/sign.rs:49:41)
  &lt;i16 as num_integer::Integer&gt;::gcd (num-integer/src/lib.rs:513:41)
  &lt;i16 as num_integer::Integer&gt;::gcd_lcm (num-integer/src/lib.rs:562:32)
  &lt;i16 as num_integer::Integer&gt;::lcm (num-integer/src/lib.rs:552:22)
  num_integer::lcm::&lt;i16&gt; (num-integer/src/lib.rs:428:7)
  repro_num_traits_src_sign_rs_75_1_964925::main (src/main.rs:2:13)</pre></td></tr>
</table>
