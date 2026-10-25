# num-traits - crashes

Source: https://github.com/rust-num/num-traits @ 1166177ec398601d6dd4d88891cf7674f3978888

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = num_traits::pow::&lt;u16&gt;(100u16, 15usize);</pre></td><td valign="top"><pre>thread 'main' panicked at num-traits/src/pow.rs:197:19:
attempt to multiply with overflow
backtrace:
  &lt;u16 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  num_traits::pow::pow::&lt;u16&gt; (num-traits/src/pow.rs:197:19)
  repro_num_traits_src_pow_rs_197_19_9a974d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_traits::pow::&lt;u16&gt;(10000u16, 1000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at num-traits/src/pow.rs:185:16:
attempt to multiply with overflow
backtrace:
  &lt;u16 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  num_traits::pow::pow::&lt;u16&gt; (num-traits/src/pow.rs:185:16)
  repro_num_traits_src_pow_rs_185_16_52a2d7::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_traits::pow::&lt;usize&gt;(256usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at num-traits/src/pow.rs:195:16:
attempt to multiply with overflow
backtrace:
  &lt;usize as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  num_traits::pow::pow::&lt;usize&gt; (num-traits/src/pow.rs:195:16)
  repro_num_traits_src_pow_rs_195_16_91da23::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = num_traits::abs_sub::&lt;i32&gt;(0i32, i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at num-traits/src/sign.rs:75:1:
attempt to subtract with overflow
backtrace:
  &lt;i32 as num_traits::sign::Signed&gt;::abs_sub (num-traits/src/sign.rs:54:49)
  num_traits::sign::abs_sub::&lt;i32&gt; (num-traits/src/sign.rs:172:7)
  repro_num_traits_src_sign_rs_75_1_964925::main (src/main.rs:2:13)</pre></td></tr>
</table>
