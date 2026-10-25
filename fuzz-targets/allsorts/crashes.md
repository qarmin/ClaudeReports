# allsorts - crashes

Source: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = allsorts::checksum::table_checksum(&amp;vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/checksum.rs:15:5:
assertion `left == right` failed: data end is not 32-bit aligned
  left: 1
 right: 0
backtrace:
  allsorts::checksum::table_checksum (allsorts/src/checksum.rs:15:5)
  repro_allsorts_src_checksum_rs_15_5_f78030::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;allsorts::tables::F2Dot14 as ::core::ops::Neg&gt;::neg(F2Dot14::from(
    2i16));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables.rs:1317:17:
attempt to negate with overflow
backtrace:
  &lt;allsorts::tables::F2Dot14 as core::ops::arith::Neg&gt;::neg (allsorts/src/tables.rs:1317:17)
  repro_allsorts_src_tables_rs_1317_17_0e7d1e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let coverage = Coverage::Format2 { coverage_range_array: vec![&lt;allsorts::
    layout::CoverageRangeRecord as allsorts::binary::read::ReadFrom&gt;::read_from(
    (1u16, 0u16, 0u16))] };
let _ = coverage.glyph_count();</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/layout.rs:3387:21:
attempt to subtract with overflow
backtrace:
  &lt;allsorts::layout::Coverage&gt;::glyph_count::{closure#0} (allsorts/src/layout.rs:3387:21)
  &lt;allsorts::layout::Coverage&gt;::glyph_count (allsorts/src/layout.rs:3386:18)
  repro_allsorts_src_layout_rs_3387_21_0ace8b::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = allsorts::binary::word_align(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary.rs:35:5:
attempt to multiply with overflow
backtrace:
  allsorts::binary::word_align (allsorts/src/binary.rs:35:5)
  repro_allsorts_src_binary_rs_35_5_2ee6ee::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = allsorts::binary::long_align(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary.rs:20:5:
attempt to multiply with overflow
backtrace:
  allsorts::binary::long_align (allsorts/src/binary.rs:20:5)
  repro_allsorts_src_binary_rs_20_5_f65f6f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Operand::from(f32::NAN);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/cff.rs:2021:26:
internal error: entered unreachable code
backtrace:
  &lt;allsorts::cff::Operand&gt;::bcd_encode (allsorts/src/cff.rs:2021:26)
  &lt;allsorts::cff::Operand as core::convert::From&lt;f32&gt;&gt;::from (allsorts/src/cff.rs:2069:9)
  repro_allsorts_src_cff_rs_2021_26_5749f6::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut write_buffer = WriteBuffer::new();
let _ = &lt;allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::
    write::WriteBinary&gt;::write::&lt;allsorts::binary::write::WriteBuffer&gt;(
    &amp;mut write_buffer,
    CmapSubtable::Format4(CmapSubtableFormat4 { language: 0u16,
    end_codes: vec![], start_codes: vec![], id_deltas: vec![],
    id_range_offsets: vec![], glyph_id_array: vec![] }));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/cmap.rs:398:9:
attempt to subtract with overflow
backtrace:
  &lt;allsorts::tables::cmap::Format4Calculator&gt;::range_shift (allsorts/src/tables/cmap.rs:398:9)
  &lt;allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary&gt;::write::&lt;allsorts::binary::write::WriteBuffer&gt; (allsorts/src/tables/cmap.rs:1138:45)
  repro_allsorts_src_tables_cmap_rs_398_9_9e2577::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = F2Dot14::from(
    &lt;allsorts::tables::Fixed as ::core::ops::Div&lt;allsorts::tables::Fixed&gt;&gt;::div(
    Fixed::from(0.0f32), Fixed::from(0.0f32)));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables.rs:1332:18:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::F2Dot14 as core::convert::From&lt;allsorts::tables::Fixed&gt;&gt;::from (allsorts/src/tables.rs:1332:18)
  repro_allsorts_src_tables_rs_1332_18_78675a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = allsorts::scripts::indic::indic2_tag(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/scripts/indic.rs:1270:14:
Expected an Indic1 script tag
backtrace:
  allsorts::scripts::indic::indic2_tag (allsorts/src/scripts/indic.rs:1270:14)
  repro_allsorts_src_scripts_indic_rs_1270_14_625094::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let read_scope_owned = ReadScopeOwned::new(ReadScope::new(&amp;vec));
let read_scope = read_scope_owned.scope();
let mut read_ctxt = read_scope.ctxt();
let _ = read_ctxt.read_array::&lt;allsorts::binary::I16Be&gt;(
    10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/read.rs:566:37:
attempt to multiply with overflow
backtrace:
  &lt;allsorts::binary::read::ReadCtxt&gt;::read_array::&lt;allsorts::binary::I16Be&gt; (allsorts/src/binary/read.rs:566:37)
  repro_allsorts_src_binary_read_rs_566_37_ac1fae::main (src/main.rs:9:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;allsorts::tables::glyf::Point as ::core::ops::Add&lt;allsorts::tables::
    glyf::Point&gt;&gt;::add(Point(i16::MAX, 0i16), Point(1i16, 0i16));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/glyf.rs:1371:15:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::glyf::Point as core::ops::arith::Add&gt;::add (allsorts/src/tables/glyf.rs:1371:15)
  repro_allsorts_src_tables_glyf_rs_1371_15_e8b261::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let clip_box = ClipBox { x_min: i16::MIN, y_min: 0i16, x_max: 0i16, y_max: 0i16,
    var_index_base: None };
let _ = clip_box.width();</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/colr.rs:1024:9:
attempt to subtract with overflow
backtrace:
  &lt;allsorts::tables::colr::ClipBox&gt;::width (allsorts/src/tables/colr.rs:1024:9)
  repro_allsorts_src_tables_colr_rs_1024_9_94b30c::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let clip_box = ClipBox { x_min: 0i16, y_min: i16::MAX, x_max: 0i16,
    y_max: -10000i16, var_index_base: None };
let _ = clip_box.height();</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/colr.rs:1029:9:
attempt to subtract with overflow
backtrace:
  &lt;allsorts::tables::colr::ClipBox&gt; (allsorts/src/tables/colr.rs:1029:9)
  repro_allsorts_src_tables_colr_rs_1029_9_98da2f::main (src/main.rs:5:22)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let read_scope = ReadScope::new(&amp;vec);
let mut read_ctxt = read_scope.ctxt();
let _ = read_ctxt.read_array_stride::&lt;allsorts::binary::U8&gt;(9usize,
    10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/read.rs:584:37:
attempt to multiply with overflow
backtrace:
  &lt;allsorts::binary::read::ReadCtxt&gt;::read_array_stride::&lt;allsorts::binary::U8&gt; (allsorts/src/binary/read.rs:584:37)
  repro_allsorts_src_binary_read_rs_584_37_c9c235::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;allsorts::tables::Fixed as ::core::ops::Add&lt;allsorts::tables::Fixed&gt;&gt;::
    add(&lt;allsorts::tables::Fixed as allsorts::binary::read::ReadFrom&gt;::
    read_from(1i32),
    &lt;allsorts::tables::Fixed as ::core::ops::Div&lt;allsorts::tables::Fixed&gt;&gt;::div(
    Fixed::from_raw(0i32), Fixed::from(0i32))).abs();</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/num/mod.rs:429:5:
attempt to negate with overflow
backtrace:
  &lt;i32&gt;::abs (core/src/num/int_macros.rs:3635:17)
  &lt;allsorts::tables::Fixed&gt;::abs (allsorts/src/tables.rs:1140:22)
  repro_allsorts_src_tables_rs_1140_22_7bf1a4::main (src/main.rs:4:288)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let read_scope = ReadScope::new(&amp;vec);
let read_scope_owned = ReadScopeOwned::new(read_scope.offset(1usize));
let read_scope2 = read_scope_owned.scope();
let _ = read_scope2.offset(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/read.rs:271:20:
attempt to add with overflow
backtrace:
  &lt;allsorts::binary::read::ReadScope&gt;::offset (allsorts/src/binary/read.rs:271:20)
  repro_allsorts_src_binary_read_rs_271_20_edac22::main (src/main.rs:9:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;allsorts::tables::glyf::Point as ::core::ops::Add&lt;allsorts::tables::
    glyf::Point&gt;&gt;::add(Point(0i16, i16::MAX), Point(0i16, 1i16));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/glyf.rs:1371:23:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::glyf::Point as core::ops::arith::Add&gt;::add (allsorts/src/tables/glyf.rs:1371:23)
  repro_allsorts_src_tables_glyf_rs_1371_23_4ecc40::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;allsorts::tables::Fixed as ::core::ops::Neg&gt;::neg(Fixed::from_raw(
    i32::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables.rs:1190:15:
attempt to negate with overflow
backtrace:
  &lt;allsorts::tables::Fixed as core::ops::arith::Neg&gt;::neg (allsorts/src/tables.rs:1190:15)
  repro_allsorts_src_tables_rs_1190_15_6bb8e5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = allsorts::font_specimen::specimen("",
    &amp;include_bytes!("data_0.bin").to_vec(), SpecimenOptions::default());</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/font_specimen.rs:185:50:
range start index 99 out of range for slice of length 80
backtrace:
  &lt;[u8] as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  allsorts::font_specimen::specimen (allsorts/src/font_specimen.rs:185:50)
  repro_allsorts_src_font_specimen_rs_185_50_6fc4d2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let u32_v = LOCA;
let cmap_subtable = CmapSubtable::Format12(
    CmapSubtableFormat12 { language: 0u32,
    groups: vec![&lt;allsorts::tables::cmap::SequentialMapGroup as allsorts::
    binary::read::ReadFrom&gt;::read_from((0u32, u32_v, u32::MAX))] });
let _ = cmap_subtable.map_glyph(u32_v);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/cmap.rs:1038:44:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::cmap::owned::CmapSubtable&gt;::map_glyph (allsorts/src/tables/cmap.rs:1038:44)
  repro_allsorts_src_tables_cmap_rs_1038_44_4d9697::main (src/main.rs:8:27)</pre></td></tr>
<tr><td valign="top"><pre>let _ = allsorts::font_specimen::specimen("",
    &amp;include_bytes!("data_0.bin").to_vec(), SpecimenOptions { index: 3u32,
    sample_text: None });</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/woff2.rs:918:22:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;allsorts::woff2::Woff2TableProvider&gt;::table_directory (allsorts/src/woff2.rs:918:22)
  &lt;allsorts::woff2::Woff2TableProvider&gt;::new (allsorts/src/woff2.rs:884:28)
  &lt;allsorts::woff2::Woff2Font&gt;::table_provider (allsorts/src/woff2.rs:199:9)
  &lt;allsorts::font_data::FontData&gt;::table_provider (allsorts/src/font_data.rs:85:37)
  allsorts::font_specimen::specimen (allsorts/src/font_specimen.rs:177:25)
  repro_allsorts_src_woff2_rs_918_22_b817ba::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let single_subst = SingleSubst::Format2 { coverage: std::sync::Arc::new(
    Coverage::Format2 { coverage_range_array: vec![&lt;allsorts::layout::
    CoverageRangeRecord as allsorts::binary::read::ReadFrom&gt;::read_from((0u16,
    1u16, u16::MAX))] }), substitute_glyph_array: vec![] };
let _ = single_subst.apply_glyph(1u16);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/layout.rs:3368:29:
attempt to add with overflow
backtrace:
  &lt;allsorts::layout::Coverage&gt;::glyph_coverage_value (allsorts/src/layout.rs:3368:29)
  &lt;allsorts::layout::SingleSubst&gt;::apply_glyph (allsorts/src/layout.rs:1434:33)
  repro_allsorts_src_layout_rs_3368_29_782fde::main (src/main.rs:6:26)</pre></td></tr>
<tr><td valign="top"><pre>let mut write_counter = WriteCounter::new();
let _ = &lt;allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::
    write::WriteBinary&gt;::write::&lt;allsorts::binary::write::WriteCounter&gt;(
    &amp;mut write_counter,
    CmapSubtable::Format12(CmapSubtableFormat12 { language: 0u32,
    groups: vec![] }));
let _ = &lt;allsorts::binary::write::WriteCounter as allsorts::binary::write::
    WriteContext&gt;::write_zeros(&amp;mut write_counter, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/write.rs:412:9:
attempt to add with overflow
backtrace:
  &lt;allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext&gt;::write_zeros (allsorts/src/binary/write.rs:412:9)
  repro_allsorts_src_binary_write_rs_412_9_de064d::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut write_counter = WriteCounter::new();
let _ = &lt;allsorts::binary::write::WriteCounter as allsorts::binary::write::
    WriteContext&gt;::write_zeros(&amp;mut write_counter, usize::MAX);
let _ = &lt;allsorts::tables::F2Dot14 as allsorts::binary::write::WriteBinary&gt;::
    write::&lt;allsorts::binary::write::WriteCounter&gt;(&amp;mut write_counter,
    F2Dot14::from(0i16));</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/write.rs:407:9:
attempt to add with overflow
backtrace:
  &lt;allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext&gt;::write_bytes (allsorts/src/binary/write.rs:407:9)
  &lt;allsorts::binary::I16Be as allsorts::binary::write::WriteBinary&lt;i16&gt;&gt;::write::&lt;allsorts::binary::write::WriteCounter&gt; (allsorts/src/binary/write.rs:241:14)
  &lt;allsorts::tables::F2Dot14 as allsorts::binary::write::WriteBinary&gt;::write::&lt;allsorts::binary::write::WriteCounter&gt; (allsorts/src/tables.rs:1100:9)
  repro_allsorts_src_binary_write_rs_407_9_c91352::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let read_scope = ReadScope::new(&amp;vec);
let mut read_ctxt = read_scope.ctxt();
let _ = &lt;allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary&gt;::
    read(&amp;mut read_ctxt);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/glyf.rs:645:56:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::glyf::SimpleGlyph as allsorts::binary::read::ReadBinaryDep&gt;::read_dep (allsorts/src/tables/glyf.rs:645:56)
  &lt;allsorts::binary::read::ReadCtxt&gt;::read_dep::&lt;allsorts::tables::glyf::SimpleGlyph&gt; (allsorts/src/binary/read.rs:448:9)
  &lt;allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary&gt;::read (allsorts/src/tables/glyf.rs:540:30)
  repro_allsorts_src_tables_glyf_rs_645_56_110076::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let read_scope = ReadScope::new(&amp;vec);
let read_scope2 = read_scope.offset(1usize);
let _ = read_scope2.offset_length(usize::MAX, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/binary/read.rs:280:28:
attempt to add with overflow
backtrace:
  &lt;allsorts::binary::read::ReadScope&gt;::offset_length (allsorts/src/binary/read.rs:280:28)
  repro_allsorts_src_binary_read_rs_280_28_b923b0::main (src/main.rs:7:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fixed::from(-32768.0f64);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables.rs:1246:25:
attempt to multiply with overflow
backtrace:
  &lt;allsorts::tables::Fixed as core::convert::From&lt;f64&gt;&gt;::from (allsorts/src/tables.rs:1246:25)
  repro_allsorts_src_tables_rs_1246_25_3844cb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut write_buffer = WriteBuffer::new();
let _ = &lt;allsorts::binary::write::WriteBuffer as allsorts::binary::write::
    WriteContext&gt;::write_zeros(&amp;mut write_buffer, 2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  allsorts::binary::write::{impl#9}::write_zeros (src/binary/write.rs:316)
  repro_out_of_memory_in_allsorts_binary_write_writebuffer_write_zer_28ace7::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Fixed::from(-32768.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables.rs:1236:25:
attempt to multiply with overflow
backtrace:
  &lt;allsorts::tables::Fixed as core::convert::From&lt;f32&gt;&gt;::from (allsorts/src/tables.rs:1236:25)
  repro_allsorts_src_tables_rs_1236_25_dc6ac6::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let read_scope = ReadScope::new(&amp;vec);
let mut read_ctxt = read_scope.ctxt();
let _ = &lt;allsorts::woff::WoffHeader as allsorts::binary::read::ReadBinary&gt;::
    read(&amp;mut read_ctxt);
let _ = &lt;allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary&gt;::
    read(&amp;mut read_ctxt);</pre></td><td valign="top"><pre>thread 'main' panicked at allsorts/src/tables/glyf.rs:645:32:
attempt to add with overflow
backtrace:
  &lt;allsorts::tables::glyf::SimpleGlyph as allsorts::binary::read::ReadBinaryDep&gt;::read_dep (allsorts/src/tables/glyf.rs:645:32)
  &lt;allsorts::binary::read::ReadCtxt&gt;::read_dep::&lt;allsorts::tables::glyf::SimpleGlyph&gt; (allsorts/src/binary/read.rs:448:9)
  &lt;allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary&gt;::read (allsorts/src/tables/glyf.rs:540:30)
  repro_allsorts_src_tables_glyf_rs_645_32_79e6eb::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut write_buffer = WriteBuffer::new();
let _ = &lt;allsorts::binary::write::WriteBuffer as allsorts::binary::write::
    WriteContext&gt;::write_zeros(&amp;mut write_buffer, 10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from allsorts/src/binary/write.rs:316:19)
capacity overflow
backtrace:
  &lt;allsorts::binary::write::WriteBuffer as allsorts::binary::write::WriteContext&gt;::write_zeros (allsorts/src/binary/write.rs:316:19)
  repro_allsorts_src_binary_write_rs_316_19_dd059a::main (src/main.rs:5:13)</pre></td></tr>
</table>
