# y4m - crashes

Source: https://github.com/image-rs/y4m @ 76635a27866db6bad1ec5f9b08711dfa908f2e6d

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let encoder_builder = EncoderBuilder::new(0usize, usize::MAX,
    Ratio { num: 0usize, den: 1usize });
let encoder_builder2 = encoder_builder.with_colorspace(Colorspace::C422p10);
let encoder_builder3 = encoder_builder2.append_vendor_extension(
    VendorExtensionString::new(vec![]).unwrap());
let _ = encoder_builder3.write_header::&lt;Vec&lt;u8&gt;&gt;(vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at y4m/src/lib.rs:299:30:
attempt to add with overflow
backtrace:
  y4m::get_plane_sizes (y4m/src/lib.rs:299:30)
  &lt;y4m::EncoderBuilder&gt;::write_header::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (y4m/src/lib.rs:627:37)
  repro_y4m_src_lib_rs_299_30_2d08b1::main (src/main.rs:10:30)</pre></td></tr>
<tr><td valign="top"><pre>let encoder_builder = EncoderBuilder::new(usize::MAX, 12usize,
    Ratio::new(1usize, 0usize));
let _ = encoder_builder.write_header::&lt;Vec&lt;u8&gt;&gt;(include_bytes!(
    "mutated_0_Cargo.toml").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at y4m/src/lib.rs:296:24:
attempt to multiply with overflow
backtrace:
  y4m::get_plane_sizes (y4m/src/lib.rs:296:24)
  &lt;y4m::EncoderBuilder&gt;::write_header::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (y4m/src/lib.rs:627:37)
  repro_y4m_src_lib_rs_296_24_46cf82::main (src/main.rs:6:29)</pre></td></tr>
<tr><td valign="top"><pre>let encoder_builder = y4m::encode(usize::MAX, 0usize, Ratio { num: 0usize,
    den: 0usize });
let _ = encoder_builder.write_header::&lt;Vec&lt;u8&gt;&gt;(vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at y4m/src/lib.rs:299:10:
attempt to add with overflow
backtrace:
  y4m::get_plane_sizes (y4m/src/lib.rs:299:10)
  &lt;y4m::EncoderBuilder&gt;::write_header::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (y4m/src/lib.rs:627:37)
  repro_y4m_src_lib_rs_299_10_f14f26::main (src/main.rs:5:29)</pre></td></tr>
</table>
