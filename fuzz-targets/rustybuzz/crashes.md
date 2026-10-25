# rustybuzz - crashes

Source: https://github.com/harfbuzz/rustybuzz @ 9faca967408677f17bc15366dbdc3d2b683d1489

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let _ = Face::from_slice(&amp;vec, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rustybuzz/src/hb/set_digest.rs:93:12:
attempt to subtract with overflow
backtrace:
  &lt;rustybuzz (rustybuzz/src/hb/set_digest.rs:93:12)
  &lt;rustybuzz (rustybuzz/src/hb/set_digest.rs:158:31)
  &lt;rustybuzz (rustybuzz/src/hb/set_digest.rs:159:32)
  &lt;ttf_parser::ggg::Coverage as rustybuzz (rustybuzz/src/hb/ot_layout_common.rs:49:32)
  &lt;rustybuzz (rustybuzz/src/hb/ot_layout_common.rs:116:33)
  &lt;&lt;rustybuzz (core/src/ops/function.rs:166:5)
  &lt;&amp;mut &lt;rustybuzz (core/src/ops/function.rs:310:21)
  &lt;rustybuzz (rustybuzz/src/hb/ot_layout_common.rs:91:73)
  &lt;&lt;rustybuzz (core/src/ops/function.rs:250:5)
  &lt;rustybuzz (rustybuzz/src/hb/face.rs:88:38)
  &lt;rustybuzz (rustybuzz/src/hb/face.rs:76:14)
  repro_rustybuzz_src_hb_set_digest_rs_93_12_a2f048::main (src/main.rs:5:13)</pre></td></tr>
</table>
