# rspolib - crashes

Source: https://github.com/mondeja/rspolib @ 12343ca23f0c9cbe6d7a2cdcfbf5881a6cb1fbe8

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = rspolib::mofile::&lt;rspolib::FileOptions&gt;(
    FileOptions { path_or_content: "".to_string(), wrapwidth: 0usize,
    byte_content: None });</pre></td><td valign="top"><pre>thread 'main' panicked at rust/src/moparser.rs:83:52:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;rspolib::moparser::MOFileParser&gt;::new (rust/src/moparser.rs:83:52)
  rspolib::file::mofile::mofile::&lt;rspolib::file::FileOptions&gt; (rust/src/file/mofile.rs:71:22)
  repro_rust_src_moparser_rs_83_52_4c27dd::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let p_o_entry = POEntry { msgid: "".to_string(), msgstr: None,
    msgid_plural: None, msgstr_plural: vec![], msgctxt: None, obsolete: false,
    comment: Some("a".to_string()), tcomment: None, occurrences: vec![],
    flags: vec![], previous_msgid: None, previous_msgid_plural: None,
    previous_msgctxt: None, linenum: 0usize };
let _ = p_o_entry.to_string_with_wrapwidth(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust/src/entry/poentry.rs:99:45:
attempt to subtract with overflow
backtrace:
  &lt;rspolib::entry::poentry::POEntry&gt;::format_comment_inplace (rust/src/entry/poentry.rs:99:45)
  &lt;rspolib::entry::poentry::POEntry&gt;::to_string_with_wrapwidth (rust/src/entry/poentry.rs:124:18)
  repro_rust_src_entry_poentry_rs_99_45_58ce14::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let file_options = FileOptions { path_or_content: "\"".to_string(),
    wrapwidth: 0usize, byte_content: None };
let _ = rspolib::pofile::&lt;rspolib::FileOptions&gt;(file_options.clone());</pre></td><td valign="top"><pre>thread 'main' panicked at rust/src/poparser.rs:849:18:
byte range starts at 1 but ends at 0
backtrace:
  rspolib::poparser::maybe_raise_unescaped_double_quote_found_error (rust/src/poparser.rs:849:18)
  &lt;rspolib::poparser::POFileParser&gt;::parse_line (rust/src/poparser.rs:381:13)
  &lt;rspolib::poparser::POFileParser&gt;::parse_with_handler (rust/src/poparser.rs:262:14)
  &lt;rspolib::poparser::POFileParser&gt;::parse_content (rust/src/poparser.rs:253:14)
  &lt;rspolib::poparser::POFileParser&gt;::parse (rust/src/poparser.rs:235:18)
  rspolib::file::pofile::pofile::&lt;rspolib::file::FileOptions&gt; (rust/src/file/pofile.rs:140:12)
  repro_rust_src_poparser_rs_849_18_93f10a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rspolib::pofile::&lt;rspolib::FileOptions&gt;(
    FileOptions { path_or_content: "\"\u{5cac8}\"".to_string(),
    wrapwidth: 0usize, byte_content: None });</pre></td><td valign="top"><pre>thread 'main' panicked at rust/src/poparser.rs:220:48:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;rspolib::poparser::POFileParser&gt;::process (rust/src/poparser.rs:220:48)
  &lt;rspolib::poparser::POFileParser&gt;::parse_line (rust/src/poparser.rs:388:18)
  &lt;rspolib::poparser::POFileParser&gt;::parse_with_handler (rust/src/poparser.rs:262:14)
  &lt;rspolib::poparser::POFileParser&gt;::parse_content (rust/src/poparser.rs:253:14)
  &lt;rspolib::poparser::POFileParser&gt;::parse (rust/src/poparser.rs:235:18)
  rspolib::file::pofile::pofile::&lt;rspolib::file::FileOptions&gt; (rust/src/file/pofile.rs:140:12)
  repro_rust_src_poparser_rs_220_48_d2deae::main (src/main.rs:4:13)</pre></td></tr>
</table>
