# jxl-rs - crashes

Source: https://github.com/libjxl/jxl-rs @ 7f539c803ddd9c5d2eb654de0814d8595e2225b2

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut jxl_decoder = JxlDecoder::new(JxlDecoderOptions::default());
let _ = jxl_decoder.set_pixel_format(JxlPixelFormat::rgba_f16(
    1000000000000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2000000000000000000 bytes failed
backtrace:
  jxl::api::data_types::JxlPixelFormat::rgba_f16 (src/api/data_types.rs:211)
  repro_out_of_memory_in_jxl_api_jxldecoder_set_pixel_format_450c70::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
</table>
