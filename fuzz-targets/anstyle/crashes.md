# anstyle - crashes

Source: https://github.com/rust-cli/anstyle @ d891cdae9f5d9e923232bb42cecc597be09afe7a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut ascii_parser = AsciiParser;
let _ = &lt;anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator&gt;::add(
    &amp;mut ascii_parser, 0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at anstyle-parse/src/lib.rs:343:9:
internal error: entered unreachable code: multi-byte UTF8 characters are unsupported
backtrace:
  &lt;anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator&gt;::add (anstyle-parse/src/lib.rs:343:9)
  repro_anstyle_parse_src_lib_rs_343_9_5e41fd::main (src/main.rs:5:13)</pre></td></tr>
</table>
