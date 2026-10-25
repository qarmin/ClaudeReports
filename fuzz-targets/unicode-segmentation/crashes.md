# unicode-segmentation - crashes

Source: https://github.com/unicode-rs/unicode-segmentation @ e77d18649cbc109fefd15a9d7db8320480abfd4e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, false);
let _ = grapheme_cursor.is_boundary("\u{1c30b}", 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-segmentation/src/grapheme.rs:660:27:
start byte index 1 is not a char boundary; it is inside '\u{1c30b}' (bytes 0..4 of string)
backtrace:
  &lt;str as core::ops::index::Index&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index (core/src/str/traits.rs:63:15)
  &lt;unicode_segmentation::grapheme::GraphemeCursor&gt;::is_boundary (unicode-segmentation/src/grapheme.rs:660:27)
  repro_unicode_segmentation_src_grapheme_rs_660_27_25eb87::main (src/main.rs:5:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut grapheme_cursor = GraphemeCursor::new(1usize, 0usize, true);
let usize_v = grapheme_cursor.cur_cursor();
let _ = grapheme_cursor.is_boundary("a", usize_v);
grapheme_cursor.provide_context("", 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-segmentation/src/grapheme.rs:464:48:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;unicode_segmentation::grapheme::GraphemeCursor&gt;::provide_context (unicode-segmentation/src/grapheme.rs:464:48)
  repro_unicode_segmentation_src_grapheme_rs_464_48_1f98fe::main (src/main.rs:7:21)</pre></td></tr>
<tr><td valign="top"><pre>let mut grapheme_cursor = GraphemeCursor::new(2usize, 1usize, false);
let usize_v = grapheme_cursor.cur_cursor();
let _ = grapheme_cursor.is_boundary("a", usize_v);
grapheme_cursor.provide_context("", usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at unicode-segmentation/src/grapheme.rs:476:56:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;unicode_segmentation::grapheme::GraphemeCursor&gt;::provide_context (unicode-segmentation/src/grapheme.rs:476:56)
  repro_unicode_segmentation_src_grapheme_rs_476_56_545ca2::main (src/main.rs:7:21)</pre></td></tr>
</table>
