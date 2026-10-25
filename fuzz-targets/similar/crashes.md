# similar - crashes

Source: https://github.com/mitsuhiko/similar @ 96993d92851eff9a484ea157614de30f80dfdc73

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = UnifiedHunkHeader::new(&amp;[DiffOp::Insert { old_index: 1usize,
    new_index: usize::MAX, new_len: 1usize }]);</pre></td><td valign="top"><pre>thread 'main' panicked at similar/src/types.rs:348:28:
attempt to add with overflow
backtrace:
  &lt;similar::types::DiffOp&gt;::as_tag_tuple (similar/src/types.rs:348:28)
  &lt;similar::types::DiffOp&gt;::old_range (similar/src/types.rs:304:14)
  &lt;similar::udiff::UnifiedHunkHeader&gt;::new (similar/src/udiff.rs:90:31)
  repro_similar_src_types_rs_348_28_840810::main (src/main.rs:5:13)</pre></td></tr>
</table>
