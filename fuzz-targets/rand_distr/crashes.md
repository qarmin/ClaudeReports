# rand_distr - crashes

Source: https://github.com/rust-random/rand_distr @ d65b9bbf991e56d8a097a35d934e6f93d9194ac0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Hypergeometric::new(u64::MAX, 0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rand_distr/src/hypergeometric.rs:204:53:
attempt to add with overflow
backtrace:
  &lt;rand_distr (rand_distr/src/hypergeometric.rs:204:53)
  repro_rand_distr_src_hypergeometric_rs_204_53_540aeb::main (src/main.rs:4:13)</pre></td></tr>
</table>
