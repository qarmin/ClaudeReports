# bigdecimal - crashes

Source: https://github.com/akubera/bigdecimal-rs @ f0a1f920f222d9776584b26659a3c9326dd1aca1

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;bigdecimal::BigDecimal as ::core::ops::Add&lt;i64&gt;&gt;::add(BigDecimal::from(
    1i32).with_scale_round(-2535105207928642425i64, RoundingMode::Up), 18i64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:46)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (num-bigint/src/biguint/multiplication.rs:482)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:103)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:100)
... (previous line repeated 8 more times)
  bigdecimal::arithmetic::ten_to_the (src/arithmetic/mod.rs:56)
  bigdecimal::BigDecimal::with_scale (src/lib.rs:335)
  bigdecimal::arithmetic::addition::addassign_bigdecimal_ref&lt;bigdecimal::BigDecimalRef&gt; (src/arithmetic/addition.rs:105)
  bigdecimal::impl_ops_add::{impl#16}::add_assign&lt;bigdecimal::BigDecimalRef&gt; (src/impl_ops_add.rs:152)
  bigdecimal::arithmetic::addition::addassign_bigdecimals (src/arithmetic/addition.rs:93)
  bigdecimal::impl_ops_add::{impl#15}::add_assign (src/impl_ops_add.rs:145)
  bigdecimal::impl_ops::{impl#104}::add_assign (src/impl_ops.rs:71)
  bigdecimal::impl_ops::{impl#99}::add (src/impl_ops.rs:18)
  repro_timeout_in_bigdecimal_bigdecimal_add_i64_dee6b7::main (src/main.rs:5)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;bigdecimal::BigDecimal as ::core::ops::Sub&lt;i32&gt;&gt;::sub(BigDecimal::from(
    1i8).with_scale(1000000000i64), 0i32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::addition::__add2 (src/biguint/addition.rs:64)
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:53)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
... (previous line repeated 1 more times)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (num-bigint/src/biguint/multiplication.rs:482)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:103)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:100)
  bigdecimal::arithmetic::ten_to_the (src/arithmetic/mod.rs:56)
  bigdecimal::BigDecimal::with_scale (bigdecimal/src/lib.rs:335)
  repro_timeout_in_bigdecimal_bigdecimal_sub_i32_5b64fb::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let rounding_mode = RoundingMode::Floor;
let _ = &lt;bigdecimal::BigDecimal as bigdecimal::ToPrimitive&gt;::to_i64(
    &amp;BigDecimal::from(1u16).with_precision_round(std::num::NonZero::new(
    4294967295u64).unwrap(), rounding_mode));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:46)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:271)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:358)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (num-bigint/src/biguint/multiplication.rs:482)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:105)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:100)
... (previous line repeated 1 more times)
  bigdecimal::arithmetic::ten_to_the (src/arithmetic/mod.rs:56)
  bigdecimal::BigDecimal::with_scale_round (src/lib.rs:372)
  bigdecimal::BigDecimal::with_precision_round (src/lib.rs:555)
  repro_timeout_in_bigdecimal_bigdecimal_to_i64_97f628::main (src/main.rs:6)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let big_decimal = &lt;bigdecimal::BigDecimal as bigdecimal::Signed&gt;::abs_sub(
    &amp;BigDecimal::from(0i16), &amp;BigDecimal::from(0i8));
let big_decimal2 = big_decimal.normalized();
let big_decimal_ref = big_decimal2.to_ref();
let _ = big_decimal_ref.to_owned_with_scale(1000000000i64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  num_bigint::biguint::addition::__add2 (src/biguint/addition.rs:63)
  num_bigint::biguint::multiplication::mac_digit (src/biguint/multiplication.rs:53)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:105)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:242)
... (previous line repeated 1 more times)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:255)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:361)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:352)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#7}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#14}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:355)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (src/biguint/multiplication.rs:482)
  num_bigint::bigint::multiplication::{impl#17}::mul (src/bigint/multiplication.rs:34)
  num_bigint::biguint::multiplication::mac3 (src/biguint/multiplication.rs:349)
  num_bigint::biguint::multiplication::mul3 (src/biguint/multiplication.rs:419)
  num_bigint::biguint::multiplication::{impl#10}::mul (num-bigint/src/biguint/multiplication.rs:482)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:103)
  bigdecimal::arithmetic::ten_to_the_uint (src/arithmetic/mod.rs:100)
  bigdecimal::BigDecimalRef::to_owned_with_scale (src/lib.rs:1285)
  repro_timeout_in_bigdecimal_bigdecimalref_to_owned_with_scale_77161e::main (src/main.rs:7)
  (4 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let big_decimal = &lt;bigdecimal::BigDecimal as ::core::ops::Mul&lt;i64&gt;&gt;::mul(
    &lt;bigdecimal::BigDecimal as bigdecimal::FromPrimitive&gt;::from_u64(
    10000000000000000000u64).unwrap(), 65535i64);
let big_decimal_ref = big_decimal.to_ref();
let _ = big_decimal_ref.exp();</pre></td><td valign="top"><pre>thread 'main' panicked at bigdecimal-rs/src/arithmetic/pow.rs:266:5:
attempt to multiply with overflow
backtrace:
  bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint (bigdecimal-rs/src/arithmetic/pow.rs:266:5)
  bigdecimal::arithmetic::pow::pow_2_k_scaled_biguint (bigdecimal-rs/src/arithmetic/pow.rs:272:5)
... (previous line repeated 10 more times)</pre></td></tr>
<tr><td valign="top"><pre>let big_decimal = &lt;bigdecimal::BigDecimal as ::core::ops::Neg&gt;::neg(
    BigDecimal::from(0i32));
let big_decimal2 = big_decimal.with_scale_round(0i64, RoundingMode::HalfEven);
let big_decimal3 = big_decimal2.with_scale(0i64);
let big_decimal_ref = big_decimal3.to_ref();
let _ = big_decimal_ref.sqrt_copysign_with_context(
    &amp;Context::default().with_precision(std::num::NonZero::new(
    u64::MAX).unwrap()));</pre></td><td valign="top"><pre>thread 'main' panicked at bigdecimal-rs/src/arithmetic/sqrt.rs:16:29:
attempt to add with overflow
backtrace:
  bigdecimal::arithmetic::sqrt::impl_sqrt (bigdecimal-rs/src/arithmetic/sqrt.rs:16:29)
  &lt;bigdecimal::BigDecimalRef&gt;::sqrt_copysign_with_context (bigdecimal-rs/src/lib.rs:1426:26)
  repro_bigdecimal_rs_src_arithmetic_sqrt_rs_16_29_ca81b6::main (src/main.rs:10:29)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BigDecimal::from(2i64).powi(i64::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at bigdecimal-rs/src/arithmetic/multiplication.rs:686:19:
attempt to add with overflow
backtrace:
  bigdecimal::arithmetic::multiplication::mul_scaled_slices_truncating_into::&lt;bigdecimal::bigdigit::radix::RADIX_10p19_u64, bigdecimal::bigdigit::endian::LittleEndian, bigdecimal::bigdigit::endian::LittleEndian, bigdecimal::bigdigit::endian::LittleEndian&gt; (bigdecimal-rs/src/arithmetic/multiplication... (+15 chars)
  bigdecimal::arithmetic::pow::pow_u64_with_context (bigdecimal-rs/src/arithmetic/pow.rs:158:33)
  bigdecimal::arithmetic::pow::powi_with_context (bigdecimal-rs/src/arithmetic/pow.rs:54:13)
  bigdecimal::arithmetic::pow::impl_powi_with_context::&lt;bigdecimal::BigDecimalRef&gt; (bigdecimal-rs/src/arithmetic/pow.rs:29:5)
  &lt;bigdecimal::BigDecimal&gt;::powi_with_context (bigdecimal-rs/src/lib.rs:798:9)
  &lt;bigdecimal::BigDecimal&gt;::powi (bigdecimal-rs/src/lib.rs:787:14)
  repro_bigdecimal_rs_src_arithmetic_multiplication_rs_686_19_a02f63::main (src/main.rs:4:36)</pre></td></tr>
<tr><td valign="top"><pre>let context = Context::default();
let context2 = context.with_rounding_mode(RoundingMode::Down);
let context3 = context2.with_precision(std::num::NonZero::new(
    u64::MAX).unwrap());
let big_decimal = &lt;bigdecimal::BigDecimal as bigdecimal::FromPrimitive&gt;::
    from_u64(1000u64).unwrap();
let _ = context3.round_decimal_ref::&lt;bigdecimal::BigDecimalRef&gt;(
    big_decimal.to_ref());</pre></td><td valign="top"><pre>thread 'main' panicked at bigdecimal-rs/src/lib.rs:560:26:
precision overflow
backtrace:
  &lt;bigdecimal::BigDecimal&gt;::with_precision_round (bigdecimal-rs/src/lib.rs:560:26)
  &lt;bigdecimal::context::Context&gt;::round_decimal_ref::&lt;bigdecimal::BigDecimalRef&gt; (bigdecimal-rs/src/context.rs:99:11)
  repro_bigdecimal_rs_src_lib_rs_560_26_3e1678::main (src/main.rs:9:22)</pre></td></tr>
<tr><td valign="top"><pre>let big_decimal = &lt;bigdecimal::BigDecimal as ::core::ops::Mul&lt;i16&gt;&gt;::mul(
    BigDecimal::try_from(1.0f32).unwrap(), 1i16);
let big_decimal2 = big_decimal.sqrt().unwrap();
let big_decimal_ref = big_decimal2.to_ref();
let _ = Context::new(std::num::NonZero::new(u64::MAX).unwrap(),
    RoundingMode::HalfDown).invert::&lt;bigdecimal::BigDecimalRef&gt;(
    big_decimal_ref);</pre></td><td valign="top"><pre>thread 'main' panicked at bigdecimal-rs/src/arithmetic/inverse.rs:73:77:
attempt to add with overflow
backtrace:
  bigdecimal::arithmetic::inverse::impl_inverse_uint_scale (bigdecimal-rs/src/arithmetic/inverse.rs:73:77)
  bigdecimal::arithmetic::inverse::inverse_scaled_uint_with_context::&lt;bigdecimal::BigDecimalRef&gt; (bigdecimal-rs/src/arithmetic/inverse.rs:12:5)
  &lt;bigdecimal::BigDecimalRef&gt;::inverse_with_context (bigdecimal-rs/src/lib.rs:1400:9)
  &lt;bigdecimal::context::Context&gt;::invert::&lt;bigdecimal::BigDecimalRef&gt; (bigdecimal-rs/src/context.rs:184:18)
  repro_bigdecimal_rs_src_arithmetic_inverse_rs_73_77_1ffea9::main (src/main.rs:9:93)</pre></td></tr>
</table>
