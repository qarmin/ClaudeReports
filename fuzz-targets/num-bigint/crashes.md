# num-bigint - crashes

Source: https://github.com/rust-num/num-bigint @ 41f0b035d12971c5c7f7501917e31f0f6439c26c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = BigInt::from_radix_le(Sign::NoSign, &amp;vec![], 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at num-bigint/src/biguint/convert.rs:176:5:
The radix must be within 2...256
backtrace:
  num_bigint::biguint::convert::from_radix_le (num-bigint/src/biguint/convert.rs:176:5)
  &lt;num_bigint::biguint::BigUint&gt;::from_radix_le (num-bigint/src/biguint.rs:538:9)
  &lt;num_bigint::bigint::BigInt&gt;::from_radix_le (num-bigint/src/bigint.rs:751:17)
  repro_num_bigint_src_biguint_convert_rs_176_5_5bd037::main (src/main.rs:5:13)</pre></td></tr>
</table>
