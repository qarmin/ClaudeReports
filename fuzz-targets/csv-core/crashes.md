# csv-core - crashes

Source: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let reader_builder = ReaderBuilder::new();
let mut reader = reader_builder.build();
reader.set_line(u64::MAX);
let _ = reader.read_record(&amp;vec![10u8], &amp;mut vec![0u8], &amp;mut vec![0usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at csv-core/src/reader.rs:665:13:
attempt to add with overflow
backtrace:
  &lt;csv_core::reader::Reader&gt;::read_record_dfa (csv-core/src/reader.rs:665:13)
  &lt;csv_core::reader::Reader&gt;::read_record (csv-core/src/reader.rs:601:18)
  repro_csv_core_src_reader_rs_665_13_d95f79::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let writer_builder = WriterBuilder::default();
let mut writer = writer_builder.build();
let _ = writer.field(&amp;vec![10u8], &amp;mut vec![]);
let _ = writer.terminator(&amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at csv-core/src/writer.rs:372:13:
assertion failed: !self.state.quoting
backtrace:
  &lt;csv_core::writer::Writer&gt;::terminator (csv-core/src/writer.rs:372:13)
  repro_csv_core_src_writer_rs_372_13_d9b4f2::main (src/main.rs:7:20)</pre></td></tr>
<tr><td valign="top"><pre>let mut reader = Reader::new();
reader.set_line(u64::MAX);
let _ = reader.read_field(&amp;vec![10u8], &amp;mut vec![73u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at csv-core/src/reader.rs:724:13:
attempt to add with overflow
backtrace:
  &lt;csv_core::reader::Reader&gt;::read_field_dfa (csv-core/src/reader.rs:724:13)
  &lt;csv_core::reader::Reader&gt;::read_field (csv-core/src/reader.rs:544:18)
  repro_csv_core_src_reader_rs_724_13_a2634e::main (src/main.rs:6:20)</pre></td></tr>
</table>
