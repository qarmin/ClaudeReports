# libm - crashes

Source: https://github.com/rust-lang/compiler-builtins @ f521807de24520770991263387ba8c06e462edf8

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = libm::support::linear_mul_reduction::&lt;u16&gt;(0u16, 1u32, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/modular.rs:23:5:
assertion failed: x &lt; (y &lt;&lt; 1)
backtrace:
  libm::math::support::modular::linear_mul_reduction::&lt;u16&gt; (libm/src/math/support/modular.rs:23:5)
  repro_libm_src_math_support_modular_rs_23_5_539c5b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = libm::support::linear_mul_reduction::&lt;u16&gt;(0u16, u32::MAX, 1000u16);</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/modular.rs:42:5:
attempt to add with overflow
backtrace:
  libm::math::support::modular::linear_mul_reduction::&lt;u16&gt; (libm/src/math/support/modular.rs:42:5)
  repro_libm_src_math_support_modular_rs_42_5_21e813::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = libm::support::linear_mul_reduction::&lt;u8&gt;(0u8, 0u32, 100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/modular.rs:22:5:
assertion failed: y &lt;= U::MAX &gt;&gt; 2
backtrace:
  libm::math::support::modular::linear_mul_reduction::&lt;u8&gt; (libm/src/math/support/modular.rs:22:5)
  repro_libm_src_math_support_modular_rs_22_5_8c4a3a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = libm::support::hf32("");</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/hex_float.rs:21:39:
no hex indicator
backtrace:
  libm::math::support (libm/src/math/support/hex_float.rs:21:39)
  repro_libm_src_math_support_hex_float_rs_21_39_31542c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = libm::support::hex_float::parse_any("", 6u32, 0u32, Round::Positive);</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/hex_float.rs:79:26:
attempt to subtract with overflow
backtrace:
  libm::math::support (libm/src/math/support/hex_float.rs:79:26)
  repro_libm_src_math_support_hex_float_rs_79_26_d79c40::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = libm::support::hf64("");</pre></td><td valign="top"><pre>thread 'main' panicked at libm/src/math/support/hex_float.rs:29:39:
no hex indicator
backtrace:
  libm::math::support (libm/src/math/support/hex_float.rs:29:39)
  repro_libm_src_math_support_hex_float_rs_29_39_6537e4::main (src/main.rs:2:13)</pre></td></tr>
</table>
