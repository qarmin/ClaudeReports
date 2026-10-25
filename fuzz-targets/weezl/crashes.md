# weezl - crashes

Source: https://github.com/image-rs/weezl @ 606f9c79b054176ed409152a4852b2e047460d2b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Configuration::with_tiff_size_switch(BitOrder::Lsb, 100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at weezl/src/lib.rs:96:5:
Maximum code size 12 required, got 100
backtrace:
  weezl::assert_decode_size (weezl/src/lib.rs:96:5)
  &lt;weezl::decode::Configuration&gt;::with_tiff_size_switch (weezl/src/decode.rs:238:9)
  repro_weezl_src_lib_rs_96_5_555e52::main (src/main.rs:5:13)</pre></td></tr>
</table>
