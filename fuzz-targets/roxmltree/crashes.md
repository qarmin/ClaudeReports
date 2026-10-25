# roxmltree - crashes

Source: https://github.com/RazrFalcon/roxmltree @ 3f5c78dea57662b43a2f41961397b1f5ffe27166

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = NodeId::from(u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at roxmltree/src/lib.rs:368:9:
assertion failed: id &lt; u32::MAX
backtrace:
  &lt;roxmltree::NodeId&gt;::new (roxmltree/src/lib.rs:368:9)
  &lt;roxmltree::NodeId as core::convert::From&lt;u32&gt;&gt;::from (roxmltree/src/lib.rs:390:9)
  repro_roxmltree_src_lib_rs_368_9_20c889::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NodeId::from(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at roxmltree/src/lib.rs:398:9:
assertion failed: id &lt;= u32::MAX as usize
backtrace:
  &lt;roxmltree::NodeId as core::convert::From&lt;usize&gt;&gt;::from (roxmltree/src/lib.rs:398:9)
  repro_roxmltree_src_lib_rs_398_9_9f58ed::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let string = include_str!("string_0.txt").to_string();
let document = Document::parse(&amp;string).unwrap();
let node = document.root_element();
let document2 = node.document();
let _ = document2.text_pos_at(1389usize);</pre></td><td valign="top"><pre>thread 'main' panicked at roxmltree/src/tokenizer.rs:1187:22:
end byte index 1389 is not a char boundary; it is inside '😀' (bytes 1387..1391 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;roxmltree::tokenizer::Stream&gt;::calc_curr_col (roxmltree/src/tokenizer.rs:1187:22)
  &lt;roxmltree::tokenizer::Stream&gt;::gen_text_pos (roxmltree/src/tokenizer.rs:1160:19)
  &lt;roxmltree::tokenizer::Stream&gt;::gen_text_pos_from (roxmltree/src/tokenizer.rs:1171:11)
  &lt;roxmltree::Document&gt;::text_pos_at (roxmltree/src/lib.rs:202:43)
  repro_roxmltree_src_tokenizer_rs_1187_22_0c7649::main (src/main.rs:8:23)</pre></td></tr>
</table>
