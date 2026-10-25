# lofty-rs - crashes

Source: https://github.com/Serial-ATA/lofty-rs @ f88b02cfa787debffff99c341b54c3a77b83b832

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = PictureInformation::from_jpeg(&amp;vec![255u8, 216u8, 255u8, 224u8, 0u8,
    16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 0u8,
    0u8, 255u8, 192u8, 0u8, 17u8, 8u8, 36u8, 46u8, 39u8, 32u8, 34u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lofty/src/picture/mod.rs:474:44:
attempt to multiply with overflow
backtrace:
  &lt;lofty::picture::PictureInformation&gt;::from_jpeg (lofty/src/picture/mod.rs:474:29)
  repro_lofty_src_picture_mod_rs_474_44_4ed854::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PictureInformation::from_jpeg(&amp;vec![255u8, 216u8, 255u8, 224u8, 0u8,
    16u8, 74u8, 70u8, 73u8, 70u8, 0u8, 1u8, 2u8, 0u8, 0u8, 1u8, 0u8, 1u8, 255u8,
    255u8, 255u8, 255u8, 0u8, 0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lofty/src/picture/mod.rs:479:53:
attempt to subtract with overflow
backtrace:
  &lt;lofty::picture::PictureInformation&gt;::from_jpeg (lofty/src/picture/mod.rs:479:44)
  repro_lofty_src_picture_mod_rs_479_53_7edac3::main (src/main.rs:4:13)</pre></td></tr>
</table>
