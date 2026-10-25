# blake2 - crashes

Source: https://github.com/RustCrypto/hashes @ ff2b46e7d972c7f8f59d0650e83db6d80a7783f0

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Blake2bVarCore::new_with_params(&amp;vec![], &amp;vec![73u8, 73u8, 42u8, 0u8,
    42u8, 0u8 /* x11 */, 0u8], 0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at blake2/src/lib.rs:40:1:
assertion failed: persona.len() &lt;= length
backtrace:
  &lt;blake2::Blake2bVarCore&gt;::new_with_params (blake2/src/macros.rs:39:17)
  repro_blake2_src_lib_rs_40_1_cf342d::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Blake2sVarCore::new_with_params(&amp;vec![], &amp;vec![],
    1000000000000000000usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at blake2/src/lib.rs:88:1:
assertion failed: key_size &lt;= U32::to_usize()
backtrace:
  &lt;blake2::Blake2sVarCore&gt;::new_with_params (blake2/src/macros.rs:33:17)
  repro_blake2_src_lib_rs_88_1_25ac98::main (src/main.rs:4:13)</pre></td></tr>
</table>
