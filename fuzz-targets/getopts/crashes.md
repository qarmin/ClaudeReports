# getopts - crashes

Source: https://github.com/rust-lang/getopts @ c0cdd215f184c920adca2aba7a2086d38f468119

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut options = Options::default();
let _ = options.optmulti("", "a", "", "");</pre></td><td valign="top"><pre>thread 'main' panicked at getopts/src/lib.rs:705:5:
the long_name (second argument) should be longer than a single character, or an empty string for none
backtrace:
  getopts::validate_names (getopts/src/lib.rs:705:5)
  &lt;getopts::Options&gt;::optmulti (getopts/src/lib.rs:331:9)
  repro_getopts_src_lib_rs_705_5_e11618::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut options = Options::default();
let fail = Fail::OptionDuplicated("".to_string());
let string = fail.to_string();
let _ = options.optflag("م", &amp;string, "");</pre></td><td valign="top"><pre>thread 'main' panicked at getopts/src/lib.rs:699:5:
the short_name (first argument) should be a single character, or an empty string for none
backtrace:
  getopts::validate_names (getopts/src/lib.rs:699:5)
  &lt;getopts::Options&gt;::optflag (getopts/src/lib.rs:219:9)
  repro_getopts_src_lib_rs_699_5_9a8b55::main (src/main.rs:8:21)</pre></td></tr>
</table>
