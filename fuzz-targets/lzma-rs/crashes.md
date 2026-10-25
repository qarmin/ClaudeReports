# lzma-rs - crashes

Source: https://github.com/gendx/lzma-rs @ 1f14478def43b8ea0d27a6b3db88bb307aafd18a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = LzmaDecoder::new(LzmaParams::new(LzmaProperties { lc: 0u32, lp: 1u32,
    pb: 100u32 }, 0u32, None), None);</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rs/src/decode/lzma.rs:65:9:
assertion failed: self.pb &lt;= 4
backtrace:
  &lt;lzma_rs::decode::lzma::LzmaProperties&gt;::validate (lzma-rs/src/decode/lzma.rs:65:9)
  &lt;lzma_rs::decode::lzma::DecoderState&gt;::new (lzma-rs/src/decode/lzma.rs:189:20)
  &lt;lzma_rs::decode::lzma::LzmaDecoder&gt;::new (lzma-rs/src/decode/lzma.rs:611:20)
  repro_lzma_rs_src_decode_lzma_rs_65_9_141ec2::main (src/main.rs:6:13)</pre></td></tr>
</table>
