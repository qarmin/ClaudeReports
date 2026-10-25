# parley - crashes

Source: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Script::from_str_unchecked("a");</pre></td><td valign="top"><pre>thread 'main' panicked at parlance/src/script.rs:52:37:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;parlance::script::Script&gt;::from_str_unchecked (parlance/src/script.rs:52:37)
  repro_parlance_src_script_rs_52_37_e7c8e6::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Script::from_str_unchecked("");</pre></td><td valign="top"><pre>thread 'main' panicked at parlance/src/script.rs:52:27:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;parlance::script::Script&gt;::from_str_unchecked (parlance/src/script.rs:52:27)
  repro_parlance_src_script_rs_52_27_a25177::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let script = Script::from_bytes([255u8, 0u8, 0u8, 0u8]);
let _ = script.as_str();</pre></td><td valign="top"><pre>thread 'main' panicked at parlance/src/script.rs:66:41:
only ASCII: Utf8Error { valid_up_to: 0, error_len: Some(1) }
backtrace:
  &lt;parlance::script::Script&gt;::as_str (parlance/src/script.rs:66:41)
  repro_parlance_src_script_rs_66_41_39fd2d::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Script::from_str_unchecked("\u{feff}");</pre></td><td valign="top"><pre>thread 'main' panicked at parlance/src/script.rs:52:57:
index out of bounds: the len is 3 but the index is 3
backtrace:
  &lt;parlance::script::Script&gt;::from_str_unchecked (parlance/src/script.rs:52:57)
  repro_parlance_src_script_rs_52_57_59c163::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Script::from_str_unchecked("v6");</pre></td><td valign="top"><pre>thread 'main' panicked at parlance/src/script.rs:52:47:
index out of bounds: the len is 2 but the index is 2
backtrace:
  &lt;parlance::script::Script&gt;::from_str_unchecked (parlance/src/script.rs:52:47)
  repro_parlance_src_script_rs_52_47_636220::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut layout_context = &lt;parley::LayoutContext&lt;[u8; 4]&gt; as ::core::default::
    Default&gt;::default();
let mut font_context = FontContext::new();
let mut style_run_builder = layout_context.style_run_builder(&amp;mut font_context,
    "", 0.0f32, false);
style_run_builder.reserve(usize::MAX, 2usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from parley/src/builder.rs:145:30)
capacity overflow
backtrace:
  &lt;parley::builder::StyleRunBuilder&lt;[u8; 4]&gt;&gt;::reserve (parley/src/builder.rs:145:30)
  repro_parley_src_builder_rs_145_30_f848fa::main (src/main.rs:7:23)</pre></td></tr>
<tr><td valign="top"><pre>let mut layout_context = &lt;parley::LayoutContext&lt;u8&gt; as ::core::default::
    Default&gt;::default();
let mut font_context = FontContext::new();
let mut style_run_builder = layout_context.style_run_builder(&amp;mut font_context,
    "", 1.0f32, false);
style_run_builder.reserve(1000000000usize, 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 144000000000 bytes failed
backtrace:
  parley::builder::StyleRunBuilder&lt;u8&gt;::reserve&lt;u8&gt; (parley/src/builder.rs:145)
  repro_out_of_memory_in_parley_stylerunbuilder_u8_reserve_48d838::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut layout_context = &lt;parley::LayoutContext&lt;u32&gt; as ::core::default::
    Default&gt;::default();
let mut font_context = FontContext::default();
let mut style_run_builder = layout_context.style_run_builder(&amp;mut font_context,
    "", 0.0f32, false);
style_run_builder.reserve(0usize, 1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 24000000000 bytes failed
backtrace:
  parley::builder::StyleRunBuilder&lt;u32&gt;::reserve&lt;u32&gt; (parley/src/builder.rs:146)
  repro_out_of_memory_in_parley_stylerunbuilder_u32_reserve_e3ce34::main (src/main.rs:7)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut layout_context = &lt;parley::LayoutContext&lt;[u8; 4]&gt; as ::core::default::
    Default&gt;::default();
let mut font_context = FontContext { collection: Collection::default(),
    source_cache: SourceCache::new(SourceCacheOptions::default()) };
let mut style_run_builder = layout_context.style_run_builder(&amp;mut font_context,
    "", 0.0f32, false);
style_run_builder.reserve(0usize, 1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 24000000000 bytes failed
backtrace:
  parley::builder::StyleRunBuilder&lt;[u8; 4]&gt;::reserve&lt;[u8; 4]&gt; (parley/src/builder.rs:146)
  repro_out_of_memory_in_parley_stylerunbuilder_u8_4_reserve_355fa9::main (src/main.rs:10)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut layout = &lt;parley::Layout&lt;[u8; 4]&gt;&gt;::new();
layout.break_all_lines(Some(f32::NEG_INFINITY));</pre></td><td valign="top"><pre>thread 'main' panicked at parley/src/layout/line_break.rs:1029:9:
assertion failed: self.state.layout_max_advance == f32::INFINITY ||
    self.state.line_max_advance - self.state.layout_max_advance &lt; 1.0
backtrace:
  &lt;parley::layout::line_break::BreakLines&lt;[u8; 4]&gt;&gt;::break_next (parley/src/layout/line_break.rs:1029:9)
  &lt;parley::layout::line_break::BreakLines&lt;[u8; 4]&gt;&gt;::break_remaining (parley/src/layout/line_break.rs:1435:43)
  &lt;parley::layout::layout::Layout&lt;[u8; 4]&gt;&gt;::break_all_lines (parley/src/layout/layout.rs:212:14)
  repro_parley_src_layout_line_break_rs_1029_9_acb874::main (src/main.rs:3:12)</pre></td></tr>
</table>
