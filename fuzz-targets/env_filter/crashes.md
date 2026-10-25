# env_filter - crashes

Source: https://github.com/rust-cli/env_logger @ f80dca81223bb7cd50fc81f484e20eaef22be0aa

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new();
let _ = builder.build();
let _ = builder.build();</pre></td><td valign="top"><pre>thread 'main' panicked at env_filter/src/filter.rs:143:9:
attempt to re-use consumed builder
backtrace:
  &lt;env_filter::filter::Builder&gt;::build (env_filter/src/filter.rs:143:9)
  repro_env_filter_src_filter_rs_143_9_47197e::main (src/main.rs:6:21)</pre></td></tr>
</table>
