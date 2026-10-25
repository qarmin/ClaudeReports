# strict-num - crashes

Source: https://github.com/RazrFalcon/strict-num @ 76d36febdb73e875ae3e0c6ae1310a084a7860c7

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;strict_num::PositiveF32 as strict_num::ApproxEqUlps&gt;::approx_eq_ulps(
    &amp;PositiveF32::new(1.0f32).unwrap(), &amp;PositiveF32::default(), i32::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at float-cmp/src/ulps_eq.rs:48:17:
attempt to negate with overflow
backtrace:
  &lt;f32 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (float-cmp/src/ulps_eq.rs:48:17)
  &lt;strict_num::FiniteF32 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (strict-num/src/lib.rs:63:24)
  &lt;strict_num::PositiveF32 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (strict-num/src/lib.rs:63:24)
  repro_float_cmp_src_ulps_eq_rs_48_17_9c4c47::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;strict_num::PositiveF64 as strict_num::ApproxEqUlps&gt;::approx_eq_ulps(
    &amp;PositiveF64::new(1.0f64).unwrap(), &amp;PositiveF64::ZERO, i64::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at float-cmp/src/ulps_eq.rs:91:17:
attempt to negate with overflow
backtrace:
  &lt;f64 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (float-cmp/src/ulps_eq.rs:91:17)
  &lt;strict_num::FiniteF64 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (strict-num/src/lib.rs:93:24)
  &lt;strict_num::PositiveF64 as float_cmp::ulps_eq::ApproxEqUlps&gt;::approx_eq_ulps (strict-num/src/lib.rs:93:24)
  repro_float_cmp_src_ulps_eq_rs_91_17_5b7a28::main (src/main.rs:4:13)</pre></td></tr>
</table>
