# htmlparser - crashes

Source: https://github.com/jdrouet/htmlparser @ 036a1f71cc277094f34b67d53c353a8634fab91b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, usize::MAX..0usize);
let _ = stream.next_byte();</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:130:12:
attempt to add with overflow
backtrace:
  &lt;htmlparser::stream::Stream&gt;::next_byte (htmlparser/src/stream.rs:130:12)
  repro_htmlparser_src_stream_rs_130_12_6e4adf::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 0usize..1000000000usize);
let stream = tokenizer.stream();
let _ = stream.next_byte();</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:134:12:
index out of bounds: the len is 0 but the index is 1
backtrace:
  &lt;htmlparser::stream::Stream&gt;::next_byte (htmlparser/src/stream.rs:134:12)
  repro_htmlparser_src_stream_rs_134_12_0e412b::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let mut stream = Stream::from_substr(&amp;string, usize::MAX..0usize);
stream.advance(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:150:23:
attempt to add with overflow
backtrace:
  &lt;htmlparser::stream::Stream&gt;::advance (htmlparser/src/stream.rs:150:23)
  repro_htmlparser_src_stream_rs_150_23_8dda40::main (src/main.rs:6:12)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 1usize..0usize);
let stream = tokenizer.stream();
let _ = stream.gen_text_pos();</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:621:34:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;htmlparser::stream::Stream&gt;::calc_curr_row (htmlparser/src/stream.rs:621:34)
  &lt;htmlparser::stream::Stream&gt;::gen_text_pos (htmlparser/src/stream.rs:595:19)
  repro_htmlparser_src_stream_rs_621_34_90b72e::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 0usize..1usize);
let mut stream = tokenizer.stream();
let _ = stream.consume_name();</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:290:27:
end byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;htmlparser::stream::Stream&gt;::chars (htmlparser/src/stream.rs:290:27)
  &lt;htmlparser::stream::Stream&gt;::skip_name (htmlparser/src/stream.rs:445:29)
  &lt;htmlparser::stream::Stream&gt;::consume_name (htmlparser/src/stream.rs:427:14)
  repro_htmlparser_src_stream_rs_290_27_35a280::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let tokenizer = Tokenizer::from_fragment(&amp;string, 1usize..0usize);
let stream = tokenizer.stream();
let _ = stream.slice_back(1000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/strspan.rs:50:9:
assertion failed: start &lt;= end
backtrace:
  &lt;htmlparser::strspan::StrSpan&gt;::from_substr (htmlparser/src/strspan.rs:50:9)
  &lt;htmlparser::strspan::StrSpan&gt;::slice_region (htmlparser/src/strspan.rs:89:9)
  &lt;htmlparser::stream::Stream&gt;::slice_back (htmlparser/src/stream.rs:296:19)
  repro_htmlparser_src_strspan_rs_50_9_169e66::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let mut stream = Stream::from_substr(&amp;string, 0usize..0usize);
stream.advance(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:150:9:
assertion failed: self.pos + n &lt;= self.end
backtrace:
  &lt;htmlparser::stream::Stream&gt;::advance (htmlparser/src/stream.rs:150:9)
  repro_htmlparser_src_stream_rs_150_9_f0cd20::main (src/main.rs:6:12)</pre></td></tr>
<tr><td valign="top"><pre>let string = "Zażółć".to_string();
let mut stream = Stream::from_substr(&amp;string,
    9usize..10000000000000000000usize);
let _ = stream.consume_quote();</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:632:22:
end byte index 9 is not a char boundary; it is inside 'ć' (bytes 8..10 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::RangeTo&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;htmlparser::stream::Stream&gt;::calc_curr_col (htmlparser/src/stream.rs:632:22)
  &lt;htmlparser::stream::Stream&gt;::gen_text_pos (htmlparser/src/stream.rs:596:19)
  &lt;htmlparser::stream::Stream&gt;::consume_quote (htmlparser/src/stream.rs:583:51)
  repro_htmlparser_src_stream_rs_632_22_1e7bf9::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 1usize..0usize);
let _ = stream.slice_back(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/strspan.rs:52:24:
end byte index 1 is out of bounds for string of length 0
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;htmlparser::strspan::StrSpan&gt;::from_substr (htmlparser/src/strspan.rs:52:24)
  &lt;htmlparser::strspan::StrSpan&gt;::slice_region (htmlparser/src/strspan.rs:89:9)
  &lt;htmlparser::stream::Stream&gt;::slice_back (htmlparser/src/stream.rs:296:19)
  repro_htmlparser_src_strspan_rs_52_24_373806::main (src/main.rs:6:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let stream = Stream::from_substr(&amp;string, 0usize..1usize);
let _ = stream.starts_with(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at htmlparser/src/stream.rs:170:29:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;htmlparser::stream::Stream&gt;::starts_with (htmlparser/src/stream.rs:170:29)
  repro_htmlparser_src_stream_rs_170_29_6bac37::main (src/main.rs:6:20)</pre></td></tr>
</table>
