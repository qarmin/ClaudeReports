# mp4parse - crashes

Source: https://github.com/mozilla/mp4parse-rust @ 95d98b9a6840f5bfc9a2323b3fa3f56d7f3c6077

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Error::from(Status::Oom);</pre></td><td valign="top"><pre>thread 'main' panicked at mp4parse/src/lib.rs:415:17:
Status -&gt; Error is only for Status:InvalidData errors
backtrace:
  &lt;mp4parse::Error as core::convert::From&lt;mp4parse::Status&gt;&gt;::from (mp4parse/src/lib.rs:415:17)
  repro_mp4parse_src_lib_rs_415_17_3dc9b3::main (src/main.rs:5:13)</pre></td></tr>
</table>
