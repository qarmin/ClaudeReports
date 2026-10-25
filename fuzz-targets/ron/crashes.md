# ron - crashes

Source: https://github.com/ron-rs/ron @ 7b32209108a39f76192df7ef15701b3a48cd7a5f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let span = Span { start: Position { line: 1usize, col: 10usize },
    end: Position { line: 1usize, col: 1usize } };
let _ = span.substring_exclusive("aaaaaaaaa");</pre></td><td valign="top"><pre>thread 'main' panicked at ron/src/util/span_substring.rs:63:58:
slice index starts at 9 but ends at 0
backtrace:
  &lt;[&amp;str] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;ron::error::Span&gt;::substring_exclusive (ron/src/util/span_substring.rs:63:58)
  repro_ron_src_util_span_substring_rs_63_58_b588f8::main (src/main.rs:6:18)</pre></td></tr>
<tr><td valign="top"><pre>let span = Span { start: Position { line: 1usize, col: 10usize },
    end: Position { line: 1usize, col: 1usize } };
let _ = span.substring_inclusive("a5a0aaa7a");</pre></td><td valign="top"><pre>thread 'main' panicked at ron/src/util/span_substring.rs:78:58:
slice index starts at 9 but ends at 1
backtrace:
  &lt;[&amp;str] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;ron::error::Span&gt;::substring_inclusive (ron/src/util/span_substring.rs:78:58)
  repro_ron_src_util_span_substring_rs_78_58_dfd178::main (src/main.rs:6:18)</pre></td></tr>
</table>
