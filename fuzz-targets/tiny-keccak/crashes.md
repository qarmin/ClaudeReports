# tiny-keccak - crashes

Source: https://github.com/debris/tiny-keccak @ f0fa14629021cdffe1041eee7261843fb05bb373

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut parallel_hash = ParallelHash::v256(&amp;vec![], 0usize);
&lt;tiny_keccak::ParallelHash as tiny_keccak::Hasher&gt;::update(&amp;mut parallel_hash,
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at tiny-keccak/src/parallel_hash.rs:108:32:
attempt to divide by zero
backtrace:
  &lt;tiny_keccak::parallel_hash::ParallelHash as tiny_keccak::Hasher&gt;::update (tiny-keccak/src/parallel_hash.rs:108:32)
  repro_tiny_keccak_src_parallel_hash_rs_108_32_f06dc7::main (src/main.rs:5:5)</pre></td></tr>
</table>
