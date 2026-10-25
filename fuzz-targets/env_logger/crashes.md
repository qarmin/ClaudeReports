# env_logger - crashes

Source: https://github.com/rust-cli/env_logger @ f80dca81223bb7cd50fc81f484e20eaef22be0aa

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut builder = Builder::new();
let _ = builder.build();
let _ = builder.try_init();</pre></td><td valign="top"><pre>thread 'main' panicked at env_logger/src/logger.rs:530:9:
attempt to re-use consumed builder
backtrace:
  &lt;env_logger::logger::Builder&gt;::build (env_logger/src/logger.rs:530:9)
  &lt;env_logger::logger::Builder&gt;::try_init (env_logger/src/logger.rs:499:27)
  repro_env_logger_src_logger_rs_530_9_a785b1::main (src/main.rs:6:21)</pre></td></tr>
</table>
