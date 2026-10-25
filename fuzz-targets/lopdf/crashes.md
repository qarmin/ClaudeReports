# lopdf - crashes

Source: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = FontData::new(&amp;vec![], "".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/font.rs:89:52:
Failed to parse font file: OutOfBounds
backtrace:
  &lt;lopdf::font::FontData&gt;::new (lopdf/src/font.rs:89:52)
  repro_lopdf_src_font_rs_89_52_209725::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::encode_row(FilterType::Avg, 1usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:140:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::encode_row (lopdf/src/filters/png.rs:140:54)
  repro_lopdf_src_filters_png_rs_140_54_fcfe5d::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::decode_row(FilterType::Paeth, 1usize, &amp;vec![0u8],
    &amp;mut vec![0u8, 80u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:82:86:
index out of bounds: the len is 1 but the index is 1
backtrace:
  lopdf::filters::png::decode_row (lopdf/src/filters/png.rs:82:86)
  repro_lopdf_src_filters_png_rs_82_86_2d2223::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::encode_row(FilterType::Paeth, usize::MAX,
    &amp;include_bytes!("image_0.png").to_vec(),
    &amp;mut include_bytes!("image_1.png").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:149:71:
index out of bounds: the len is 31 but the index is 31
backtrace:
  lopdf::filters::png::encode_row (lopdf/src/filters/png.rs:149:71)
  repro_lopdf_src_filters_png_rs_149_71_d0a8e0::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;lopdf::encryption::crypt_filters::Rc4CryptFilter as lopdf::encryption::
    crypt_filters::CryptFilter&gt;::decrypt(&amp;Rc4CryptFilter, &amp;vec![], &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/encryption/rc4.rs:10:9:
assertion failed: !key.is_empty() &amp;&amp; key.len() &lt;= 256
backtrace:
  &lt;lopdf::encryption::rc4::Rc4&gt;::new::&lt;&amp;[u8]&gt; (lopdf/src/encryption/rc4.rs:10:9)
  &lt;lopdf::encryption::crypt_filters::Rc4CryptFilter as lopdf::encryption::crypt_filters::CryptFilter&gt;::decrypt (lopdf/src/encryption/crypt_filters.rs:78:12)
  repro_lopdf_src_encryption_rc4_rs_10_9_aaeeca::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::encode_row(FilterType::Up, 0usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:130:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::encode_row (lopdf/src/filters/png.rs:130:54)
  repro_lopdf_src_filters_png_rs_130_54_e39348::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::decode_row(FilterType::Paeth, 1usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:78:71:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::decode_row (lopdf/src/filters/png.rs:78:71)
  repro_lopdf_src_filters_png_rs_78_71_9f7eb9::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::decode_row(FilterType::Avg, 1usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:68:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::decode_row (lopdf/src/filters/png.rs:68:54)
  repro_lopdf_src_filters_png_rs_68_54_eca3b7::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
document.renumber_objects_with(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/processor.rs:261:23:
attempt to subtract with overflow
backtrace:
  &lt;lopdf::document::Document&gt;::renumber_objects_with (lopdf/src/processor.rs:261:23)
  repro_lopdf_src_processor_rs_261_23_17328a::main (src/main.rs:5:14)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::encode_row(FilterType::Paeth, 0usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:145:86:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::encode_row (lopdf/src/filters/png.rs:145:86)
  repro_lopdf_src_filters_png_rs_145_86_0242bb::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let xref_entry = XrefEntry::Compressed { container: 0u32, index: 0u16 };
let _ = xref_entry.encode_for_xref_stream(&amp;[10000000000000000000usize,
    2147483648usize, 5830499564928007129usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/xref.rs:174:31:
attempt to multiply with overflow
backtrace:
  lopdf::xref::encode_field (lopdf/src/xref.rs:174:31)
  &lt;lopdf::xref::XrefEntry&gt;::encode_for_xref_stream (lopdf/src/xref.rs:106:17)
  repro_lopdf_src_xref_rs_174_31_2a7e71::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::decode_row(FilterType::Avg, 0usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:73:87:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::decode_row (lopdf/src/filters/png.rs:73:87)
  repro_lopdf_src_filters_png_rs_73_87_9b1517::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::encode_row(FilterType::Avg, 0usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:136:87:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::encode_row (lopdf/src/filters/png.rs:136:87)
  repro_lopdf_src_filters_png_rs_136_87_07e947::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let xref_entry = XrefEntry::Free;
let _ = xref_entry.encode_for_xref_stream(&amp;[100usize, 0usize, 0usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/xref.rs:174:21:
attempt to shift right with overflow
backtrace:
  lopdf::xref::encode_field (lopdf/src/xref.rs:174:21)
  &lt;lopdf::xref::XrefEntry&gt;::encode_for_xref_stream (lopdf/src/xref.rs:94:17)
  repro_lopdf_src_xref_rs_174_21_443cc9::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>lopdf::filters::png::decode_row(FilterType::Up, 0usize, &amp;vec![],
    &amp;mut vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/filters/png.rs:63:54:
index out of bounds: the len is 0 but the index is 0
backtrace:
  lopdf::filters::png::decode_row (lopdf/src/filters/png.rs:63:54)
  repro_lopdf_src_filters_png_rs_63_54_2fdb93::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_from_prev(&amp;Document::new());
document.renumber_objects_with(2453330354u32);
let _ = document.save_to::&lt;Vec&lt;u8&gt;&gt;(&amp;mut include_bytes!(
    "image_0.tiff").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  lopdf::xref::Xref::get (src/xref.rs:50)
  lopdf::writer::Writer::xref_sections (src/writer.rs:437)
  lopdf::writer::Writer::create_xref_steam (src/writer.rs:494)
  lopdf::document::Document::write_cross_reference_stream&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (lopdf/src/writer.rs:243)
  lopdf::document::Document::write_xref_and_trailer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (lopdf/src/writer.rs:106)
  lopdf::document::Document::save_internal&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (lopdf/src/writer.rs:90)
  lopdf::document::Document::save_to&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (lopdf/src/writer.rs:23)
  repro_timeout_in_lopdf_document_save_to_ffa94a::main (src/main.rs:6)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new_from_prev(&amp;Document::default());
let _ = document.add_bookmark(Bookmark { children: vec![0u32],
    title: "".to_string(), format: 0u32, color: [0.0f32, 0.0f32, 0.0f32],
    page: (0u32, 1u16), id: 0u32 }, None);
let _ = document.build_outline();</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/bookmarks.rs:61:55:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;lopdf::document::Document&gt;::outline_child (lopdf/src/bookmarks.rs:61:55)
  &lt;lopdf::document::Document&gt;::outline_child (lopdf/src/bookmarks.rs:104:55)
  &lt;lopdf::document::Document&gt;::build_outline (lopdf/src/bookmarks.rs:133:45)
  repro_lopdf_src_bookmarks_rs_61_55_02c39e::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::default();
let _ = document.add_object::&lt;lopdf::Object&gt;(Object::from(vec![]));
document.renumber_objects_with(u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/processor.rs:230:13:
attempt to add with overflow
backtrace:
  &lt;lopdf::document::Document&gt;::renumber_objects_with (lopdf/src/processor.rs:230:13)
  repro_lopdf_src_processor_rs_230_13_dc51a7::main (src/main.rs:7:14)</pre></td></tr>
<tr><td valign="top"><pre>let mut document = Document::new();
document.renumber_objects_with(u32::MAX);
let mut incremental_document = IncrementalDocument::create_from(vec![],
    document.clone());
let _ = incremental_document.save_to::&lt;Vec&lt;u8&gt;&gt;(&amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at lopdf/src/writer.rs:237:44:
attempt to add with overflow
backtrace:
  &lt;lopdf::document::Document&gt;::write_cross_reference_stream::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (lopdf/src/writer.rs:237:44)
  &lt;lopdf::document::Document&gt;::write_xref_and_trailer::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (lopdf/src/writer.rs:106:22)
  &lt;lopdf::incremental_document::IncrementalDocument&gt;::save_internal::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (lopdf/src/writer.rs:391:14)
  &lt;lopdf::incremental_document::IncrementalDocument&gt;::save_to::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (lopdf/src/writer.rs:293:14)
  repro_lopdf_src_writer_rs_237_44_61be4c::main (src/main.rs:8:34)</pre></td></tr>
</table>
