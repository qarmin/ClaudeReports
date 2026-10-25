# chacha20 - crashes

Source: https://github.com/RustCrypto/stream-ciphers @ a2024dde67d449fe2bcb4feecda4bd3959a4704d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>&lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::set_block_pos(
    &amp;mut vec![], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at chacha20/src/variants.rs:75:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::set_block_pos (chacha20/src/variants.rs:75:9)
  repro_chacha20_src_variants_rs_75_9_63122c::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::set_block_pos(
    &amp;mut vec![0u32], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at chacha20/src/variants.rs:76:9:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::set_block_pos (chacha20/src/variants.rs:76:9)
  repro_chacha20_src_variants_rs_76_9_e21faf::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;chacha20::variants::Ietf as chacha20::variants::Variant&gt;::
    get_block_pos(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at chacha20/src/variants.rs:41:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;chacha20::variants::Ietf as chacha20::variants::Variant&gt;::get_block_pos (chacha20/src/variants.rs:41:9)
  repro_chacha20_src_variants_rs_41_9_20bf0b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::
    get_block_pos(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at chacha20/src/variants.rs:70:20:
index out of bounds: the len is 0 but the index is 1
backtrace:
  &lt;chacha20::variants::Legacy as chacha20::variants::Variant&gt;::get_block_pos (chacha20/src/variants.rs:70:20)
  repro_chacha20_src_variants_rs_70_20_54698f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;chacha20::variants::Ietf as chacha20::variants::Variant&gt;::set_block_pos(
    &amp;mut vec![], 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at chacha20/src/variants.rs:46:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;chacha20::variants::Ietf as chacha20::variants::Variant&gt;::set_block_pos (chacha20/src/variants.rs:46:9)
  repro_chacha20_src_variants_rs_46_9_562ab9::main (src/main.rs:2:5)</pre></td></tr>
</table>
