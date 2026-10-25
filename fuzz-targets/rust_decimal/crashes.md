# rust_decimal - crashes

Source: https://github.com/paupino/rust-decimal @ b23fb077e301e77cb0ac36cc1ac4df1fba6c45d5

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::powu(
    &amp;Decimal::ONE_THOUSAND, 1000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:241:21:
Pow overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::powu (rust-decimal/src/maths.rs:241:21)
  repro_rust_decimal_src_maths_rs_241_21_2976e4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::norm_cdf(
    &amp;Decimal::from(-10000i16));</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:221:21:
Pow overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::powi (rust-decimal/src/maths.rs:221:21)
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::erf (rust-decimal/src/maths.rs:465:30)
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::erf (rust-decimal/src/maths.rs:467:25)
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::norm_cdf (rust-decimal/src/maths.rs:472:89)
  repro_rust_decimal_src_maths_rs_221_21_80b8a7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::norm_pdf(
    &amp;Decimal::try_from(621.0f32).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:478:21:
Norm Pdf overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::norm_pdf (rust-decimal/src/maths.rs:478:21)
  repro_rust_decimal_src_maths_rs_478_21_39f10e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::tan(
    &amp;Decimal::HALF_PI);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:514:21:
Tan overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::tan (rust-decimal/src/maths.rs:514:21)
  repro_rust_decimal_src_maths_rs_514_21_2ee1e0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::powd(
    &amp;Decimal::from(8i16), Decimal::ONE_THOUSAND);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:264:21:
Pow overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::powd (rust-decimal/src/maths.rs:264:21)
  repro_rust_decimal_src_maths_rs_264_21_671a4c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::exp(
    &amp;Decimal::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:163:21:
Exp underflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::exp (rust-decimal/src/maths.rs:163:21)
  repro_rust_decimal_src_maths_rs_163_21_15e228::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let decimal = Decimal::from(usize::MAX);
let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::exp(&amp;decimal);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:165:21:
Exp overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::exp (rust-decimal/src/maths.rs:165:21)
  repro_rust_decimal_src_maths_rs_165_21_d1a9d4::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::
    exp_with_tolerance(&amp;Decimal::from(100u8), Decimal::PI);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:182:21:
Exp overflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::exp_with_tolerance (rust-decimal/src/maths.rs:182:21)
  repro_rust_decimal_src_maths_rs_182_21_0cbbfd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rust_decimal::Decimal as rust_decimal::MathematicalOps&gt;::
    exp_with_tolerance(&amp;&lt;rust_decimal::Decimal as ::core::ops::Neg&gt;::neg(
    Decimal::from(1000000000u64)), Decimal::ZERO);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-decimal/src/maths.rs:180:21:
Exp underflowed
backtrace:
  &lt;rust_decimal::decimal::Decimal as rust_decimal::maths::MathematicalOps&gt;::exp_with_tolerance (rust-decimal/src/maths.rs:180:21)
  repro_rust_decimal_src_maths_rs_180_21_7deed3::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let u32_v = Decimal::MAX_SCALE;
let _ = &lt;rust_decimal::Decimal as ::core::ops::Div&lt;rust_decimal::Decimal&gt;&gt;::div(
    Decimal::from(u32_v), Decimal::try_from(0.0f64).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:13:
Division by zero
backtrace:
  &lt;&amp;rust_decimal::decimal::Decimal as core::ops::arith::Div&gt;::div (rust-decimal/src/arithmetic_impls.rs:239:45)
  &lt;rust_decimal::decimal::Decimal as core::ops::arith::Div&gt;::div (rust-decimal/src/arithmetic_impls.rs:164:25)
  repro_rust_decimal_src_arithmetic_impls_rs_239_45_f5175e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let decimal = Decimal::MIN;
let _ = &lt;rust_decimal::Decimal as ::core::ops::Rem&lt;rust_decimal::Decimal&gt;&gt;::rem(
    decimal, Decimal::ZERO);</pre></td><td valign="top"><pre>thread 'main' panicked at src/main.rs:5:13:
Division by zero
backtrace:
  &lt;&amp;rust_decimal::decimal::Decimal as core::ops::arith::Rem&gt;::rem (rust-decimal/src/arithmetic_impls.rs:268:45)
  &lt;rust_decimal::decimal::Decimal as core::ops::arith::Rem&gt;::rem (rust-decimal/src/arithmetic_impls.rs:164:25)
  repro_rust_decimal_src_arithmetic_impls_rs_268_45_40c37f::main (src/main.rs:5:13)</pre></td></tr>
</table>
