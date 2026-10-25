# textwrap - crashes

Source: https://github.com/mgeisler/textwrap @ 07fdbbb92103a22c48a61be271dca36c739fdde3

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = textwrap::wrap_columns::&lt;textwrap::Options&gt;("", 10000usize,
    Options::new(usize::MAX), "\0", "�\u{10ffff}", "مرحبا".repeat(1034));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1844674407370952 bytes failed
backtrace:
  textwrap::columns::wrap_columns&lt;textwrap::options::Options&gt; (textwrap/src/columns.rs:97)
  repro_out_of_memory_in_textwrap_wrap_columns_625726::main (src/main.rs:4)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let wrap_algorithm = WrapAlgorithm::default();
let options = Options::with_termwidth();
let options2 = options.clone().wrap_algorithm(wrap_algorithm);
let _ = textwrap::wrap_columns::&lt;textwrap::Options&gt;("", 4294967296usize,
    options2.clone(), "\u{feff}", "", "\u{200b}\u{200d}".repeat(125));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  textwrap::columns::wrap_columns&lt;textwrap::options::Options&gt; (textwrap/src/columns.rs:101)
  repro_timeout_in_textwrap_wrap_columns_2eb088::main (src/main.rs:8)
  (8 std/libc frames omitted)</pre></td></tr>
</table>
