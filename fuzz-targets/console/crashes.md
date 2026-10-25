# console - crashes

Source: https://github.com/console-rs/console @ 4329b774d2e7117df706193636b4734b147e6554

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let term = Term::buffered_stdout();
let _ = term.clear_last_lines(1000000000usize);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  console::term::Term::write_str (src/term.rs:239)
  console::common_term::move_cursor_down (src/common_term.rs:7)
  console::term::Term::move_cursor_down (src/term.rs:449)
  console::term::Term::clear_last_lines (src/term.rs:485)
  repro_timeout_in_console_term_clear_last_lines_566c85::main (src/main.rs:5)
  (11 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let term = Term::buffered_stderr();
let _ = term.move_cursor_to(0usize, usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at console/src/common_term.rs:38:43:
attempt to add with overflow
backtrace:
  console::common_term::move_cursor_to (console/src/common_term.rs:38:43)
  &lt;console::term::Term&gt;::move_cursor_to (console/src/term.rs:431:9)
  repro_console_src_common_term_rs_38_43_e13547::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let term = Term::stdout();
let _ = term.move_cursor_to(usize::MAX, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at console/src/common_term.rs:38:50:
attempt to add with overflow
backtrace:
  console::common_term::move_cursor_to (console/src/common_term.rs:38:50)
  &lt;console::term::Term&gt;::move_cursor_to (console/src/term.rs:431:9)
  repro_console_src_common_term_rs_38_50_b5ecd0::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let term = Term::stderr();
let _ = term.move_cursor_right(1usize);
let _ = term.clear_last_lines(4294967295usize);</pre></td><td valign="top"><pre>\x1b[1C\x1b[4294967295A\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1b[2K\x1b[1B\x0d\x1... (+20 chars)</pre></td></tr>
</table>
