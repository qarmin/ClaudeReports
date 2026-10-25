# built - crashes

Source: https://github.com/lukaslueg/built @ 4ba8f7b0e4ceddecce72a0d170131dcd0a6497dc

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = built::util::detect_ci();</pre></td><td valign="top"><pre>thread 'main' panicked at built/src/environment.rs:55:33:
no entry found for key
backtrace:
  &lt;built::environment::EnvironmentMap&gt;::new (built/src/environment.rs:55:33)
  built::util::detect_ci (built/src/util.rs:61:5)
  repro_built_src_environment_rs_55_33_ce8500::main (src/main.rs:2:13)</pre></td></tr>
</table>
