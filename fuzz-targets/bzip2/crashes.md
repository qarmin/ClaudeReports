# bzip2 - crashes

Source: https://github.com/trifectatechfoundation/bzip2-rs @ 57f97219b90dac2ac6f0f62745e900dde0d48fde

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let compression = Compression::best();
let _ = Compress::new(compression, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at bzip2-rs/src/mem.rs:122:13:
assertion `left == right` failed
  left: -2
 right: 0
backtrace:
  &lt;bzip2::mem::Compress&gt;::new (bzip2-rs/src/mem.rs:122:13)
  repro_bzip2_rs_src_mem_rs_122_13_e6057e::main (src/main.rs:6:13)</pre></td></tr>
</table>
