# lexical-util - crashes

Source: https://github.com/Alexhuszagh/rust-lexical @ afb2efa651f42f63415a3bf3d51ced6143132b2b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = lexical_util::mul::mulhi::&lt;u16, u16&gt;(0u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/mul.rs:45:14:
attempt to shift right with overflow
backtrace:
  &lt;u16 as core::ops::bit::Shr&lt;i32&gt;&gt;::shr (core/src/ops/bit.rs:613:17)
  lexical_util::mul::mulhi::&lt;u16, u16&gt; (lexical-util/src/mul.rs:45:14)
  repro_lexical_util_src_mul_rs_45_14_83109e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lexical_util::mul::mul::&lt;u32, u8&gt;(1000000000u32, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/mul.rs:27:14:
attempt to multiply with overflow
backtrace:
  &lt;u32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  lexical_util::mul::mul::&lt;u32, u8&gt; (lexical-util/src/mul.rs:27:14)
  repro_lexical_util_src_mul_rs_27_14_a42056::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lexical_util::mul::mulhi::&lt;u32, u8&gt;(1000000000u32, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/mul.rs:57:5:
attempt to multiply with overflow
backtrace:
  &lt;u32 as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  lexical_util::mul::mulhi::&lt;u32, u8&gt; (lexical-util/src/mul.rs:57:5)
  repro_lexical_util_src_mul_rs_57_5_230457::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lexical_util::digit::digit_to_char_const(65535u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/digit.rs:61:9:
attempt to add with overflow
backtrace:
  lexical_util::digit::digit_to_char_const (lexical-util/src/digit.rs:61:9)
  repro_lexical_util_src_digit_rs_61_9_88bc42::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lexical_util::digit::digit_to_char_const(65535u32, 100u32);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/digit.rs:63:9:
attempt to add with overflow
backtrace:
  lexical_util::digit::digit_to_char_const (lexical-util/src/digit.rs:63:9)
  repro_lexical_util_src_digit_rs_63_9_f2bc4a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = lexical_util::mul::mul::&lt;u16, u64&gt;(0u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at lexical-util/src/mul.rs:17:14:
attempt to shift right with overflow
backtrace:
  &lt;u16 as core::ops::bit::Shr&lt;i32&gt;&gt;::shr (core/src/ops/bit.rs:613:17)
  lexical_util::mul::mul::&lt;u16, u64&gt; (lexical-util/src/mul.rs:17:14)
  repro_lexical_util_src_mul_rs_17_14_871c77::main (src/main.rs:2:13)</pre></td></tr>
</table>
