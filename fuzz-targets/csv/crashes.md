# csv - crashes

Source: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ByteRecord::with_capacity(4294967296usize, 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:142)
  repro_out_of_memory_in_csv_byterecord_with_capacity_be702c::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let string_record = StringRecord::with_capacity(1000000000000000000usize,
    0usize);
let _ = string_record.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:142)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_stringrecord_iter_b266c5::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let byte_record = ByteRecord::with_capacity(0usize, 1000000000usize);
let _ = byte_record.iter();</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  repro_out_of_memory_in_csv_byterecord_iter_90d822::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let string_record = StringRecord::from_byte_record(ByteRecord::from(
    StringRecord::with_capacity(2147483648usize, 0usize))).unwrap();
let _ = string_record.as_byte_record();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:142)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_stringrecord_as_byte_record_a595a7::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::from(StringRecord::from_byte_record(
    ByteRecord::with_capacity(0usize, 1000000000usize)).unwrap());</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  repro_out_of_memory_in_csv_stringrecord_from_csv_stringrecord_362b12::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = ByteRecord::from(StringRecord::with_capacity(0usize, 1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_byterecord_from_csv_stringrecord_473d52::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::with_capacity(0usize, 1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_stringrecord_with_capacity_c3e54b::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::from_byte_record(ByteRecord::with_capacity(0usize,
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  repro_out_of_memory_in_csv_stringrecord_from_byte_record_8da08e::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::from_byte_record_lossy(ByteRecord::with_capacity(0usize,
    1000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  repro_out_of_memory_in_csv_stringrecord_from_byte_record_lossy_dcde07::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let byte_record = ByteRecord::from(StringRecord::with_capacity(
    1000000000000000000usize, 0usize));
let _ = byte_record.position();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:142)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_byterecord_position_117f13::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_builder = WriterBuilder::default();
let _ = writer_builder.buffer_capacity(1000000000000000000usize);
let _ = writer_builder.from_writer::&lt;Vec&lt;u8&gt;&gt;(vec![]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::new&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:634)
  csv::writer::WriterBuilder::from_writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:117)
  repro_out_of_memory_in_csv_writerbuilder_from_writer_1e8dd1::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let string_record = StringRecord::with_capacity(0usize, 1000000000usize);
let _ = string_record.position();</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  csv::byte_record::Bounds::with_capacity (rust-csv/src/byte_record.rs:670)
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:143)
  csv::string_record::StringRecord::with_capacity (rust-csv/src/string_record.rs:119)
  repro_out_of_memory_in_csv_stringrecord_position_a1359a::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer = &lt;csv::Writer&lt;Vec&lt;u8&gt;&gt;&gt;::from_writer(vec![]);
let _ = writer.write_byte_record(&amp;ByteRecord::with_capacity(4294967296usize,
    0usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 4294967296 bytes failed
backtrace:
  csv::byte_record::ByteRecord::with_capacity (rust-csv/src/byte_record.rs:142)
  repro_out_of_memory_in_csv_writer_vec_u8_write_byte_record_8b9319::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_builder = WriterBuilder::default();
let _ = writer_builder.buffer_capacity(1000000000000000000usize);
let writer = writer_builder.from_writer::&lt;Vec&lt;u8&gt;&gt;(vec![]);
let _ = writer.get_ref();</pre></td><td valign="top"><pre>out of memory: memory allocation of 1000000000000000000 bytes failed
backtrace:
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::new&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:634)
  csv::writer::WriterBuilder::from_writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:117)
  repro_out_of_memory_in_csv_writer_vec_u8_get_ref_271071::main (src/main.rs:6)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_builder = WriterBuilder::default();
let _ = writer_builder.buffer_capacity(0usize);
let mut writer = writer_builder.from_writer::&lt;Vec&lt;u8&gt;&gt;(vec![]);
let _ = writer.write_byte_record(&amp;ByteRecord::default());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  csv_core::writer::Writer::write (src/writer.rs:490)
  csv_core::writer::Writer::terminator (src/writer.rs:373)
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_terminator&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:1131)
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_record&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, &amp;csv::byte_record::ByteRecord, &amp;[u8]&gt; (rust-csv/src/writer.rs:926)
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_byte_record&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:962)
  repro_timeout_in_csv_writer_vec_u8_write_byte_record_099496::main (src/main.rs:8)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_builder = WriterBuilder::new();
let _ = writer_builder.buffer_capacity(1usize);
let mut writer = writer_builder.from_writer::&lt;Vec&lt;u8&gt;&gt;(include_bytes!(
    "image_0.jpg").to_vec());
let _ = writer.write_field::&lt;Vec&lt;u8&gt;&gt;(include_bytes!("rustfmt.toml").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  memchr::memchr::memchr (memchr/src/memchr.rs:31)
  csv_core::writer::quote (src/writer.rs:550)
  csv_core::writer::Writer::field (src/writer.rs:316)
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_field_impl&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:1063)
  csv::writer::Writer&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;::write_field&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;, alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt; (rust-csv/src/writer.rs:1049)
  repro_timeout_in_csv_writer_vec_u8_write_field_448b04::main (src/main.rs:7)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::with_capacity(0usize, 1243054458072431503usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:24:31)
capacity overflow
backtrace:
  &lt;usize as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:24:31)
  &lt;csv::byte_record::Bounds&gt;::with_capacity (rust-csv/src/byte_record.rs:670:24)
  &lt;csv::byte_record::ByteRecord&gt;::with_capacity (rust-csv/src/byte_record.rs:143:21)
  &lt;csv::string_record::StringRecord&gt;::with_capacity (rust-csv/src/string_record.rs:119:22)
  repro_rust_csv_src_byte_record_rs_670_24_bdf25b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = StringRecord::with_capacity(10000000000000000000usize, 2usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;csv::byte_record::ByteRecord&gt;::with_capacity (rust-csv/src/byte_record.rs:142:21)
  &lt;csv::string_record::StringRecord&gt;::with_capacity (rust-csv/src/string_record.rs:119:22)
  repro_rust_csv_src_byte_record_rs_142_21_c520ee::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut writer_builder = WriterBuilder::new();
let _ = writer_builder.buffer_capacity(10000000000000000000usize);
let _ = writer_builder.from_writer::&lt;Vec&lt;u8&gt;&gt;(include_bytes!(
    "image_0.jpg").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;csv::writer::Writer&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;::new (rust-csv/src/writer.rs:634:32)
  &lt;csv::writer::WriterBuilder&gt;::from_writer::&lt;alloc::vec::Vec&lt;u8&gt;&gt; (rust-csv/src/writer.rs:117:9)
  repro_rust_csv_src_writer_rs_634_32_c303a1::main (src/main.rs:6:28)</pre></td></tr>
</table>
