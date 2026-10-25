# glow - crashes

Source: https://github.com/grovesNL/glow @ 62250dc64e7e531c280beea251f39aa49ba0b2f1

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = glow::compute_size(1000000000i32, -1000000000i32, 0u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at glow/src/lib.rs:2000:5:
attempt to multiply with overflow
backtrace:
  glow::compute_size (glow/src/lib.rs:2000:5)
  repro_glow_src_lib_rs_2000_5_13a47c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = glow::bytes_per_type(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at glow/src/lib.rs:1995:14:
unsupported pixel type: 0
backtrace:
  glow::bytes_per_type (glow/src/lib.rs:1995:14)
  repro_glow_src_lib_rs_1995_14_0ff307::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let u32_v = MAX_TESS_CONTROL_IMAGE_UNIFORMS;
let _ = glow::components_per_format(u32_v);</pre></td><td valign="top"><pre>thread 'main' panicked at glow/src/lib.rs:1982:14:
unsupported format: 37067
backtrace:
  glow::components_per_format (glow/src/lib.rs:1982:14)
  repro_glow_src_lib_rs_1982_14_df85d8::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ShaderPrecisionFormat::common_desktop_hardware(0u32, false);</pre></td><td valign="top"><pre>thread 'main' panicked at glow/src/lib.rs:89:18:
internal error: entered unreachable code: invalid precision
backtrace:
  &lt;glow::ShaderPrecisionFormat&gt;::common_desktop_hardware (glow/src/lib.rs:89:18)
  repro_glow_src_lib_rs_89_18_dd5ae1::main (src/main.rs:4:13)</pre></td></tr>
</table>
