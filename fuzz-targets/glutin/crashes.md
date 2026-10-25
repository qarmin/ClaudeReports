# glutin - crashes

Source: https://github.com/rust-windowing/glutin @ 940a394cc205f152eb12076feff59f1eaaff4f7a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let config_template_builder = ConfigTemplateBuilder::default();
let config_template_builder2 = config_template_builder.clone().with_transparency(
    false);
let _ = config_template_builder2.clone().with_multisampling(0u8);</pre></td><td valign="top"><pre>thread 'main' panicked at glutin/src/config.rs:144:9:
assertion failed: num_samples.is_power_of_two()
backtrace:
  &lt;glutin::config::ConfigTemplateBuilder&gt;::with_multisampling (glutin/src/config.rs:144:9)
  repro_glutin_src_config_rs_144_9_053d61::main (src/main.rs:6:46)</pre></td></tr>
</table>
