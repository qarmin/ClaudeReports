# ark-ff - crashes

Source: https://github.com/arkworks-rs/algebra @ 4a19a1c50606ae4a0e96ccce107f6281aa0be38e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ark_ff::fields::utils::k_adicity(1u64, 1u64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ark_ff::fields::utils::k_adicity (src/fields/utils.rs:11)
  repro_timeout_in_ark_ff_fields_utils_k_adicity_04d269::main (src/main.rs:2)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ark_ff::biginteger::arithmetic::find_relaxed_naf(&amp;[2u64]);</pre></td><td valign="top"><pre>thread 'main' panicked at ff/src/biginteger/arithmetic.rs:190:33:
attempt to subtract with overflow
backtrace:
  ark_ff::biginteger::arithmetic::find_relaxed_naf (ff/src/biginteger/arithmetic.rs:190:33)
  repro_ff_src_biginteger_arithmetic_rs_190_33_de90cd::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ark_ff::biginteger::arithmetic::find_relaxed_naf(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at ff/src/biginteger/arithmetic.rs:190:12:
attempt to subtract with overflow
backtrace:
  ark_ff::biginteger::arithmetic::find_relaxed_naf (ff/src/biginteger/arithmetic.rs:190:12)
  repro_ff_src_biginteger_arithmetic_rs_190_12_b3c4f0::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ark_ff::fields::utils::k_adicity(0u64, 1u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ff/src/fields/utils.rs:11:11:
attempt to calculate the remainder with a divisor of zero
backtrace:
  ark_ff::fields::utils::k_adicity (ff/src/fields/utils.rs:11:11)
  repro_ff_src_fields_utils_rs_11_11_38e67c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ark_ff::signed_mod_reduction(0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ff/src/biginteger/mod.rs:909:13:
attempt to calculate the remainder with a divisor of zero
backtrace:
  ark_ff::biginteger::signed_mod_reduction (ff/src/biginteger/mod.rs:909:13)
  repro_ff_src_biginteger_mod_rs_909_13_e8411e::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ark_ff::signed_mod_reduction(u64::MAX, 10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at ff/src/biginteger/mod.rs:911:9:
attempt to subtract with overflow
backtrace:
  ark_ff::biginteger::signed_mod_reduction (ff/src/biginteger/mod.rs:911:9)
  repro_ff_src_biginteger_mod_rs_911_9_156b87::main (src/main.rs:2:13)</pre></td></tr>
</table>
