# libflate - crashes

Source: https://github.com/sile/libflate @ fd6cfb6b21328ee81ff95178aed45d64a94a86a3

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut lz77_decoder = Lz77Decoder::default();
let _ = lz77_decoder.decode(Code::Pointer { length: 0u16,
    backward_distance: 0u16 });</pre></td><td valign="top"><pre>thread 'main' panicked at rle-decode-fast/src/lib.rs:97:5:
attempt to repeat fragment of size 0
backtrace:
  rle_decode_fast::lookbehind_length_fail (rle-decode-fast/src/lib.rs:97:5)
  rle_decode_fast::rle_decode::&lt;u8&gt; (rle-decode-fast/src/lib.rs:41:9)
  &lt;libflate_lz77::Lz77Decoder&gt;::decode (libflate_lz77/src/lib.rs:186:17)
  repro_rle_decode_fast_src_lib_rs_97_5_f1c9aa::main (src/main.rs:6:26)</pre></td></tr>
</table>
