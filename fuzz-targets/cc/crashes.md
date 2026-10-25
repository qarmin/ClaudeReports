# cc - crashes

Source: https://github.com/rust-lang/cc-rs @ 7c27a1ab65e9f5909ed48eabf6bb4483876b9218

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let build = Build::new();
let _ = build.get_compiler();</pre></td><td valign="top"><pre>error occurred in cc-rs: environment variable OPT_LEVEL not defined</pre></td></tr>
<tr><td valign="top"><pre>let build = Build::default();
let _ = build.compile_intermediates();</pre></td><td valign="top"><pre>error occurred in cc-rs: Environment variable OUT_DIR not defined.</pre></td></tr>
<tr><td valign="top"><pre>let build = Build::new();
build.compile("");</pre></td><td valign="top"><pre>error occurred in cc-rs: argument of `compile` must be a single normal path component</pre></td></tr>
<tr><td valign="top"><pre>let build = Build::new();
let _ = build.get_ranlib();</pre></td><td valign="top"><pre>error occurred in cc-rs: environment variable TARGET not defined</pre></td></tr>
<tr><td valign="top"><pre>let build = Build::new();
let _ = build.get_archiver();</pre></td><td valign="top"><pre>error occurred in cc-rs: environment variable TARGET not defined</pre></td></tr>
<tr><td valign="top"><pre>let build = Build::default();
let _ = build.expand();</pre></td><td valign="top"><pre>error occurred in cc-rs: environment variable OPT_LEVEL not defined</pre></td></tr>
</table>
