# resvg - crashes

Source: https://github.com/linebender/resvg @ 978775b5f2d31f35b370e0ac4059038be117101c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = StrokeMiterlimit::new(0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at usvg/src/tree/mod.rs:512:9:
assertion failed: n &gt;= 1.0
backtrace:
  &lt;usvg::tree::StrokeMiterlimit&gt;::new (usvg/src/tree/mod.rs:512:9)
  repro_usvg_src_tree_mod_rs_512_9_06c3c8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StrokeMiterlimit::from(f32::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at usvg/src/tree/mod.rs:511:9:
assertion failed: n.is_finite()
backtrace:
  &lt;usvg::tree::StrokeMiterlimit&gt;::new (usvg/src/tree/mod.rs:511:9)
  &lt;usvg::tree::StrokeMiterlimit as core::convert::From&lt;f32&gt;&gt;::from (usvg/src/tree/mod.rs:536:9)
  repro_usvg_src_tree_mod_rs_511_9_61e5b6::main (src/main.rs:4:13)</pre></td></tr>
</table>
