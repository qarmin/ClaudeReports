# cssparser - crashes

Source: https://github.com/servo/rust-cssparser @ 9f3ceddf398f5ce4ab76fdad1b42ccb6c578a926

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let parser = Parser::new(&amp;string);
let parser_state = parser.state();
let string2 = "Y".to_string();
let mut parser2 = Parser::new(&amp;string2);
let _ = cssparser::parse_nth(&amp;mut parser2);
let _ = parser.slice(parser_state.position()..parser2.position());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-cssparser/src/tokenizer.rs:298:9:
assertion failed: self.input.is_char_boundary(range.end.0)
backtrace:
  &lt;cssparser::parser::Parser&gt;::slice (rust-cssparser/src/tokenizer.rs:298:9)
  repro_rust_cssparser_src_tokenizer_rs_298_9_730318::main (src/main.rs:10:20)</pre></td></tr>
<tr><td valign="top"><pre>let string = "a".to_string();
let mut parser = Parser::new(&amp;string);
let _ = cssparser::parse_nth(&amp;mut parser);
let string2 = "".to_string();
let parser2 = Parser::new(&amp;string2);
let source_position = parser.position();
let _ = parser2.slice_from(source_position);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-cssparser/src/tokenizer.rs:297:9:
assertion failed: self.input.is_char_boundary(range.start.0)
backtrace:
  &lt;cssparser::parser::Parser&gt;::slice (rust-cssparser/src/tokenizer.rs:297:9)
  &lt;cssparser::parser::Parser&gt;::slice_from (rust-cssparser/src/tokenizer.rs:291:14)
  repro_rust_cssparser_src_tokenizer_rs_297_9_2545d4::main (src/main.rs:10:21)</pre></td></tr>
</table>
