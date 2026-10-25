# sec1 - crashes

Source: https://github.com/RustCrypto/formats @ 13a0390d40297121e272044ee73d796e1bfb72ee

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let tag = Tag::Compact;
let _ = tag.message_len(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at sec1/src/point.rs:556:9:
attempt to add with overflow
backtrace:
  &lt;sec1::point::Tag&gt;::message_len (sec1/src/point.rs:556:9)
  repro_sec1_src_point_rs_556_9_7f1ae6::main (src/main.rs:5:17)</pre></td></tr>
<tr><td valign="top"><pre>let tag = Tag::Uncompressed;
let _ = tag.message_len(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at sec1/src/point.rs:559:34:
attempt to multiply with overflow
backtrace:
  &lt;sec1::point::Tag&gt;::message_len (sec1/src/point.rs:559:34)
  repro_sec1_src_point_rs_559_34_063442::main (src/main.rs:5:17)</pre></td></tr>
</table>
