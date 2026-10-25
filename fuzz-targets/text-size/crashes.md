# text-size - crashes

Source: https://github.com/rust-analyzer/text-size @ f5048daacdf8a8b7b6ff61572f75b42b8fe657ba

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = TextRange::at(TextSize::new(u32::MAX),
    TextSize::try_from(1usize).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at text-size/src/range.rs:69:46:
attempt to add with overflow
backtrace:
  &lt;text_size::range::TextRange&gt;::at (text-size/src/range.rs:69:46)
  repro_text_size_src_range_rs_69_46_ad59d0::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let text_size = TextSize::from(u32::MAX);
let text_size2 = TextSize::new(0u32);
let text_size3 = TextSize::new(1u32);
let _ = &lt;text_size::TextRange as ::core::ops::Add&lt;text_size::TextSize&gt;&gt;::add(
    TextRange::at(text_size3, text_size2), text_size);</pre></td><td valign="top"><pre>thread 'main' panicked at text-size/src/range.rs:422:14:
TextRange +offset overflowed
backtrace:
  &lt;text_size::range::TextRange as core::ops::arith::Add&lt;text_size::size::TextSize&gt;&gt;::add (text-size/src/range.rs:422:14)
  repro_text_size_src_range_rs_422_14_df0a7f::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let text_size = TextSize::from(0u32);
let text_range = TextRange::empty(text_size);
let text_range2 = text_range.cover_offset(text_size);
let text_range3 = text_range2.cover(text_range);
let _ = &lt;text_size::TextRange as ::core::ops::Sub&lt;text_size::TextSize&gt;&gt;::sub(
    text_range3, TextSize::try_from(1usize).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at text-size/src/range.rs:431:14:
TextRange -offset overflowed
backtrace:
  &lt;text_size::range::TextRange as core::ops::arith::Sub&lt;text_size::size::TextSize&gt;&gt;::sub (text-size/src/range.rs:431:14)
  repro_text_size_src_range_rs_431_14_1658a4::main (src/main.rs:9:13)</pre></td></tr>
</table>
