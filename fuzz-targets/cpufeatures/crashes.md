# cpufeatures - crashes

Source: https://github.com/RustCrypto/utils @ 861955eaccf7a7ebe9a214667b8df221971a2e2f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>&lt;ctutils::Choice as ctutils::CtAssignSlice&gt;::ct_assign_slice(
    &amp;mut vec![Choice::from_u32_lsb(1u32)], &amp;[], Choice::FALSE);</pre></td><td valign="top"><pre>thread 'main' panicked at ctutils/src/traits/ct_assign.rs:42:9:
assertion `left == right` failed: source slice length (0) does not match destination slice length (1)
  left: 1
 right: 0
backtrace:
  &lt;ctutils::choice::Choice as ctutils::traits::ct_assign::CtAssignSlice&gt;::ct_assign_slice (ctutils/src/traits/ct_assign.rs:42:9)
  repro_ctutils_src_traits_ct_assign_rs_42_9_979218::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::NoPadding as block_padding::Padding&gt;::raw_pad(&amp;mut vec![],
    1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:344:9:
`pos` is bigger than block size
backtrace:
  &lt;block_padding::NoPadding as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:344:9)
  repro_block_padding_src_lib_rs_344_9_3b1409::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_unpad(
    &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:264:9:
block size is too big for ANSI X9.23
backtrace:
  &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_unpad (block-padding/src/lib.rs:264:9)
  repro_block_padding_src_lib_rs_264_9_419c48::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_pad(&amp;mut vec![],
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:256:9:
`pos` is bigger or equal to block size
backtrace:
  &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:256:9)
  repro_block_padding_src_lib_rs_256_9_6855de::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::Iso7816 as block_padding::Padding&gt;::raw_pad(&amp;mut vec![],
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:299:9:
`pos` is bigger or equal to block size
backtrace:
  &lt;block_padding::Iso7816 as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:299:9)
  repro_block_padding_src_lib_rs_299_9_7b5652::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_unpad(
    &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:169:9:
block size is too big for PKCS#7
backtrace:
  &lt;block_padding::Pkcs7&gt;::unpad (block-padding/src/lib.rs:169:9)
  &lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_unpad (block-padding/src/lib.rs:194:9)
  repro_block_padding_src_lib_rs_169_9_fe7d9d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;block_padding::Iso10126 as block_padding::Padding&gt;::raw_unpad(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:171:23:
attempt to subtract with overflow
backtrace:
  &lt;block_padding::Pkcs7&gt;::unpad (block-padding/src/lib.rs:171:23)
  &lt;block_padding::Iso10126 as block_padding::Padding&gt;::raw_unpad (block-padding/src/lib.rs:228:9)
  repro_block_padding_src_lib_rs_171_23_57c9a7::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_pad(
    &amp;mut include_bytes!("data_0.bin").to_vec(), 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:255:9:
block size is too big for ANSI X9.23
backtrace:
  &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:255:9)
  repro_block_padding_src_lib_rs_255_9_e0a8ae::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_unpad(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:266:23:
attempt to subtract with overflow
backtrace:
  &lt;block_padding::AnsiX923 as block_padding::Padding&gt;::raw_unpad (block-padding/src/lib.rs:266:23)
  repro_block_padding_src_lib_rs_266_23_cf1bd5::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_pad(&amp;mut include_bytes!(
    "data_0.bin").to_vec(), 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:186:9:
block size is too big for PKCS#7
backtrace:
  &lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:186:9)
  repro_block_padding_src_lib_rs_186_9_f7d2bb::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::ZeroPadding as block_padding::Padding&gt;::raw_pad(&amp;mut vec![],
    1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:107:9:
`pos` is bigger than block size
backtrace:
  &lt;block_padding::ZeroPadding as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:107:9)
  repro_block_padding_src_lib_rs_107_9_8089f9::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>&lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_pad(&amp;mut vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at block-padding/src/lib.rs:187:9:
`pos` is bigger or equal to block size
backtrace:
  &lt;block_padding::Pkcs7 as block_padding::Padding&gt;::raw_pad (block-padding/src/lib.rs:187:9)
  repro_block_padding_src_lib_rs_187_9_20623b::main (src/main.rs:2:5)</pre></td></tr>
</table>
