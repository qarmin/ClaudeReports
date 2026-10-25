# phf_shared - crashes

Source: https://github.com/rust-phf/rust-phf @ c15adb08f43f3dadbb725b04dbf7f88ebec7d1de

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let hashes = phf_shared::hash::&lt;u16&gt;(&amp;0u16, &amp;0u64);
let _ = phf_shared::get_index(&amp;hashes, &amp;[], 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at phf_shared/src/lib.rs:64:26:
attempt to calculate the remainder with a divisor of zero
backtrace:
  phf_shared::get_index (phf_shared/src/lib.rs:64:26)
  repro_phf_shared_src_lib_rs_64_26_fcc62e::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = phf_shared::ptrhash::get_index(0u64, 0u64, &amp;vec![], &amp;[], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at phf_shared/src/ptrhash.rs:34:39:
index out of bounds: the len is 0 but the index is 0
backtrace:
  phf_shared::ptrhash::get_index (phf_shared/src/ptrhash.rs:34:39)
  repro_phf_shared_src_ptrhash_rs_34_39_bbd7f9::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = phf_shared::ptrhash::get_index(0u64, 0u64, &amp;vec![35u8], &amp;[], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at phf_shared/src/ptrhash.rs:41:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  phf_shared::ptrhash::get_index (phf_shared/src/ptrhash.rs:41:9)
  repro_phf_shared_src_ptrhash_rs_41_9_76bafc::main (src/main.rs:2:13)</pre></td></tr>
</table>
