# gimli - crashes

Source: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let line_string = LineString::LineStringRef(
    LineStringTable::default().add::&lt;Vec&lt;u8&gt;&gt;(vec![]));
let _ = line_string.get(&amp;StringTable::default(), &amp;LineStringTable::default());</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/str.rs:108:1:
assertion `left == right` failed
  left: BaseId(2)
 right: BaseId(0)
backtrace:
  &lt;gimli::write::str::LineStringTable&gt;::get (gimli/src/write/str.rs:65:17)
  &lt;gimli::write::line::LineString&gt;::get (gimli/src/write/line.rs:896:60)
  repro_gimli_src_write_str_rs_108_1_6c3c9d::main (src/main.rs:7:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = AttributeSpecification::new(DW_AT_upc_threads_scaled, DW_FORM_ref8,
    Some(0i64));</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/read/abbrev.rs:495:9:
assertion failed: (form == constants::DW_FORM_implicit_const &amp;&amp; implicit_const_value.is_some())
    ||
    (form != constants::DW_FORM_implicit_const &amp;&amp;
            implicit_const_value.is_none())
backtrace:
  &lt;gimli::read::abbrev::AttributeSpecification&gt;::new (gimli/src/read/abbrev.rs:495:9)
  repro_gimli_src_read_abbrev_rs_495_9_691710::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut expression = Expression::new();
expression.set_target(0usize, 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/op.rs:176:9:
assertion failed: new_target &lt;= self.next_index()
backtrace:
  &lt;gimli::write::op::Expression&gt;::set_target (gimli/src/write/op.rs:176:9)
  repro_gimli_src_write_op_rs_176_9_fd66a6::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let call_frame_instruction = CallFrameInstruction::Restore(RiscV::X28);
let mut frame_description_entry = FrameDescriptionEntry::new(Address::Constant(
    0u64), 0u32);
frame_description_entry.add_instruction(1u32,
    CallFrameInstruction::RestoreState);
frame_description_entry.add_instruction(0u32, call_frame_instruction.clone());</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/cfi.rs:299:9:
assertion failed: self.instructions.last().map(|x| x.0).unwrap_or(0) &lt;= offset
backtrace:
  &lt;gimli::write::cfi::FrameDescriptionEntry&gt;::add_instruction (gimli/src/write/cfi.rs:299:9)
  repro_gimli_src_write_cfi_rs_299_9_d204b2::main (src/main.rs:10:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut expression = Expression::raw(vec![]);
expression.set_target(0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/op.rs:177:9:
assertion `left != right` failed
  left: 0
 right: 0
backtrace:
  &lt;gimli::write::op::Expression&gt;::set_target (gimli/src/write/op.rs:177:9)
  repro_gimli_src_write_op_rs_177_9_b0ed5f::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut expression = Expression::raw(vec![]);
expression.set_target(1usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/op.rs:178:30:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[gimli::write::op::Operation]&gt;&gt;::index_mut (core/src/slice/index.rs:245:14)
  &lt;[gimli::write::op::Operation] as core::ops::index::IndexMut&lt;usize&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;gimli::write::op::Expression&gt;::set_target (gimli/src/write/op.rs:178:30)
  repro_gimli_src_write_op_rs_178_30_9f9080::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut line_program = LineProgram::new(LineProgram::none().encoding(),
    LineEncoding { minimum_instruction_length: 100u8,
    maximum_operations_per_instruction: 1u8, default_is_stmt: false,
    line_base: 0i8, line_range: 1u8 },
    LineString::StringRef(StringTable::default().add::&lt;Vec&lt;u8&gt;&gt;(vec![])), None,
    LineString::StringRef(StringTable::default().add::&lt;Vec&lt;u8&gt;&gt;(vec![])), None);
line_program.end_sequence(1u64);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/line.rs:499:13:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;gimli::write::line::LineProgram&gt;::op_advance (gimli/src/write/line.rs:499:13)
  &lt;gimli::write::line::LineProgram&gt;::end_sequence (gimli/src/write/line.rs:365:31)
  repro_gimli_src_write_line_rs_499_13_2aa801::main (src/main.rs:8:18)</pre></td></tr>
<tr><td valign="top"><pre>let format = Format::Dwarf64;
let line_encoding = LineEncoding { minimum_instruction_length: 0u8,
    maximum_operations_per_instruction: 0u8, default_is_stmt: false,
    line_base: 0i8, line_range: 1u8 };
let mut line_program = LineProgram::new(Encoding { address_size: 0u8,
    format: format, version: 0u16 }, line_encoding,
    LineString::LineStringRef(LineStringTable::default().add::&lt;Vec&lt;u8&gt;&gt;(
    vec![])), None, LineString::new::&lt;Vec&lt;u8&gt;&gt;(vec![],
    Encoding { address_size: 0u8, format: format, version: 0u16 },
    &amp;mut LineStringTable::default()), None);
line_program.end_sequence(0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/line.rs:500:17:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;gimli::write::line::LineProgram&gt;::op_advance (gimli/src/write/line.rs:500:17)
  &lt;gimli::write::line::LineProgram&gt;::end_sequence (gimli/src/write/line.rs:365:31)
  repro_gimli_src_write_line_rs_500_17_ab77fd::main (src/main.rs:12:18)</pre></td></tr>
<tr><td valign="top"><pre>let mut expression = Expression::raw(vec![]);
expression.set_target(0usize, 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/op.rs:182:18:
not implemented
backtrace:
  &lt;gimli::write::op::Expression&gt;::set_target (gimli/src/write/op.rs:182:18)
  repro_gimli_src_write_op_rs_182_18_a16ca5::main (src/main.rs:5:16)</pre></td></tr>
<tr><td valign="top"><pre>let mut line_program = LineProgram::new(Encoding { address_size: 0u8,
    format: Format::Dwarf64, version: 0u16 },
    LineEncoding { minimum_instruction_length: 1u8,
    maximum_operations_per_instruction: 10u8, default_is_stmt: false,
    line_base: 0i8, line_range: 1u8 },
    LineString::LineStringRef(LineStringTable::default().add::&lt;Vec&lt;u8&gt;&gt;(
    vec![])), None, LineString::String(vec![]), None);
line_program.end_sequence(10000000000000000000u64);</pre></td><td valign="top"><pre>thread 'main' panicked at gimli/src/write/line.rs:505:9:
attempt to multiply with overflow
backtrace:
  &lt;gimli::write::line::LineProgram&gt;::op_advance (gimli/src/write/line.rs:505:9)
  &lt;gimli::write::line::LineProgram&gt;::end_sequence (gimli/src/write/line.rs:365:31)
  repro_gimli_src_write_line_rs_505_9_a95ee8::main (src/main.rs:10:18)</pre></td></tr>
</table>
