# p256 - crashes

Source: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let scalar = Scalar::ONE;
let _ = scalar.sqn_vartime(1000000000usize);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::primitives::carrying_add (crypto-bigint/src/primitives.rs:11)
  crypto_bigint::limb::Limb::carrying_add (crypto-bigint/src/limb/add.rs:20)
  p256::arithmetic::scalar::scalar_impl::q1_times_mu_shift_five (src/arithmetic/scalar/scalar64.rs:104)
  p256::arithmetic::scalar::scalar_impl::barrett_reduce (src/arithmetic/scalar/scalar64.rs:61)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:90)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_sqn_vartime_da2c16::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::ZERO;
let _ = &lt;p256::Scalar as ::core::ops::Neg&gt;::neg(scalar.sqn_vartime(
    10000000000000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::uint::mul::schoolbook::mul_wide (crypto-bigint/src/uint/mul/schoolbook.rs:26)
  crypto_bigint::uint::mul::karatsuba::widening_mul_fixed&lt;4, 4&gt; (crypto-bigint/src/uint/mul/karatsuba.rs:111)
  crypto_bigint::uint::Uint&lt;4&gt;::widening_mul&lt;4, 4&gt; (crypto-bigint/src/uint/mul.rs:45)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:89)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_neg_0898bc::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::from(0u64);
let scalar2 = scalar.double();
let _ = &lt;p256::Scalar as ::core::ops::Sub&lt;p256::Scalar&gt;&gt;::sub(
    scalar2.sqn_vartime(usize::MAX), Scalar::from(32327102942228415u64));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::uint::mul::schoolbook::mul_wide (crypto-bigint/src/uint/mul/schoolbook.rs:26)
  crypto_bigint::uint::mul::karatsuba::widening_mul_fixed&lt;4, 4&gt; (crypto-bigint/src/uint/mul/karatsuba.rs:111)
  crypto_bigint::uint::Uint&lt;4&gt;::widening_mul&lt;4, 4&gt; (crypto-bigint/src/uint/mul.rs:45)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:89)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_sub_p256_scalar_ea086d::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = &lt;p256::Scalar as ::core::ops::Add&lt;p256::Scalar&gt;&gt;::add(Scalar::from(
    0u64),
    &lt;p256::Scalar as ::core::ops::Neg&gt;::neg(Scalar::from(u32::MAX)).sqn_vartime(
    2usize));
let scalar2 = scalar.shr_vartime(3880915770u32);
let scalar3 = &lt;p256::Scalar as ::core::ops::Neg&gt;::neg(scalar2);
let _ = Scalar::from(scalar3.sqn_vartime(2147483648usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::primitives::carrying_mul_add (crypto-bigint/src/primitives.rs:63)
  crypto_bigint::limb::Limb::carrying_mul_add (crypto-bigint/src/limb/mul.rs:24)
  p256::arithmetic::scalar::scalar_impl::q1_times_mu_shift_five (src/arithmetic/scalar/scalar64.rs:92)
  p256::arithmetic::scalar::scalar_impl::barrett_reduce (src/arithmetic/scalar/scalar64.rs:61)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:90)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_from_p256_scalar_7f5503::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = &lt;p256::Scalar as ::core::ops::Shr&lt;usize&gt;&gt;::shr(Scalar::from(0u32),
    0usize);
let _ = scalar.multiply(&amp;Scalar::default().sqn_vartime(usize::MAX));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  p256::arithmetic::scalar::scalar_impl::q3_times_n_keep_five (src/arithmetic/scalar/scalar64.rs:136)
  p256::arithmetic::scalar::scalar_impl::barrett_reduce (src/arithmetic/scalar/scalar64.rs:64)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:90)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_multiply_61ff65::main (src/main.rs:5)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::default();
let _ = &lt;p256::Scalar as ::core::ops::Add&lt;p256::Scalar&gt;&gt;::add(scalar,
    Scalar::ZERO.sqn_vartime(1000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  p256::arithmetic::scalar::scalar_impl::barrett_reduce (src/arithmetic/scalar/scalar64.rs:65)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:90)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_add_p256_scalar_34e8c9::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NistP256;
let _ = Scalar::try_from(Scalar::from(0u32).sqn_vartime(usize::MAX));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::primitives::carrying_mul_add (crypto-bigint/src/primitives.rs:63)
  crypto_bigint::limb::Limb::carrying_mul_add (crypto-bigint/src/limb/mul.rs:24)
  p256::arithmetic::scalar::scalar_impl::q3_times_n_keep_five (src/arithmetic/scalar/scalar64.rs:133)
  p256::arithmetic::scalar::scalar_impl::barrett_reduce (src/arithmetic/scalar/scalar64.rs:64)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:90)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_try_from_p256_scalar_486a6a::main (src/main.rs:6)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;p256::Scalar as ::core::ops::Mul&lt;p256::Scalar&gt;&gt;::mul(Scalar::ZERO,
    Scalar::default().sqn_vartime(usize::MAX));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::primitives::carrying_mul_add (crypto-bigint/src/primitives.rs:53)
  crypto_bigint::limb::Limb::carrying_mul_add (crypto-bigint/src/limb/mul.rs:24)
  crypto_bigint::uint::mul::schoolbook::mul_wide (crypto-bigint/src/uint/mul/schoolbook.rs:28)
  crypto_bigint::uint::mul::karatsuba::widening_mul_fixed&lt;4, 4&gt; (crypto-bigint/src/uint/mul/karatsuba.rs:111)
  crypto_bigint::uint::Uint&lt;4&gt;::widening_mul&lt;4, 4&gt; (crypto-bigint/src/uint/mul.rs:45)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:89)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_mul_p256_scalar_5788d8::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;p256::Scalar as ::core::ops::Shr&lt;usize&gt;&gt;::shr(
    &lt;p256::Scalar as ::core::ops::Add&lt;p256::Scalar&gt;&gt;::add(Scalar::from(
    Scalar::from(64u64)).sqn_vartime(usize::MAX), Scalar::from(255u32)),
    2147483648usize);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::uint::mul::schoolbook::mul_wide (crypto-bigint/src/uint/mul/schoolbook.rs:25)
  crypto_bigint::uint::mul::karatsuba::widening_mul_fixed&lt;4, 4&gt; (crypto-bigint/src/uint/mul/karatsuba.rs:111)
  crypto_bigint::uint::Uint&lt;4&gt;::widening_mul&lt;4, 4&gt; (crypto-bigint/src/uint/mul.rs:45)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:89)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_shr_usize_5fd7a7::main (src/main.rs:4)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::ONE;
let _ = scalar.add(&amp;Scalar::default().sqn_vartime(1000000000000000000usize));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crypto_bigint::primitives::carrying_mul_add (crypto-bigint/src/primitives.rs:53)
  crypto_bigint::limb::Limb::carrying_mul_add (crypto-bigint/src/limb/mul.rs:24)
  crypto_bigint::uint::mul::schoolbook::mul_wide (crypto-bigint/src/uint/mul/schoolbook.rs:28)
  crypto_bigint::uint::mul::karatsuba::widening_mul_fixed&lt;4, 4&gt; (crypto-bigint/src/uint/mul/karatsuba.rs:111)
  crypto_bigint::uint::Uint&lt;4&gt;::widening_mul&lt;4, 4&gt; (crypto-bigint/src/uint/mul.rs:45)
  p256::arithmetic::scalar::Scalar::multiply (src/arithmetic/scalar.rs:89)
  p256::arithmetic::scalar::Scalar::square (src/arithmetic/scalar.rs:97)
  p256::arithmetic::scalar::Scalar::sqn_vartime (src/arithmetic/scalar.rs:173)
  repro_timeout_in_p256_scalar_add_4f68bf::main (src/main.rs:5)</pre></td></tr>
</table>
