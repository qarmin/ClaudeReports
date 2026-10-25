# p384 - crashes

Source: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let scalar = Scalar::try_from(Scalar::ZERO.sqn_vartime(65536usize)).unwrap();
let scalar2 = scalar.add(&amp;Scalar::default());
let _ = scalar2.sqn_vartime(4294967296usize);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:114)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_sqn_vartime_cfd136::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Scalar::try_from(Scalar::ONE.sqn_vartime(10000000000000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:114)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_try_from_p384_scalar_4c3f99::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;p384::Scalar as ::core::ops::Sub&lt;p384::Scalar&gt;&gt;::sub(Scalar::from(
    0u64), Scalar::from(0u64).sqn_vartime(10000000000000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:113)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_sub_p384_scalar_b0ec2a::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::from_u64(0u64);
let _ = scalar.sub(&amp;Scalar::from(0u64).sqn_vartime(1000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ctutils::choice::Choice::to_u8 (src/choice.rs:398)
  ctutils::choice::Choice::to_u8_vartime (src/choice.rs:421)
  crypto_bigint::word::choice_to_mask (crypto-bigint/src/word.rs:129)
  crypto_bigint::word::select (crypto-bigint/src/word.rs:113)
  crypto_bigint::limb::Limb::select (crypto-bigint/src/limb/ct.rs:10)
  crypto_bigint::uint::Uint&lt;6&gt;::select&lt;6&gt; (crypto-bigint/src/uint/ct.rs:14)
  crypto_bigint::uint::Uint&lt;6&gt;::try_sub_with_carry&lt;6&gt; (crypto-bigint/src/uint/sub_mod.rs:25)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:169)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_sub_ba827b::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Scalar::from(Scalar::from(255u64).sqn_vartime(
    2147483648usize).double());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:116)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_from_p384_scalar_83a86a::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::default();
let scalar2 = scalar.neg();
let _ = scalar.add(&amp;scalar2.sqn_vartime(1000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:114)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_add_c04f59::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;p384::Scalar as ::core::ops::Add&lt;p384::Scalar&gt;&gt;::add(Scalar::from(
    0u32), Scalar::default().sqn_vartime(1000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:33)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_add_p384_scalar_19d4ad::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;p384::Scalar as ::core::ops::Mul&lt;p384::Scalar&gt;&gt;::mul(Scalar::from(
    0u64).sqn_vartime(10000000000000000000usize), Scalar::from(16u32));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::modular::mul::montgomery_multiply_inner (crypto-bigint/src/modular/mul.rs:114)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:162)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_mul_p384_scalar_a34493::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::ONE;
let _ = scalar.multiply(&amp;Scalar::default().sqn_vartime(2147483648usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ctutils::choice::Choice::to_u8 (src/choice.rs:398)
  ctutils::choice::Choice::to_u8_vartime (src/choice.rs:421)
  crypto_bigint::word::choice_to_mask (crypto-bigint/src/word.rs:129)
  crypto_bigint::word::select (crypto-bigint/src/word.rs:113)
  crypto_bigint::limb::Limb::select (crypto-bigint/src/limb/ct.rs:10)
  crypto_bigint::uint::Uint&lt;6&gt;::select&lt;6&gt; (crypto-bigint/src/uint/ct.rs:14)
  crypto_bigint::uint::Uint&lt;6&gt;::try_sub_with_carry&lt;6&gt; (crypto-bigint/src/uint/sub_mod.rs:25)
  crypto_bigint::modular::mul::square_montgomery_form&lt;6&gt; (crypto-bigint/src/modular/mul.rs:169)
  crypto_bigint::modular::const_monty_form::ConstMontyForm&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (crypto-bigint/src/modular/const_monty_form/mul.rs:34)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::square&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:366)
  primefield::monty::MontyFieldElement&lt;p384::arithmetic::scalar::ScalarParams, 6&gt;::sqn_vartime&lt;p384::arithmetic::scalar::ScalarParams, 6&gt; (primefield/src/monty.rs:420)
  p384::arithmetic::scalar::Scalar::sqn_vartime (primefield/src/macros.rs:263)
  repro_timeout_in_p384_scalar_multiply_1452d2::main (src/main.rs:5)</pre></td></tr>
</table>
