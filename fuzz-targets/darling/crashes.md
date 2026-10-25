# darling - crashes

Source: https://github.com/TedDriggs/darling @ 70b280dede2eaa1ee53f445a970657b16299767b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Accumulator::default();</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/error/mod.rs:955:26:
darling::error::Accumulator dropped without being finished
backtrace:
  &lt;darling_core::error::Accumulator as core::ops::drop::Drop&gt;::drop (core/src/error/mod.rs:955:26)
  repro_core_src_error_mod_rs_955_26_fc365c::main (src/main.rs:4:35)</pre></td></tr>
</table>
