# ttf-parser - crashes

Source: https://github.com/harfbuzz/ttf-parser @ 0c7291223fe9e0bc2808f0255cb991066eedd796

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;ttf_parser::colr::CompositeMode as ttf_parser::FromData&gt;::parse(
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ttf-parser/src/tables/colr.rs:642:15:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;ttf_parser::tables::colr::CompositeMode as ttf_parser::parser::FromData&gt;::parse (ttf-parser/src/tables/colr.rs:642:15)
  repro_ttf_parser_src_tables_colr_rs_642_15_43c8a6::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ttf_parser::colr::GradientExtend as ttf_parser::FromData&gt;::parse(
    &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at ttf-parser/src/tables/colr.rs:208:15:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;ttf_parser::tables::colr::GradientExtend as ttf_parser::parser::FromData&gt;::parse (ttf-parser/src/tables/colr.rs:208:15)
  repro_ttf_parser_src_tables_colr_rs_208_15_fc012c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = NormalizedCoordinate::from(f32::NEG_INFINITY);</pre></td><td valign="top"><pre>thread 'main' panicked at ttf-parser/src/parser.rs:932:5:
assertion failed: val.is_finite()
backtrace:
  ttf_parser::parser::f32_bound (ttf-parser/src/parser.rs:932:5)
  &lt;ttf_parser::NormalizedCoordinate as core::convert::From&lt;f32&gt;&gt;::from (ttf-parser/src/lib.rs:177:31)
  repro_ttf_parser_src_parser_rs_932_5_b6d2a2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let rect = Rect { x_min: i16::MIN, y_min: 0i16, x_max: 0i16, y_max: 0i16 };
let _ = rect.width();</pre></td><td valign="top"><pre>thread 'main' panicked at ttf-parser/src/lib.rs:355:9:
attempt to subtract with overflow
backtrace:
  &lt;ttf_parser::Rect&gt;::width (ttf-parser/src/lib.rs:355:9)
  repro_ttf_parser_src_lib_rs_355_9_a6efc5::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let rect = Rect { x_min: 0i16, y_min: i16::MAX, x_max: 0i16, y_max: -10000i16 };
let _ = rect.height();</pre></td><td valign="top"><pre>thread 'main' panicked at ttf-parser/src/lib.rs:361:9:
attempt to subtract with overflow
backtrace:
  &lt;ttf_parser::Rect&gt; (ttf-parser/src/lib.rs:361:9)
  repro_ttf_parser_src_lib_rs_361_9_416cd9::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let subtable12 = Subtable12::parse(&amp;vec).unwrap();
subtable12.codepoints(|_| ());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ttf_parser::tables::cmap::format12::Subtable12::codepoints&lt;repro_timeout_in_ttf_parser_cmap_subtable12_codepoints_eeb8ca::main::{closure_env#0}&gt; (ttf-parser/src/tables/cmap/format12.rs:77)
  repro_timeout_in_ttf_parser_cmap_subtable12_codepoints_eeb8ca::main (src/main.rs:6)
  (13 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec = include_bytes!("data_0.bin").to_vec();
let subtable13 = Subtable13::parse(&amp;vec).unwrap();
subtable13.codepoints(|_| ());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ttf_parser::tables::cmap::format13::Subtable13::codepoints&lt;repro_timeout_in_ttf_parser_cmap_subtable13_codepoints_d81c5a::main::{closure_env#0}&gt; (ttf-parser/src/tables/cmap/format13.rs:44)
  repro_timeout_in_ttf_parser_cmap_subtable13_codepoints_d81c5a::main (src/main.rs:6)</pre></td></tr>
</table>
