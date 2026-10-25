# comfy-table - crashes

Source: https://github.com/nukesor/comfy-table @ b947cb6bf4a9aef846e4ec00a67f68bd85d41c73

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = comfy_table::utils::arrangement::helper::count_border_columns(
    &amp;Table::default(), usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at comfy-table/src/utils/arrangement/helper.rs:51:9:
attempt to add with overflow
backtrace:
  comfy_table::utils::arrangement (comfy-table/src/utils/arrangement/helper.rs:51:9)
  repro_comfy_table_src_utils_arrangement_helper_rs_51_9_43574e::main (src/main.rs:4:13)</pre></td></tr>
</table>
