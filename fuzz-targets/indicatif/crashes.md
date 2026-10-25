# indicatif - crashes

Source: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = InMemoryTerm::new(1u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/in_memory.rs:21:9:
cols must be &gt; 0
backtrace:
  &lt;indicatif::in_memory::InMemoryTerm&gt;::new (indicatif/src/in_memory.rs:21:9)
  repro_indicatif_src_in_memory_rs_21_9_342603::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = InMemoryTerm::new(0u16, 0u16);</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/in_memory.rs:20:9:
rows must be &gt; 0
backtrace:
  &lt;indicatif::in_memory::InMemoryTerm&gt;::new (indicatif/src/in_memory.rs:20:9)
  repro_indicatif_src_in_memory_rs_20_9_f59208::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::width(
    &amp;InMemoryTerm::new(10000u16, 10000u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_width_abee5d::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::height(
    &amp;InMemoryTerm::new(10000u16, 10000u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_height_77b2b8::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::move_cursor_down(
    &amp;InMemoryTerm::new(10000u16, 10000u16), 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_move_cursor_down_d9d37f::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let in_memory_term = InMemoryTerm::new(u16::MAX, 1000u16);
in_memory_term.reset();</pre></td><td valign="top"><pre>out of memory: memory allocation of 32000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::reset (src/in_memory.rs:29)
  repro_out_of_memory_in_indicatif_inmemoryterm_reset_29a40d::main (src/main.rs:5)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = InMemoryTerm::new(10000u16, 10000u16);</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_new_bf295c::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = MultiProgress::new();
let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::move_cursor_up(
    &amp;InMemoryTerm::new(10000u16, 10000u16), 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_move_cursor_up_cdb4f6::main (src/main.rs:6)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::write_str(
    &amp;InMemoryTerm::new(10000u16, 10000u16), "");</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_write_str_0c5f48::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::write_line(
    &amp;InMemoryTerm::new(10000u16, 10000u16), "");</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_write_line_54e92f::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::flush(
    &amp;InMemoryTerm::new(10000u16, 10000u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_flush_a9df8e::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::move_cursor_right(
    &amp;InMemoryTerm::new(10000u16, 10000u16), 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_move_cursor_right_ba7ea2::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::move_cursor_left(
    &amp;InMemoryTerm::new(10000u16, 10000u16), 0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_move_cursor_left_f76c08::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::clear_line(
    &amp;InMemoryTerm::new(10000u16, 10000u16));</pre></td><td valign="top"><pre>out of memory: memory allocation of 320000 bytes failed
backtrace:
  vt100::row::Row::new (src/row.rs:12)
  vt100::grid::{impl#0}::allocate_rows::{closure#0} (src/grid.rs:39)
  vt100::grid::Grid::allocate_rows (src/grid.rs:37)
  vt100::screen::Screen::new (src/screen.rs:73)
  vt100::perform::WrappedScreen&lt;()&gt;::new_with_callbacks&lt;()&gt; (src/perform.rs:24)
  vt100::perform::WrappedScreen&lt;()&gt;::new (src/perform.rs:12)
  vt100::parser::Parser&lt;()&gt;::new (src/parser.rs:15)
  indicatif::in_memory::InMemoryTermState::new (src/in_memory.rs:207)
  indicatif::in_memory::InMemoryTerm::new (src/in_memory.rs:23)
  repro_out_of_memory_in_indicatif_inmemoryterm_clear_line_9146c2::main (src/main.rs:4)
  (25 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::write_line(
    &amp;InMemoryTerm::new(1u16, 1u16), "\n\t");</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/in_memory.rs:165:9:
calling write_line with embedded newlines is not allowed
backtrace:
  &lt;indicatif::in_memory::InMemoryTerm as indicatif::term_like::TermLike&gt;::write_line (indicatif/src/in_memory.rs:165:9)
  repro_indicatif_src_in_memory_rs_165_9_d7c7dc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = MultiProgress::with_draw_target(ProgressDrawTarget::stdout_with_hz(
    0u8)).insert_before(&amp;ProgressBar::hidden(), ProgressBar::no_length());</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/multi.rs:250:64:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;indicatif::multi::MultiProgress&gt;::insert_before (indicatif/src/multi.rs:250:64)
  repro_indicatif_src_multi_rs_250_64_a1e424::main (src/main.rs:6:86)</pre></td></tr>
<tr><td valign="top"><pre>let multi_progress = MultiProgress::default();
let _ = multi_progress.insert_after(&amp;ProgressBar::with_draw_target(None,
    ProgressDrawTarget::stderr_with_hz(0u8)), ProgressBar::new_spinner());</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/multi.rs:262:62:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;indicatif::multi::MultiProgress&gt;::insert_after (indicatif/src/multi.rs:262:62)
  repro_indicatif_src_multi_rs_262_62_e941db::main (src/main.rs:7:28)</pre></td></tr>
<tr><td valign="top"><pre>let progress_draw_target = ProgressDrawTarget::stdout();
let multi_progress = MultiProgress::with_draw_target(
    ProgressDrawTarget::stdout_with_hz(0u8));
multi_progress.remove(&amp;MultiProgress::default().insert(1usize,
    ProgressBar::with_draw_target(None, progress_draw_target)));</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/multi.rs:276:17:
assertion failed: Arc::ptr_eq(&amp;self.state, state)
backtrace:
  &lt;indicatif::multi::MultiProgress&gt;::remove (indicatif/src/multi.rs:276:17)
  repro_indicatif_src_multi_rs_276_17_fa310c::main (src/main.rs:8:20)</pre></td></tr>
<tr><td valign="top"><pre>let progress_style = ProgressStyle::with_template("").unwrap();
let _ = progress_style.clone().tick_chars("");</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/style.rs:106:9:
at least 2 tick chars required
backtrace:
  &lt;indicatif::style::ProgressStyle&gt;::tick_chars (indicatif/src/style.rs:106:9)
  repro_indicatif_src_style_rs_106_9_292d94::main (src/main.rs:5:36)</pre></td></tr>
<tr><td valign="top"><pre>let human_float_count = HumanFloatCount(1.0f64);
let string = human_float_count.to_string();
let _ = ProgressStyle::with_template(&amp;string).unwrap().progress_chars("");</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/style.rs:136:9:
at least 2 progress chars required
backtrace:
  &lt;indicatif::style::ProgressStyle&gt;::progress_chars (indicatif/src/style.rs:136:9)
  repro_indicatif_src_style_rs_136_9_16e748::main (src/main.rs:7:60)</pre></td></tr>
<tr><td valign="top"><pre>let progress_style = ProgressStyle::default_bar();
let progress_style2 = progress_style.clone().template("").unwrap();
let _ = progress_style2.clone().progress_chars("\u{3ffb6}\"");</pre></td><td valign="top"><pre>thread 'main' panicked at indicatif/src/style.rs:877:30:
assertion `left == right` failed: got passed un-equal width progress characters
  left: 2
 right: 1
backtrace:
  indicatif::style::width::{closure#1} (indicatif/src/style.rs:877:30)
  indicatif::style::width (indicatif/src/style.rs:874:10)
  &lt;indicatif::style::ProgressStyle&gt;::progress_chars (indicatif/src/style.rs:140:27)
  repro_indicatif_src_style_rs_877_30_f03593::main (src/main.rs:6:37)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;indicatif::InMemoryTerm as indicatif::TermLike&gt;::write_str(
    &amp;InMemoryTerm::new(1u16, 2u16), "👍🏽");</pre></td><td valign="top"><pre>thread 'main' panicked at vt100/src/grid.rs:683:13:
attempt to subtract with overflow
backtrace:
  &lt;vt100::grid::Grid&gt;::col_wrap (vt100/src/grid.rs:683:13)
  &lt;vt100::screen::Screen&gt;::text (vt100/src/screen.rs:745:25)
  &lt;vt100::perform::WrappedScreen as vte::Perform&gt;::print (vt100/src/perform.rs:38:25)
  &lt;vte::Parser&lt;1024&gt;&gt;::ground_dispatch::&lt;vt100::perform::WrappedScreen&gt; (vte/src/lib.rs:726:32)
  &lt;vte::Parser&lt;1024&gt;&gt;::advance_ground::&lt;vt100::perform::WrappedScreen&gt; (vte/src/lib.rs:611:17)
  &lt;vte::Parser&lt;1024&gt;&gt;::advance::&lt;vt100::perform::WrappedScreen&gt; (vte/src/lib.rs:119:44)
  &lt;vt100::parser::Parser&gt;::process (vt100/src/parser.rs:49:21)
  &lt;vt100::parser::Parser as core::io::write::Write&gt;::write (vt100/src/parser.rs:89:14)
  &lt;vt100::parser::Parser as core::io::write::Write&gt;::write_all (core/src/io/write.rs:216:24)
  &lt;indicatif::in_memory::InMemoryTermState&gt;::write_str (indicatif/src/in_memory.rs:213:21)
  &lt;indicatif::in_memory::InMemoryTerm as indicatif::term_like::TermLike&gt;::write_str (indicatif/src/in_memory.rs:179:15)
  repro_vt100_src_grid_rs_683_13_b689f5::main (src/main.rs:4:13)</pre></td></tr>
</table>
