# flate2 - crashes

Source: https://github.com/rust-lang/flate2-rs @ 31d6aef2a1478b61d8aeb5931d74f34761add25a

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut compress = Compress::new(Compression::default(), false);
let _ = compress.set_level(Compression::new(1000000000u32));</pre></td><td valign="top"><pre>thread 'main' panicked at flate2-rs/src/mem.rs:338:22:
unknown return code: -2
backtrace:
  &lt;flate2::mem::Compress&gt;::set_level (flate2-rs/src/mem.rs:338:22)
  repro_flate2_rs_src_mem_rs_338_22_bd4a40::main (src/main.rs:6:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;flate2::write::GzEncoder&lt;Vec&lt;u8&gt;&gt;&gt;::new(vec![],
    Compression::new(1000u32));</pre></td><td valign="top"><pre>thread 'main' panicked at flate2-rs/src/ffi/c.rs:369:13:
assertion `left == right` failed
  left: -2
 right: 0
backtrace:
  &lt;flate2::ffi::c::Deflate as flate2::ffi::DeflateBackend&gt;::make (flate2-rs/src/ffi/c.rs:369:13)
  &lt;flate2::mem::Compress&gt;::new (flate2-rs/src/mem.rs:201:20)
  flate2::gz::write::gz_encoder::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (flate2-rs/src/gz/write.rs:55:36)
  &lt;flate2::gz::GzBuilder&gt;::write::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (flate2-rs/src/gz/mod.rs:392:9)
  &lt;flate2::gz::write::GzEncoder&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::new (flate2-rs/src/gz/write.rs:71:26)
  repro_flate2_rs_src_ffi_c_rs_369_13_3dd9d7::main (src/main.rs:4:13)</pre></td></tr>
</table>
