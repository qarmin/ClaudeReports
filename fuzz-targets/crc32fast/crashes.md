# crc32fast - crashes

Source: https://github.com/srijs/rust-crc32fast @ ad14266cf782ba6936d1af37fc9fd6382ef73b7c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let u32_v = crc32fast::hash(&amp;vec![]);
let mut hasher = Hasher::new_with_initial_len(u32_v, u64::MAX);
hasher.update(&amp;vec![34u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-crc32fast/src/lib.rs:118:9:
attempt to add with overflow
backtrace:
  &lt;crc32fast::Hasher&gt;::update (rust-crc32fast/src/lib.rs:118:9)
  repro_rust_crc32fast_src_lib_rs_118_9_da7ff8::main (src/main.rs:6:12)</pre></td></tr>
</table>
