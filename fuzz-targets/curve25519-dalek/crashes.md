# curve25519-dalek - crashes

Source: https://github.com/dalek-cryptography/curve25519-dalek @ 97a020d185970839ed1d3f3a5c22aca816631786

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Scalar::invert_batch_alloc(&amp;mut vec![Scalar::from_bytes_mod_order([0u8,
    0u8 /* x30 */, 0u8])]);</pre></td><td valign="top"><pre>thread 'main' panicked at curve25519-dalek/src/scalar.rs:837:9:
assertion failed: acc.pack() != Scalar::ZERO
backtrace:
  &lt;curve25519_dalek::scalar::Scalar&gt;::invert_batch_internal (curve25519-dalek/src/scalar.rs:837:9)
  &lt;curve25519_dalek::scalar::Scalar&gt;::invert_batch_alloc (curve25519-dalek/src/scalar.rs:810:9)
  repro_curve25519_dalek_src_scalar_rs_837_9_e43ec2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = RistrettoPoint::mul_base(
    &amp;&lt;curve25519_dalek::Scalar as ::core::ops::Sub&lt;curve25519_dalek::Scalar&gt;&gt;::
    sub(Scalar::from(0u32), Scalar::from_bits([0u8, 0u8, 0u8, 0u8, 1u8,
    0u8 /* x15 */, 1u8, 0u8 /* x10 */, 64u8])));</pre></td><td valign="top"><pre>thread 'main' panicked at curve25519-dalek/src/scalar.rs:1022:9:
assertion failed: self[31] &lt;= 127
backtrace:
  &lt;curve25519_dalek::scalar::Scalar&gt;::as_radix_16 (curve25519-dalek/src/scalar.rs:1022:9)
  &lt;curve25519_dalek::scalar::Scalar&gt;::as_radix_2w (curve25519-dalek/src/scalar.rs:1100:25)
  &lt;curve25519_dalek::edwards::EdwardsBasepointTable as curve25519_dalek::traits::BasepointTable&gt;::mul_base (curve25519-dalek/src/edwards.rs:1232:32)
  &lt;&amp;curve25519_dalek::edwards::EdwardsBasepointTable as core::ops::arith::Mul&lt;&amp;curve25519_dalek::scalar::Scalar&gt;&gt;::mul (curve25519-dalek/src/edwards.rs:1258:22)
  &lt;&amp;curve25519_dalek::scalar::Scalar as core::ops::arith::Mul&lt;&amp;curve25519_dalek::edwards::EdwardsBasepointTable&gt;&gt;::mul (curve25519-dalek/src/edwards.rs:1268:17)
  &lt;&amp;curve25519_dalek::scalar::Scalar as core::ops::arith::Mul&lt;&amp;curve25519_dalek::ristretto::RistrettoBasepointTable&gt;&gt;::mul (curve25519-dalek/src/ristretto.rs:1096:24)
  &lt;curve25519_dalek::ristretto::RistrettoPoint&gt;::mul_base (curve25519-dalek/src/ristretto.rs:947:13)
  repro_curve25519_dalek_src_scalar_rs_1022_9_1a3a0f::main (src/main.rs:5:13)</pre></td></tr>
</table>
