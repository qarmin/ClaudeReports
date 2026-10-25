# zeno - crashes

Source: https://github.com/dfrg/zeno @ 698399439f8035736d01de00cbcdc413b3976a18

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let format = Format::CustomSubpixel([0.0f32, 0.0f32, 0.0f32]);
let _ = format.buffer_size(1u32, 1169106533u32);</pre></td><td valign="top"><pre>thread 'main' panicked at zeno/src/mask.rs:34:9:
attempt to multiply with overflow
backtrace:
  &lt;zeno::mask::Format&gt;::buffer_size (zeno/src/mask.rs:34:9)
  repro_zeno_src_mask_rs_34_9_16e438::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let format = Format::subpixel_bgra();
let _ = format.buffer_size(1000000000u32, 1000000000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at zeno/src/mask.rs:34:10:
attempt to multiply with overflow
backtrace:
  &lt;zeno::mask::Format&gt;::buffer_size (zeno/src/mask.rs:34:10)
  repro_zeno_src_mask_rs_34_10_871527::main (src/main.rs:5:20)</pre></td></tr>
</table>
