# unicode-bidi - crashes

Source: https://github.com/servo/unicode-bidi @ 9a8c7b03be01def5cf7014b89d5af90f7382fdad

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let bidi_info = BidiInfo::new(&amp;string, None);
let _ = bidi_info.visual_runs(&amp;ParagraphInfo { range: 0usize..0usize,
    level: Level::from(0u8) }, 0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:548:9:
assertion failed: line.end &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:548:9)
  &lt;unicode_bidi::BidiInfo&gt;::visual_runs (unicode-bidi/src/lib.rs:674:27)
  repro_unicode_bidi_src_lib_rs_548_9_ff8753::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = unicode_bidi::deprecated::visual_runs(1usize..1000000000000000000usize,
    &amp;[LTR_LEVEL]);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/deprecated.rs:30:5:
assertion failed: line.end &lt;= levels.len()
backtrace:
  unicode_bidi::deprecated::visual_runs (unicode-bidi/src/deprecated.rs:30:5)
  repro_unicode_bidi_src_deprecated_rs_30_5_aa26cf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = unicode_bidi::deprecated::visual_runs(2usize..0usize, &amp;[Level::ltr(),
    LTR_LEVEL]);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/deprecated.rs:36:25:
index out of bounds: the len is 2 but the index is 2
backtrace:
  unicode_bidi::deprecated::visual_runs (unicode-bidi/src/deprecated.rs:36:25)
  repro_unicode_bidi_src_deprecated_rs_36_25_60c852::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: usize::MAX..4294967295usize,
    level: Level::ltr() };
let _ = paragraph_info.len();</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:188:9:
attempt to subtract with overflow
backtrace:
  &lt;unicode_bidi::ParagraphInfo&gt;::len (unicode-bidi/src/lib.rs:188:9)
  repro_unicode_bidi_src_lib_rs_188_9_8c12cb::main (src/main.rs:6:28)</pre></td></tr>
<tr><td valign="top"><pre>let string = "Z".to_string();
let bidi_info = BidiInfo::new(&amp;string, None);
let _ = bidi_info.reordered_levels_per_char(
    &amp;ParagraphInfo { range: 0usize..0usize, level: RTL_LEVEL }, 1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:551:50:
slice index starts at 1 but ends at 0
backtrace:
  &lt;[unicode_bidi::char_data::tables::BidiClass] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:551:50)
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels_per_char (unicode-bidi/src/lib.rs:583:27)
  repro_unicode_bidi_src_lib_rs_551_50_e0868f::main (src/main.rs:8:23)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: 0usize..0usize,
    level: Level::ltr() };
let hardcoded_bidi_data = HardcodedBidiData;
let string = "".to_string();
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;string, None);
let _ = bidi_info.reordered_levels(&amp;paragraph_info, 1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:547:9:
assertion failed: line.start &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:547:9)
  repro_unicode_bidi_src_lib_rs_547_9_c562f8::main (src/main.rs:11:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "".to_string();
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;string, None);
let _ = bidi_info.reorder_line(&amp;ParagraphInfo { range: 1usize..0usize,
    level: Level::new_explicit(0u8).unwrap() }, 0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:595:40:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::BidiInfo&gt;::reorder_line (unicode-bidi/src/lib.rs:595:40)
  repro_unicode_bidi_src_lib_rs_595_40_8cbd85::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Level::from(255u8);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/level.rs:235:28:
Level number error: OutOfRangeNumber
backtrace:
  &lt;unicode_bidi::level::Level as core::convert::From&lt;u8&gt;&gt;::from (unicode-bidi/src/level.rs:235:28)
  repro_unicode_bidi_src_level_rs_235_28_4b1cfc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: 0usize..0usize,
    level: Level::new(0u8).unwrap() };
let string = "\u{896db}".to_string();
let bidi_info = BidiInfo::new(&amp;string, None);
let _ = bidi_info.reorder_line(&amp;paragraph_info, 1usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:596:29:
start byte index 1 is not a char boundary; it is inside '\u{896db}' (bytes 0..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;unicode_bidi::BidiInfo&gt;::reorder_line (unicode-bidi/src/lib.rs:596:29)
  repro_unicode_bidi_src_lib_rs_596_29_6a1b0c::main (src/main.rs:9:23)</pre></td></tr>
<tr><td valign="top"><pre>let _ = unicode_bidi::deprecated::visual_runs(usize::MAX..4294967296usize,
    &amp;[Level::ltr()]);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/deprecated.rs:29:5:
assertion failed: line.start &lt;= levels.len()
backtrace:
  unicode_bidi::deprecated::visual_runs (unicode-bidi/src/deprecated.rs:29:5)
  repro_unicode_bidi_src_deprecated_rs_29_5_ba28fb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let _ = bidi_info.reordered_levels(&amp;ParagraphInfo { range: 0usize..0usize,
    level: Level::new_explicit(0u8).unwrap() }, 1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:228:9:
assertion failed: line.start &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::utf16::BidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:228:9)
  repro_unicode_bidi_src_utf16_rs_228_9_f0b988::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "a".to_string();
let paragraph_bidi_info = ParagraphBidiInfo::
    new_with_data_source::&lt;unicode_bidi::HardcodedBidiData&gt;(
    &amp;hardcoded_bidi_data, &amp;string, None);
let _ = paragraph_bidi_info.reordered_levels(1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:795:50:
slice index starts at 1 but ends at 0
backtrace:
  &lt;[unicode_bidi::char_data::tables::BidiClass] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:795:50)
  repro_unicode_bidi_src_lib_rs_795_50_85d886::main (src/main.rs:8:33)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;vec, None);
let _ = paragraph_bidi_info.reorder_line(0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:505:40:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reorder_line (unicode-bidi/src/utf16.rs:505:40)
  repro_unicode_bidi_src_utf16_rs_505_40_260a9e::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "\u{72cbc}".to_string();
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;string, None);
let _ = bidi_info.reordered_levels_per_char(
    &amp;ParagraphInfo { range: 0usize..0usize, level: Level::new(1u8).unwrap() },
    1usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:1392:14:
start byte index 1 is not a char boundary; it is inside '\u{72cbc}' (bytes 0..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;str as unicode_bidi::TextSource&gt;::subrange (unicode-bidi/src/lib.rs:1392:14)
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:557:23)
  &lt;unicode_bidi::BidiInfo&gt;::reordered_levels_per_char (unicode-bidi/src/lib.rs:583:27)
  repro_unicode_bidi_src_lib_rs_1392_14_753df9::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let paragraph_info = ParagraphInfo { range: 0usize..1usize,
    level: Level::new(0u8).unwrap() };
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.direction();</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:614:41:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::Paragraph&gt;::direction (unicode-bidi/src/utf16.rs:614:41)
  repro_unicode_bidi_src_utf16_rs_614_41_c1b26a::main (src/main.rs:13:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let _ = bidi_info.visual_runs(&amp;ParagraphInfo { range: 0usize..0usize,
    level: LTR_LEVEL }, 0usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:946:31:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[unicode_bidi::level::Level]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  unicode_bidi::visual_runs_for_line (unicode-bidi/src/lib.rs:946:31)
  &lt;unicode_bidi::utf16::BidiInfo&gt;::visual_runs (unicode-bidi/src/utf16.rs:352:9)
  repro_unicode_bidi_src_lib_rs_946_31_ae1206::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![0u16];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let _ = bidi_info.reordered_levels(&amp;ParagraphInfo { range: 0usize..0usize,
    level: Level::new(0u8).unwrap() }, 1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:232:50:
slice index starts at 1 but ends at 0
backtrace:
  &lt;[unicode_bidi::char_data::tables::BidiClass] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::BidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:232:50)
  repro_unicode_bidi_src_utf16_rs_232_50_d62ba0::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let paragraph_info = ParagraphInfo { range: 0usize..0usize,
    level: Level::ltr() };
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.level_at(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:621:25:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[unicode_bidi::level::Level]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::Paragraph&gt;::level_at (unicode-bidi/src/utf16.rs:621:25)
  repro_unicode_bidi_src_utf16_rs_621_25_299c6c::main (src/main.rs:13:23)</pre></td></tr>
<tr><td valign="top"><pre>let string = "".to_string();
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;string, None);
let _ = paragraph_bidi_info.reorder_line(0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:828:40:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::ParagraphBidiInfo&gt;::reorder_line (unicode-bidi/src/lib.rs:828:40)
  repro_unicode_bidi_src_lib_rs_828_40_50f352::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "".to_string();
let paragraph_bidi_info = ParagraphBidiInfo::
    new_with_data_source::&lt;unicode_bidi::HardcodedBidiData&gt;(
    &amp;hardcoded_bidi_data, &amp;string, None);
let _ = paragraph_bidi_info.reordered_levels(1usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:791:9:
assertion failed: line.start &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:791:9)
  repro_unicode_bidi_src_lib_rs_791_9_413b6d::main (src/main.rs:8:33)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: 0usize..0usize,
    level: Level::from(1u8) };
let string = "".to_string();
let bidi_info = BidiInfo::new(&amp;string, None);
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.level_at(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:1248:25:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[unicode_bidi::level::Level]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::Paragraph&gt;::level_at (unicode-bidi/src/lib.rs:1248:25)
  repro_unicode_bidi_src_lib_rs_1248_25_55be31::main (src/main.rs:11:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "a".to_string();
let paragraph_bidi_info = ParagraphBidiInfo::
    new_with_data_source::&lt;unicode_bidi::HardcodedBidiData&gt;(
    &amp;hardcoded_bidi_data, &amp;string, None);
let _ = paragraph_bidi_info.reordered_levels(1usize..10usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:792:9:
assertion failed: line.end &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/lib.rs:792:9)
  repro_unicode_bidi_src_lib_rs_792_9_8c9fca::main (src/main.rs:8:33)</pre></td></tr>
<tr><td valign="top"><pre>let string = "\u{feff}".to_string();
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;string, None);
let _ = paragraph_bidi_info.reorder_line(0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:829:29:
end byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;unicode_bidi::ParagraphBidiInfo&gt;::reorder_line (unicode-bidi/src/lib.rs:829:29)
  repro_unicode_bidi_src_lib_rs_829_29_863742::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;vec, None);
let _ = paragraph_bidi_info.reordered_levels_per_char(0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:469:9:
assertion failed: line.end &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:469:9)
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reordered_levels_per_char (unicode-bidi/src/utf16.rs:494:27)
  repro_unicode_bidi_src_utf16_rs_469_9_8b3d23::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let _ = bidi_info.reorder_line(&amp;ParagraphInfo { range: 0usize..0usize,
    level: Level::new_explicit(0u8).unwrap() }, 0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:272:40:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::BidiInfo&gt;::reorder_line (unicode-bidi/src/utf16.rs:272:40)
  repro_unicode_bidi_src_utf16_rs_272_40_003382::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let paragraph_info = ParagraphInfo { range: usize::MAX..0usize,
    level: Level::new(0u8).unwrap() };
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.level_at(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:620:31:
attempt to add with overflow
backtrace:
  &lt;unicode_bidi::utf16::Paragraph&gt;::level_at (unicode-bidi/src/utf16.rs:620:31)
  repro_unicode_bidi_src_utf16_rs_620_31_060ed3::main (src/main.rs:13:23)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: usize::MAX..0usize,
    level: Level::ltr() };
let hardcoded_bidi_data = HardcodedBidiData;
let string = "".to_string();
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;string, None);
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.level_at(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:1247:31:
attempt to add with overflow
backtrace:
  &lt;unicode_bidi::Paragraph&gt;::level_at (unicode-bidi/src/lib.rs:1247:31)
  repro_unicode_bidi_src_lib_rs_1247_31_1ff52f::main (src/main.rs:13:23)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;vec, None);
let _ = paragraph_bidi_info.visual_runs(1usize..0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:468:9:
assertion failed: line.start &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:468:9)
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::visual_runs (unicode-bidi/src/utf16.rs:531:27)
  repro_unicode_bidi_src_utf16_rs_468_9_8b182f::main (src/main.rs:6:33)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let string = "".to_string();
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;string, None);
let paragraph_info = ParagraphInfo { range: 0usize..1usize,
    level: Level::rtl() };
let paragraph = Paragraph::new(&amp;bidi_info, &amp;paragraph_info);
let _ = paragraph.direction();</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/lib.rs:1241:41:
range end index 1 out of range for slice of length 0
backtrace:
  &lt;[unicode_bidi::level::Level] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::Paragraph&gt;::direction (unicode-bidi/src/lib.rs:1241:41)
  repro_unicode_bidi_src_lib_rs_1241_41_dbd4b1::main (src/main.rs:13:23)</pre></td></tr>
<tr><td valign="top"><pre>let u8_v = Level::max_implicit_depth();
let string = "ﰍ".to_string();
let _ = BidiInfo::new(&amp;string, Some(Level::from(u8_v)));</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/implicit.rs:596:36:
Level number error: OutOfRangeNumber
backtrace:
  unicode_bidi::implicit::resolve_levels (unicode-bidi/src/implicit.rs:596:36)
  unicode_bidi::compute_bidi_info_for_para::&lt;unicode_bidi::char_data::HardcodedBidiData, str&gt; (unicode-bidi/src/lib.rs:1153:5)
  &lt;unicode_bidi::BidiInfo&gt;::new_with_data_source::&lt;unicode_bidi::char_data::HardcodedBidiData&gt; (unicode-bidi/src/lib.rs:512:13)
  &lt;unicode_bidi::BidiInfo&gt;::new (unicode-bidi/src/lib.rs:485:9)
  repro_unicode_bidi_src_implicit_rs_596_36_7e33d9::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let paragraph_info = ParagraphInfo { range: 65535usize..65535usize,
    level: Level::new_explicit(0u8).unwrap() };
let usize_v = paragraph_info.len();
let vec = vec![0u16];
let paragraph_bidi_info = ParagraphBidiInfo::new(&amp;vec, None);
let _ = paragraph_bidi_info.reordered_levels_per_char(1usize..usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:472:50:
slice index starts at 1 but ends at 0
backtrace:
  &lt;[unicode_bidi::char_data::tables::BidiClass] as core::ops::index::Index&lt;core::ops::range::Range&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:472:50)
  &lt;unicode_bidi::utf16::ParagraphBidiInfo&gt;::reordered_levels_per_char (unicode-bidi/src/utf16.rs:494:27)
  repro_unicode_bidi_src_utf16_rs_472_50_8560fa::main (src/main.rs:10:33)</pre></td></tr>
<tr><td valign="top"><pre>let u8_v = Level::max_implicit_depth();
let hardcoded_bidi_data = HardcodedBidiData;
let vec = vec![1639u16];
let _ = BidiInfo::new_with_data_source::&lt;unicode_bidi::HardcodedBidiData&gt;(
    &amp;hardcoded_bidi_data, &amp;vec, Some(Level::from(u8_v)));</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/implicit.rs:594:61:
Level number error: OutOfRangeNumber
backtrace:
  unicode_bidi::implicit::resolve_levels (unicode-bidi/src/implicit.rs:594:61)
  unicode_bidi::compute_bidi_info_for_para::&lt;unicode_bidi::char_data::HardcodedBidiData, [u16]&gt; (unicode-bidi/src/lib.rs:1153:5)
  &lt;unicode_bidi::utf16::BidiInfo&gt;::new_with_data_source::&lt;unicode_bidi::char_data::HardcodedBidiData&gt; (unicode-bidi/src/utf16.rs:193:13)
  repro_unicode_bidi_src_implicit_rs_594_61_b7ee7e::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let hardcoded_bidi_data = HardcodedBidiData;
let level = RTL_LEVEL;
let vec = vec![];
let bidi_info = BidiInfo::new_with_data_source::&lt;unicode_bidi::
    HardcodedBidiData&gt;(&amp;hardcoded_bidi_data, &amp;vec, None);
let _ = bidi_info.reordered_levels_per_char(
    &amp;ParagraphInfo { range: 0usize..0usize, level: level }, 0usize..1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-bidi/src/utf16.rs:229:9:
assertion failed: line.end &lt;= self.levels.len()
backtrace:
  &lt;unicode_bidi::utf16::BidiInfo&gt;::reordered_levels (unicode-bidi/src/utf16.rs:229:9)
  &lt;unicode_bidi::utf16::BidiInfo&gt;::reordered_levels_per_char (unicode-bidi/src/utf16.rs:260:27)
  repro_unicode_bidi_src_utf16_rs_229_9_7ea3d3::main (src/main.rs:11:23)</pre></td></tr>
</table>
