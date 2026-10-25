# qoi - crashes

Source: https://github.com/aldanor/qoi-rust @ 81c14c4b637c326a801e40701460b654d46e8fbe

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let header = Header { width: u32::MAX, height: u32::MAX,
    channels: Channels::Rgb, colorspace: ColorSpace::default() };
let _ = header.n_bytes();</pre></td><td valign="top"><pre>thread 'main' panicked at qoi-rust/src/header.rs:110:9:
attempt to multiply with overflow
backtrace:
  &lt;qoi (qoi-rust/src/header.rs:110:9)
  repro_qoi_rust_src_header_rs_110_9_9bfabf::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = qoi::encode_max_len(1000000000u32, 1000000000u32, 32u8);</pre></td><td valign="top"><pre>thread 'main' panicked at qoi-rust/src/encode.rs:93:5:
attempt to add with overflow
backtrace:
  qoi::encode::encode_max_len::&lt;u8&gt; (qoi-rust/src/encode.rs:93:5)
  repro_qoi_rust_src_encode_rs_93_5_abffa0::main (src/main.rs:2:13)</pre></td></tr>
</table>
