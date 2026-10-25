# difflib - crashes

Source: https://github.com/DimaKudosh/difflib @ f035fb8e656f27119e23eca9d5b996df14f3885e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Differ::restore(&amp;[], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at difflib/src/differ.rs:294:13:
Second parameter must be 1 or 2
backtrace:
  &lt;difflib::differ::Differ&gt;::restore (difflib/src/differ.rs:294:13)
  repro_difflib_src_differ_rs_294_13_914803::main (src/main.rs:4:13)</pre></td></tr>
</table>
