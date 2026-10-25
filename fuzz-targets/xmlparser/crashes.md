# xmlparser - crashes

Source: https://github.com/RazrFalcon/xmlparser @ 9c8e34305723118c67d7feba95772705b2247776

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, usize::MAX..0usize);
let mut stream = tokenizer.stream();
stream.advance(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:152:23:
attempt to add with overflow
backtrace:
  &lt;xmlparser::stream::Stream&gt;::advance (xmlparser/src/stream.rs:152:23)
  repro_xmlparser_src_stream_rs_152_23_667006::main (src/main.rs:7:12)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 1usize..0usize);
let _ = stream.gen_text_pos();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:605:34:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;xmlparser::stream::Stream&gt;::calc_curr_row (xmlparser/src/stream.rs:605:34)
  &lt;xmlparser::stream::Stream&gt;::gen_text_pos (xmlparser/src/stream.rs:579:19)
  repro_xmlparser_src_stream_rs_605_34_376de4::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 0usize..0usize);
let _ = stream.slice_back(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/strspan.rs:50:9:
assertion failed: start &lt;= end
backtrace:
  &lt;xmlparser::strspan::StrSpan&gt;::from_substr (xmlparser/src/strspan.rs:50:9)
  &lt;xmlparser::strspan::StrSpan&gt;::slice_region (xmlparser/src/strspan.rs:89:9)
  &lt;xmlparser::stream::Stream&gt;::slice_back (xmlparser/src/stream.rs:298:19)
  repro_xmlparser_src_strspan_rs_50_9_c0ef4f::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 0usize..10000000000000000000usize);
let _ = stream.next_byte();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:136:12:
index out of bounds: the len is 0 but the index is 1
backtrace:
  &lt;xmlparser::stream::Stream&gt;::next_byte (xmlparser/src/stream.rs:136:12)
  repro_xmlparser_src_stream_rs_136_12_1b78f1::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "\u{200b}".to_string();
let stream = Stream::from_substr(&amp;string, 1usize..0usize);
let _ = stream.gen_text_pos();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:616:22:
end byte index 1 is not a char boundary; it is inside '\u{200b}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;xmlparser::stream::Stream&gt;::calc_curr_col (xmlparser/src/stream.rs:616:22)
  &lt;xmlparser::stream::Stream&gt;::gen_text_pos (xmlparser/src/stream.rs:580:19)
  repro_xmlparser_src_stream_rs_616_22_c442dc::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 1usize..0usize);
let mut stream = tokenizer.stream();
let _ = stream.skip_name();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:292:27:
start byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;xmlparser::stream::Stream&gt;::chars (xmlparser/src/stream.rs:292:27)
  &lt;xmlparser::stream::Stream&gt;::skip_name (xmlparser/src/stream.rs:447:29)
  repro_xmlparser_src_stream_rs_292_27_ac2d05::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 0usize..0usize);
let stream = tokenizer.stream();
let text_pos = stream.gen_text_pos_from(0usize);
let string2 = text_pos.to_string();
let tokenizer2 = Tokenizer::from_fragment(&amp;string2, 1usize..0usize);
let stream2 = tokenizer2.stream();
let _ = stream2.starts_with(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:172:29:
slice index starts at 1 but ends at 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;xmlparser::stream::Stream&gt;::starts_with (xmlparser/src/stream.rs:172:29)
  repro_xmlparser_src_stream_rs_172_29_a1dead::main (src/main.rs:11:21)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, usize::MAX..0usize);
let stream = tokenizer.stream();
let _ = stream.next_byte();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:132:12:
attempt to add with overflow
backtrace:
  &lt;xmlparser::stream::Stream&gt;::next_byte (xmlparser/src/stream.rs:132:12)
  repro_xmlparser_src_stream_rs_132_12_01e472::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 0usize..0usize);
let mut stream = tokenizer.stream();
stream.advance(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/stream.rs:152:9:
assertion failed: self.pos + n &lt;= self.end
backtrace:
  &lt;xmlparser::stream::Stream&gt;::advance (xmlparser/src/stream.rs:152:9)
  repro_xmlparser_src_stream_rs_152_9_5f5013::main (src/main.rs:7:12)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 0usize..1usize);
let _ = stream.slice_tail();</pre></td><td valign="top"><pre>thread 'main' panicked at xmlparser/src/strspan.rs:52:24:
end byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;xmlparser::strspan::StrSpan&gt;::from_substr (xmlparser/src/strspan.rs:52:24)
  &lt;xmlparser::strspan::StrSpan&gt;::slice_region (xmlparser/src/strspan.rs:89:9)
  &lt;xmlparser::stream::Stream&gt;::slice_tail (xmlparser/src/stream.rs:304:19)
  repro_xmlparser_src_strspan_rs_52_24_93fb38::main (src/main.rs:6:20)</pre></td></tr>
</table>
