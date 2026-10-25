# object - crashes

Source: https://github.com/gimli-rs/object @ 4ef27e429dd86949ac3d3a995b3266440bedcf5c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
attributes_writer.write_attribute_integer(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1436:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_integer (object/src/write/elf/encoder.rs:1436:9)
  repro_object_src_write_elf_encoder_rs_1436_9_84e7d0::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = N_ALT_ENTRY.with_common_alignment(68u8);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/macho.rs:3537:9:
assertion `left == right` failed
  left: 64
 right: 0
backtrace:
  &lt;object::macho::SymbolDesc&gt;::with_common_alignment (object/src/macho.rs:3537:9)
  repro_object_src_macho_rs_3537_9_e1b41a::main (src/main.rs:4:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::default());
attributes_writer.end_subsubsection();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1462:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::end_subsubsection (object/src/write/elf/encoder.rs:1462:9)
  repro_object_src_write_elf_encoder_rs_1462_9_0205ad::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(
    &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap(), false).endian());
attributes_writer.write_subsubsection_indices(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1421:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_indices (object/src/write/elf/encoder.rs:1421:9)
  repro_object_src_write_elf_encoder_rs_1421_9_ad5190::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::default());
attributes_writer.write_subsubsection_index(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1412:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_index (object/src/write/elf/encoder.rs:1412:9)
  repro_object_src_write_elf_encoder_rs_1412_9_40bf4e::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.write_attribute_tag(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1429:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_tag (object/src/write/elf/encoder.rs:1429:9)
  repro_object_src_write_elf_encoder_rs_1429_9_bfdcad::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.write_subsubsection_attributes(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1453:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_attributes (object/src/write/elf/encoder.rs:1453:9)
  repro_object_src_write_elf_encoder_rs_1453_9_56cab5::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let encoder = &lt;object::write::elf::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap(), false).endian(), false, EM_COREA_2ND);
let _ = encoder.gnu_verneed_size(0u16, 10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1295:15:
attempt to multiply with overflow
backtrace:
  &lt;object::write::elf::encoder::Encoder&lt;object::endian::Endianness&gt;&gt;::gnu_verneed_size (object/src/write/elf/encoder.rs:1295:15)
  repro_object_src_write_elf_encoder_rs_1295_15_73f02f::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let encoder = &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    Endianness::Big, false);
let _ = encoder.load_command_size(u64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/encoder.rs:387:9:
attempt to add with overflow
backtrace:
  &lt;object::write::macho::encoder::Encoder&lt;object::endian::Endianness&gt;&gt;::load_command_size (object/src/write/macho/encoder.rs:387:9)
  repro_object_src_write_macho_encoder_rs_387_9_fc77d8::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let code_signature_encoder = CodeSignatureEncoder;
code_signature_encoder.code_directory::&lt;Vec&lt;u8&gt;&gt;(&amp;mut vec![],
    &amp;CodeDirectory { length: 0u32, version: CsVersion(1000000000u32),
    flags: CS_EXEC_SET_KILL, hash_offset: 0u32, ident_offset: 0u32,
    n_special_slots: 0u32, n_code_slots: 0u32, code_limit: 0u64, hash_size: 0u8,
    hash_type: CS_HASHTYPE_SHA384, platform: 0u8, page_size: 0u8,
    scatter_offset: 0u32, team_offset: 0u32, exec_seg_base: 0u64,
    exec_seg_limit: 0u64, exec_seg_flags: CS_EXECSEG_MAIN_BINARY });</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/encoder.rs:951:9:
assertion failed: cd.version &lt;= macho::CS_SUPPORTSEXECSEG
backtrace:
  &lt;object::write::macho::encoder::CodeSignatureEncoder&gt;::code_directory::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (object/src/write/macho/encoder.rs:951:9)
  repro_object_src_write_macho_encoder_rs_951_9_d012d1::main (src/main.rs:10:28)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::default());
attributes_writer.write_attribute_string(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1445:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_string (object/src/write/elf/encoder.rs:1445:9)
  repro_object_src_write_elf_encoder_rs_1445_9_86667c::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut string_table = StringTable::new_in_order(0u32);
let _ = string_table.write::&lt;Vec&lt;u8&gt;&gt;(&amp;mut vec![], 1u32);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/string.rs:129:13:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;object::write::string::StringTable&gt;::write::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (object/src/write/string.rs:129:13)
  repro_object_src_write_string_rs_129_13_d308ac::main (src/main.rs:5:26)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::default());
attributes_writer.end_subsection();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1391:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::end_subsection (object/src/write/elf/encoder.rs:1391:9)
  repro_object_src_write_elf_encoder_rs_1391_9_62d3fa::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let builder = Builder::new(Endianness::Little, false);
let encoder = builder.encoder();
let _ = encoder.gnu_verdef_size(0u16, 10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1227:15:
attempt to multiply with overflow
backtrace:
  &lt;object::write::elf::encoder::Encoder&lt;object::endian::Endianness&gt;&gt;::gnu_verdef_size (object/src/write/elf/encoder.rs:1227:15)
  repro_object_src_write_elf_encoder_rs_1227_15_ace09e::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Pe, Architecture::PowerPc,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
object.add_coff_exports(CoffExportStyle::Gnu);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/coff/object.rs:406:9:
assertion `left == right` failed
  left: Pe
 right: Coff
backtrace:
  &lt;object::write::Object&gt;::add_coff_exports (object/src/write/coff/object.rs:406:9)
  repro_object_src_write_coff_object_rs_406_9_d1583d::main (src/main.rs:8:12)</pre></td></tr>
<tr><td valign="top"><pre>let code_signature_encoder = CodeSignatureEncoder;
let _ = code_signature_encoder.code_directory_size(
    &lt;object::macho::CsVersion as object::Wrap&gt;::from_inner(1000000000u32));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/encoder.rs:929:9:
assertion failed: version &lt;= macho::CS_SUPPORTSEXECSEG
backtrace:
  &lt;object::write::macho::encoder::CodeSignatureEncoder&gt;::code_directory_size (object/src/write/macho/encoder.rs:929:9)
  repro_object_src_write_macho_encoder_rs_929_9_d8b45e::main (src/main.rs:5:36)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.start_subsubsection(AttributeTag::default());</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1401:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::start_subsubsection (object/src/write/elf/encoder.rs:1401:9)
  repro_object_src_write_elf_encoder_rs_1401_9_39e710::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::MachO, Architecture::Unknown,
    Endianness::default());
let _ = object.write();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/object.rs:451:60:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;object::write::Object&gt;::macho_write (object/src/write/macho/object.rs:451:60)
  &lt;object::write::Object&gt;::emit (object/src/write/mod.rs:829:41)
  &lt;object::write::Object&gt;::write (object/src/write/mod.rs:800:14)
  repro_object_src_write_macho_object_rs_451_60_682a1a::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let binary_format = BinaryFormat::native_object();
let mut object = Object::new(binary_format, Architecture::Unknown,
    Endianness::Big);
object.add_elf_gnu_property_u32(
    &lt;object::elf::GnuPropertyType as object::Wrap&gt;::from_inner(0u32), 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/object.rs:605:48:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;object::write::Object&gt;::elf_is_64 (object/src/write/elf/object.rs:605:48)
  &lt;object::write::Object&gt;::add_elf_gnu_property_u32 (object/src/write/elf/object.rs:41:36)
  repro_object_src_write_elf_object_rs_605_48_82ef5e::main (src/main.rs:9:12)</pre></td></tr>
<tr><td valign="top"><pre>let encoder = &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    Endianness::Big, false);
let _ = encoder.dylib_command_size(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/encoder.rs:664:13:
attempt to add with overflow
backtrace:
  &lt;object::write::macho::encoder::Encoder&lt;object::endian::Endianness&gt;&gt;::dylib_command_size (object/src/write/macho/encoder.rs:664:13)
  repro_object_src_write_macho_encoder_rs_664_13_e60b1f::main (src/main.rs:5:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_attribute_tag(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1430:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_tag (object/src/write/elf/encoder.rs:1430:9)
  repro_object_src_write_elf_encoder_rs_1430_9_486eb8::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let code_signature_encoder = CodeSignatureEncoder;
code_signature_encoder.code_directory::&lt;Vec&lt;u8&gt;&gt;(&amp;mut vec![],
    &amp;CodeDirectory { length: 0u32, version: CS_SUPPORTSSCATTER,
    flags: CS_RESTRICT, hash_offset: 0u32, ident_offset: 0u32,
    n_special_slots: 0u32, n_code_slots: 0u32, code_limit: u64::MAX,
    hash_size: 255u8,
    hash_type: &lt;object::macho::CsHashType as object::Wrap&gt;::from_inner(72u8),
    platform: 1u8, page_size: 191u8, scatter_offset: 4077522095u32,
    team_offset: 32u32, exec_seg_base: 0u64, exec_seg_limit: 4294967295u64,
    exec_seg_flags: CS_EXECSEG_SKIP_LV });</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/encoder.rs:953:13:
assertion failed: cd.version &gt;= macho::CS_SUPPORTSCODELIMIT64
backtrace:
  &lt;object::write::macho::encoder::CodeSignatureEncoder&gt;::code_directory::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (object/src/write/macho/encoder.rs:953:13)
  repro_object_src_write_macho_encoder_rs_953_13_573f08::main (src/main.rs:9:28)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_subsubsection_index(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1413:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_index (object/src/write/elf/encoder.rs:1413:9)
  repro_object_src_write_elf_encoder_rs_1413_9_f0e63b::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = object::macho::cpu_subtype_intel(16u32, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/macho.rs:263:5:
attempt to add with overflow
backtrace:
  object::macho::cpu_subtype_intel (object/src/macho.rs:263:5)
  repro_object_src_macho_rs_263_5_c033b9::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.end_subsubsection();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1463:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::end_subsubsection (object/src/write/elf/encoder.rs:1463:9)
  repro_object_src_write_elf_encoder_rs_1463_9_839f29::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Little);
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.start_subsection(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1379:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::start_subsection (object/src/write/elf/encoder.rs:1379:9)
  repro_object_src_write_elf_encoder_rs_1379_9_d3516c::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::Xcoff, Architecture::Unknown,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
let _ = object.write();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/xcoff.rs:217:60:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;object::write::Object&gt;::xcoff_write (object/src/write/xcoff.rs:217:60)
  &lt;object::write::Object&gt;::emit (object/src/write/mod.rs:831:41)
  &lt;object::write::Object&gt;::write (object/src/write/mod.rs:800:14)
  repro_object_src_write_xcoff_rs_217_60_f38d9b::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Big);
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_subsubsection_attributes(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1454:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_attributes (object/src/write/elf/encoder.rs:1454:9)
  repro_object_src_write_elf_encoder_rs_1454_9_8496b3::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::default());
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_attribute_integer(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1437:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_integer (object/src/write/elf/encoder.rs:1437:9)
  repro_object_src_write_elf_encoder_rs_1437_9_65e204::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(
    &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    Endianness::default(), false).endian());
attributes_writer.start_subsection(&amp;vec![]);
let _ = attributes_writer.data();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1472:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::data (object/src/write/elf/encoder.rs:1472:9)
  repro_object_src_write_elf_encoder_rs_1472_9_676e6c::main (src/main.rs:7:31)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Coff, Architecture::M68k,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
let _ = object.add_subsection(StandardSection::TlsVariables, &amp;vec![80u8, 78u8,
    71u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
let _ = object.write();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/string.rs:60:9:
assertion failed: !string.contains(&amp;0)
backtrace:
  &lt;object::write::string::StringTable&gt;::add (object/src/write/string.rs:60:9)
  &lt;object::write::coff::writer::Writer&gt;::add_string (object/src/write/coff/writer.rs:430:21)
  &lt;object::write::coff::writer::Writer&gt;::add_name (object/src/write/coff/writer.rs:438:29)
  &lt;object::write::Object&gt;::coff_write (object/src/write/coff/object.rs:435:50)
  &lt;object::write::Object&gt;::emit (object/src/write/mod.rs:825:40)
  &lt;object::write::Object&gt;::write (object/src/write/mod.rs:800:14)
  repro_object_src_write_string_rs_60_9_2dad08::main (src/main.rs:9:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_subsubsection_indices(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1422:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_subsubsection_indices (object/src/write/elf/encoder.rs:1422:9)
  repro_object_src_write_elf_encoder_rs_1422_9_ffb5cc::main (src/main.rs:6:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut attributes_writer = AttributesWriter::new(Endianness::Little);
attributes_writer.start_subsection(&amp;vec![]);
attributes_writer.write_attribute_string(&amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/elf/encoder.rs:1446:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;object::write::elf::encoder::AttributesWriter&gt;::write_attribute_string (object/src/write/elf/encoder.rs:1446:9)
  repro_object_src_write_elf_encoder_rs_1446_9_b1eaa2::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::MachO, Architecture::SuperH,
    Endianness::Little);
let mut object2 = Object::new(object.format(), Architecture::Xtensa,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
let _ = object2.add_relocation(Object::new(BinaryFormat::Elf,
    Architecture::SuperH,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel),
    Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Pe,
    Architecture::Msp430, Endianness::default()).add_file_symbol(vec![]),
    addend: 1i64, flags: RelocationFlags::MachO { r_type: PPC_RELOC_HI16,
    r_pcrel: false, r_length: 10u8 } });</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/macho/object.rs:435:16:
attempt to shift left with overflow
backtrace:
  &lt;object::write::Object&gt;::macho_relocation_size (object/src/write/macho/object.rs:435:16)
  &lt;object::write::Object&gt;::write_relocation_addend (object/src/write/mod.rs:764:41)
  &lt;object::write::Object&gt;::add_relocation_internal (object/src/write/mod.rs:746:18)
  &lt;object::write::Object&gt;::add_relocation (object/src/write/mod.rs:684:14)
  repro_object_src_write_macho_object_rs_435_16_383d6f::main (src/main.rs:13:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Coff, Architecture::Mips64,
    &lt;object::write::elf::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    Endianness::Big, false, Machine::default()).endian());
let _ = object.append_section_bss(Object::new(BinaryFormat::native_object(),
    Architecture::Aarch64_Ilp32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::TlsVariables), 0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:216:22:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::append_section_bss (object/src/write/mod.rs:216:22)
  repro_object_src_write_mod_rs_216_22_c5d509::main (src/main.rs:10:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Object::new(BinaryFormat::Xcoff, Architecture::Wasm32,
    Endianness::Little).add_subsection(StandardSection::UninitializedData,
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:340:18:
not implemented
backtrace:
  &lt;object::write::Object&gt;::subsection_name (object/src/write/mod.rs:340:18)
  &lt;object::write::Object&gt;::subsection_info (object/src/write/mod.rs:328:25)
  &lt;object::write::Object&gt;::add_subsection (object/src/write/mod.rs:298:53)
  repro_object_src_write_mod_rs_340_18_bfa265::main (src/main.rs:8:88)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Pe, Architecture::Aarch64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
let _ = object.append_section_data(Object::new(BinaryFormat::Xcoff,
    Architecture::X86_64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap()).section_id(StandardSection::ReadOnlyData), &amp;vec![], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:208:22:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::append_section_data (object/src/write/mod.rs:208:22)
  repro_object_src_write_mod_rs_208_22_99b630::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::Pe, Architecture::Ia64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
let _ = object.segment_name(StandardSegment::Debug);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:176:18:
not implemented
backtrace:
  &lt;object::write::Object&gt;::segment_name (object/src/write/mod.rs:176:18)
  repro_object_src_write_mod_rs_176_18_ddd544::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::Pe, Architecture::LoongArch32,
    Endianness::Big);
let _ = object.write();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:848:18:
not implemented
backtrace:
  &lt;object::write::Object&gt;::emit (object/src/write/mod.rs:848:18)
  &lt;object::write::Object&gt;::write (object/src/write/mod.rs:814:14)
  repro_object_src_write_mod_rs_848_18_e20ab3::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Object::new(BinaryFormat::Wasm, Architecture::Wasm32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:285:18:
not implemented
backtrace:
  &lt;object::write::Object&gt;::section_info (object/src/write/mod.rs:285:18)
  &lt;object::write::Object&gt;::section_id::{closure#0} (object/src/write/mod.rs:227:57)
  &lt;object::write::Object&gt;::section_id (object/src/write/mod.rs:226:14)
  repro_object_src_write_mod_rs_285_18_a365a6::main (src/main.rs:7:159)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm,
    Endianness::Big);
object.set_symbol_data(Object::new(BinaryFormat::Coff, Architecture::Xtensa,
    Endianness::Little).add_file_symbol(vec![]),
    Object::new(BinaryFormat::MachO, Architecture::Sparc32Plus,
    Endianness::default()).section_id(StandardSection::Data), 0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:423:22:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Symbol]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[object::write::Symbol] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;object::write::Object&gt;::symbol (object/src/write/mod.rs:423:22)
  &lt;object::write::Object&gt;::set_symbol_data (object/src/write/mod.rs:663:28)
  repro_object_src_write_mod_rs_423_22_425247::main (src/main.rs:9:12)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Pe, Architecture::M68k,
    &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap(), false).endian());
let _ = object.section_symbol(Object::new(BinaryFormat::native_object(),
    Architecture::LoongArch64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::UninitializedTls));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:567:41:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::section_symbol (object/src/write/mod.rs:567:41)
  repro_object_src_write_mod_rs_567_41_f8402b::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;object::elf::VersionIndex as ::core::ops::Sub&lt;object::elf::
    VersionIndex&gt;&gt;::sub(VersionIndex(0u16),
    VersionIndex::default().offset(1u16));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/elf.rs:2553:9:
attempt to subtract with overflow
backtrace:
  &lt;object::elf::VersionIndex&gt;::offset_from (object/src/elf.rs:2553:9)
  &lt;object::elf::VersionIndex as core::ops::arith::Sub&gt;::sub (object/src/elf.rs:2600:14)
  repro_object_src_elf_rs_2553_9_3fe0fb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::MachO, Architecture::LoongArch32,
    Endianness::Little);
let _ = object.section(Object::new(BinaryFormat::native_object(),
    Architecture::Aarch64_Ilp32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::UninitializedTls));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:183:23:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[object::write::Section] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;object::write::Object&gt;::section (object/src/write/mod.rs:183:23)
  repro_object_src_write_mod_rs_183_23_8c266d::main (src/main.rs:9:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Coff, Architecture::E2K32,
    Endianness::Little);
let _ = object.symbol_mut(Object::new(BinaryFormat::Xcoff,
    Architecture::Aarch64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).add_file_symbol(vec![]));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:429:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Symbol]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Symbol] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::symbol_mut (object/src/write/mod.rs:429:26)
  repro_object_src_write_mod_rs_429_26_d993cb::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Elf, Architecture::Unknown,
    Endianness::Little);
let _ = object.section_mut(Object::new(BinaryFormat::Coff,
    Architecture::Unknown,
    Endianness::Little).section_id(StandardSection::Text));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:189:27:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::section_mut (object/src/write/mod.rs:189:27)
  repro_object_src_write_mod_rs_189_27_a59ad5::main (src/main.rs:9:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Sparc,
    Endianness::Little);
let _ = object.add_relocation_with_subtractor(Object::new(BinaryFormat::Elf,
    Architecture::E2K32,
    Endianness::default()).section_id(StandardSection::Data),
    Relocation { offset: 0u64,
    symbol: Object::new(BinaryFormat::native_object(), Architecture::PowerPc64,
    Endianness::Little).add_file_symbol(vec![]), addend: 0i64,
    flags: RelocationFlags::Generic { kind: RelocationKind::Relative,
    encoding: RelocationEncoding::E2KDisp, size: 1u8 } }, None);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:783:33:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::write_relocation_addend (object/src/write/mod.rs:783:33)
  &lt;object::write::Object&gt;::add_relocation_internal (object/src/write/mod.rs:760:18)
  &lt;object::write::Object&gt;::add_relocation_with_subtractor (object/src/write/mod.rs:719:14)
  repro_object_src_write_mod_rs_783_33_dc3a07::main (src/main.rs:13:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Coff, Architecture::Alpha,
    Endianness::Little);
let section_id = object.section_id(StandardSection::ReadOnlyData);
let _ = object.add_symbol_data(Object::new(BinaryFormat::Wasm,
    Architecture::Sharc, Endianness::Big).add_file_symbol(vec![]), section_id,
    &amp;vec![], 10u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:980:9:
assertion `left == right` failed
  left: 8
 right: 0
backtrace:
  &lt;object::write::Section&gt;::append_data (object/src/write/mod.rs:980:9)
  &lt;object::write::Object&gt;::append_section_data (object/src/write/mod.rs:208:34)
  &lt;object::write::Object&gt;::add_symbol_data (object/src/write/mod.rs:617:27)
  repro_object_src_write_mod_rs_980_9_2b32af::main (src/main.rs:10:20)</pre></td></tr>
<tr><td valign="top"><pre>let object = Object::new(BinaryFormat::Elf, Architecture::Wasm32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
let mut object2 = Object::new(object.format(), Architecture::E2K32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
let _ = object2.add_relocation(Object::new(BinaryFormat::native_object(),
    Architecture::Arm,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).section_id(StandardSection::GnuProperty),
    Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Wasm,
    Architecture::LoongArch32, Endianness::Big).add_file_symbol(vec![]),
    addend: 0i64,
    flags: RelocationFlags::Elf { r_type: R_TILEGX_DEST_IMM8_X1 } });</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:763:22:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Section]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Section] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::add_relocation_internal (object/src/write/mod.rs:763:22)
  &lt;object::write::Object&gt;::add_relocation (object/src/write/mod.rs:694:14)
  repro_object_src_write_mod_rs_763_22_7f8375::main (src/main.rs:13:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::native_object(),
    Architecture::Xtensa,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap());
let section_id = object.section_id(StandardSection::UninitializedData);
let section = object.section(section_id);
let _ = section.data();</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:1019:9:
assertion failed: !self.is_bss()
backtrace:
  &lt;object::write::Section&gt;::data (object/src/write/mod.rs:1019:9)
  repro_object_src_write_mod_rs_1019_9_1709d2::main (src/main.rs:10:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Wasm, Architecture::LoongArch64,
    Endianness::default());
let _ = object.add_relocation(Object::new(BinaryFormat::Coff,
    Architecture::LoongArch32,
    Endianness::Big).section_id(StandardSection::EhFrame),
    Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::MachO,
    Architecture::Riscv32,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).add_file_symbol(vec![]), addend: 0i64,
    flags: RelocationFlags::Wasm { r_type: 0u8 } });</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:744:18:
not implemented
backtrace:
  &lt;object::write::Object&gt;::add_relocation_internal (object/src/write/mod.rs:744:18)
  &lt;object::write::Object&gt;::add_relocation (object/src/write/mod.rs:694:14)
  repro_object_src_write_mod_rs_744_18_7dbd88::main (src/main.rs:11:20)</pre></td></tr>
<tr><td valign="top"><pre>let comdat = Comdat { kind: ComdatKind::Unknown,
    symbol: Object::new(BinaryFormat::Wasm, Architecture::Bpf,
    Endianness::Big).add_file_symbol(vec![]), sections: vec![] };
let mut object = Object::new(BinaryFormat::Xcoff, Architecture::S390x,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_big_endian(
    false).unwrap());
let _ = object.comdat_mut(Object::new(BinaryFormat::MachO, Architecture::X86_64,
    &lt;object::endian::Endianness as object::endian::Endian&gt;::from_little_endian(
    false).unwrap()).add_comdat(comdat));</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:405:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[object::write::Comdat]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[object::write::Comdat] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;object::write::Object&gt;::comdat_mut (object/src/write/mod.rs:405:26)
  repro_object_src_write_mod_rs_405_26_30509b::main (src/main.rs:11:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::native_object(),
    Architecture::X86_64_X32, Endianness::Big);
let section_id = object.section_id(StandardSection::ReadOnlyString);
let _ = object.append_section_bss(section_id, 0u64, 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:1001:9:
assertion failed: self.is_bss()
backtrace:
  &lt;object::write::Section&gt;::append_bss (object/src/write/mod.rs:1001:9)
  &lt;object::write::Object&gt;::append_section_bss (object/src/write/mod.rs:216:34)
  repro_object_src_write_mod_rs_1001_9_059c13::main (src/main.rs:10:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Hexagon,
    &lt;object::write::macho::Encoder&lt;object::endian::Endianness&gt;&gt;::new(
    Endianness::default(), false).endian());
let section_id = object.section_id(StandardSection::UninitializedTls);
let _ = object.append_section_data(section_id, &amp;vec![], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at object/src/write/mod.rs:979:9:
assertion failed: !self.is_bss()
backtrace:
  &lt;object::write::Section&gt;::append_data (object/src/write/mod.rs:979:9)
  &lt;object::write::Object&gt;::append_section_data (object/src/write/mod.rs:208:34)
  repro_object_src_write_mod_rs_979_9_7a0c2c::main (src/main.rs:10:20)</pre></td></tr>
</table>
