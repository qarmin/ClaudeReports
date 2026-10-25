# rawler - crashes

Source: https://github.com/dnglab/dnglab @ ae01bcb2d0f8a74f9dfbb9f7b5c7e315c8e668b7

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let endian = Endian::default();
let _ = endian.read_i8(&amp;vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/bits.rs:90:5:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;rawler::bits::Endian&gt;::read_i8 (rawler/src/bits.rs:90:5)
  repro_rawler_src_bits_rs_90_5_ee1eaa::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = LookupTable::new_with_bits(&amp;[0u16], 10000u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/bits.rs:231:23:
attempt to shift left with overflow
backtrace:
  &lt;rawler::bits::LookupTable&gt;::new_with_bits (rawler/src/bits.rs:231:23)
  repro_rawler_src_bits_rs_231_23_dcf317::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rational::from(f32::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/formats/tiff/value.rs:239:57:
Failed to convert float
backtrace:
  &lt;rawler::formats::tiff::value::Rational as core::convert::From&lt;f32&gt;&gt;::from (rawler/src/formats/tiff/value.rs:239:57)
  repro_rawler_src_formats_tiff_value_rs_239_57_f08a2a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;rawler::pixarray::Pix2D&lt;f32&gt;&gt;::new_uninit(10000000000000000000usize,
    65536usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/pixarray.rs:77:35:
attempt to multiply with overflow
backtrace:
  &lt;rawler::pixarray::Pix2D&lt;f32&gt;&gt;::new_uninit (rawler/src/pixarray.rs:77:35)
  repro_rawler_src_pixarray_rs_77_35_64a27c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PlaneColor::new("a");</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/cfa.rs:314:41:
Invalid CFA color: "Unknown CFA color \"a\""
backtrace:
  &lt;rawler::cfa::PlaneColor&gt;::new (rawler/src/cfa.rs:314:41)
  repro_rawler_src_cfa_rs_314_41_3acdf3::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rational::from(54016850000000.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/formats/tiff/value.rs:242:32:
Rational numerator overflows u32: TryFromBigIntError { original: () }
backtrace:
  &lt;rawler::formats::tiff::value::Rational as core::convert::From&lt;f32&gt;&gt;::from (rawler/src/formats/tiff/value.rs:242:32)
  repro_rawler_src_formats_tiff_value_rs_242_32_51e9ac::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = CFA::new("a");</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/cfa.rs:124:12:
Unknown CFA size "a"
backtrace:
  &lt;rawler::cfa::CFA&gt;::new (rawler/src/cfa.rs:124:12)
  repro_rawler_src_cfa_rs_124_12_a0c181::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = rawler::imgop::chromatic_adaption::bradford_adaption_matrix(
    &amp;Illuminant::A, &amp;Illuminant::CloudyWeather);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/imgop/chromatic_adaption.rs:44:34:
not yet implemented
backtrace:
  rawler::imgop::chromatic_adaption::illuminant_to_XYZ_tristimulus (rawler/src/imgop/chromatic_adaption.rs:44:34)
  rawler::imgop::chromatic_adaption::bradford_adaption_matrix (rawler/src/imgop/chromatic_adaption.rs:64:25)
  repro_rawler_src_imgop_chromatic_adaption_rs_44_34_f9209f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>rawler::devtools::dump_buf::&lt;Vec&lt;u8&gt;&gt;("", vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/devtools/mod.rs:39:49:
Unable to create file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
backtrace:
  rawler::devtools::dump_buf::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (rawler/src/devtools/mod.rs:39:49)
  repro_rawler_src_devtools_mod_rs_39_49_0ed717::main (src/main.rs:2:5)</pre></td></tr>
<tr><td valign="top"><pre>let value = Value::from(CompressionMethod::Huffman);
let _ = value.get_data();</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/formats/tiff/value.rs:438:9:
Unable to call get_data() on this value type
backtrace:
  &lt;rawler::formats::tiff::value::Value&gt;::get_data (rawler/src/formats/tiff/value.rs:438:9)
  repro_rawler_src_formats_tiff_value_rs_438_9_ca4093::main (src/main.rs:6:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Rect::new_with_points(Point::new(1usize, 0usize), Point::new(0usize,
    0usize));</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/imgop/mod.rs:130:5:
assertion failed: p1.x &lt;= p2.x
backtrace:
  &lt;rawler::imgop::Rect&gt;::new_with_points (rawler/src/imgop/mod.rs:130:5)
  repro_rawler_src_imgop_mod_rs_130_5_f31ce0::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let stsc_box = StscBox { header: BoxHeader::default(), version: 0u8,
    flags: 0u32, entries: vec![] };
let _ = stsc_box.get_entry_for_sample(1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/formats/bmff/stsc.rs:31:5:
assertion `left == right` failed: stsc box must contains at least one entry
  left: true
 right: false
backtrace:
  &lt;rawler::formats::bmff::stsc::StscBox&gt;::get_entry_for_sample (rawler/src/formats/bmff/stsc.rs:31:5)
  repro_rawler_src_formats_bmff_stsc_rs_31_5_5548e3::main (src/main.rs:6:22)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let _ = LjpegDecompressor::new_full(&amp;vec, false, false);</pre></td><td valign="top"><pre>thread 'main' panicked at rawler/src/pumps.rs:345:5:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;rawler::pumps::ByteStream&gt;::peek_u8 (rawler/src/pumps.rs:345:5)
  &lt;rawler::pumps::ByteStream&gt;::get_u8 (rawler/src/pumps.rs:349:20)
  &lt;rawler::decompressors::ljpeg::LjpegDecompressor&gt;::get_next_marker (rawler/src/decompressors/ljpeg/mod.rs:254:24)
  &lt;rawler::decompressors::ljpeg::LjpegDecompressor&gt;::new_full (rawler/src/decompressors/ljpeg/mod.rs:182:8)
  repro_rawler_src_pumps_rs_345_5_adcbcb::main (src/main.rs:5:13)</pre></td></tr>
</table>
