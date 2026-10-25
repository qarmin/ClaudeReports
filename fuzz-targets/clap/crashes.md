# clap - crashes

Source: https://github.com/clap-rs/clap @ ccc89b83875474399e9d9d70841163f6319d1a41

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ValueRange::from(usize::MAX..12usize);</pre></td><td valign="top"><pre>thread 'main' panicked at clap_builder/src/builder/range.rs:69:9:
assertion failed: start_inclusive &lt;= end_inclusive
backtrace:
  &lt;clap_builder::builder::range::ValueRange&gt;::raw (clap_builder/src/builder/range.rs:69:9)
  &lt;clap_builder::builder::range::ValueRange as core::convert::From&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::from (clap_builder/src/builder/range.rs:150:9)
  repro_clap_builder_src_builder_range_rs_69_9_54892c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let error = &lt;clap_builder::error::Error&lt;clap_builder::error::KindFormatter&gt;&gt;::
    raw(ErrorKind::InvalidUtf8, "".to_string());
let error2 = error.apply::&lt;clap_builder::error::RichFormatter&gt;();
let _ = error2.exit();</pre></td><td valign="top"><pre>error:</pre></td></tr>
<tr><td valign="top"><pre>let error = &lt;clap_builder::error::Error&lt;clap_builder::error::KindFormatter&gt;&gt;::
    raw(ErrorKind::ValueValidation, "".to_string());
let _ = error.exit();</pre></td><td valign="top"><pre>error:</pre></td></tr>
<tr><td valign="top"><pre>let raw_args = RawArgs::default();
let raw_args2 = RawArgs::default();
let mut arg_cursor = raw_args2.cursor();
let _ = raw_args.next(&amp;mut arg_cursor);
let _ = raw_args2.remaining(&amp;mut arg_cursor);</pre></td><td valign="top"><pre>thread 'main' panicked at clap_lex/src/lib.rs:228:35:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[std::ffi::os_str::OsString] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;clap_lex::RawArgs&gt;::remaining (clap_lex/src/lib.rs:228:35)
  repro_clap_lex_src_lib_rs_228_35_768990::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let command = Command::new(Str::default());
let _ = command.clone().visible_short_flag_aliases(vec!['-']);</pre></td><td valign="top"><pre>thread 'main' panicked at clap_builder/src/builder/command.rs:3025:13:
short alias name cannot be `-`
backtrace:
  &lt;clap_builder::builder::command::Command&gt;::visible_short_flag_aliases::&lt;alloc::vec::Vec&lt;char&gt;&gt; (clap_builder/src/builder/command.rs:3025:13)
  repro_clap_builder_src_builder_command_rs_3025_13_e2a9e8::main (src/main.rs:6:29)</pre></td></tr>
<tr><td valign="top"><pre>let arg = Arg::default();
let _ = arg.clone().short_aliases(vec!['-']);</pre></td><td valign="top"><pre>thread 'main' panicked at clap_builder/src/builder/arg.rs:348:13:
short alias name cannot be `-`
backtrace:
  &lt;clap_builder::builder::arg::Arg&gt;::short_aliases::&lt;alloc::vec::Vec&lt;char&gt;&gt; (clap_builder/src/builder/arg.rs:348:13)
  repro_clap_builder_src_builder_arg_rs_348_13_096c70::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let arg = Arg::new(Id::default());
let _ = arg.clone().visible_short_aliases(vec!['-']);</pre></td><td valign="top"><pre>thread 'main' panicked at clap_builder/src/builder/arg.rs:463:13:
short alias name cannot be `-`
backtrace:
  &lt;clap_builder::builder::arg::Arg&gt;::visible_short_aliases::&lt;alloc::vec::Vec&lt;char&gt;&gt; (clap_builder/src/builder/arg.rs:463:13)
  repro_clap_builder_src_builder_arg_rs_463_13_bcb34a::main (src/main.rs:6:25)</pre></td></tr>
</table>
