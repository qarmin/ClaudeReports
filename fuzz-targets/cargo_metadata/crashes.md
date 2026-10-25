# cargo_metadata - crashes

Source: https://github.com/oli-obk/cargo_metadata @ 7cc507b09ac0d87bb83de45f61c17dbfb006f51b

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let workspace_default_members = WorkspaceDefaultMembers::default();
let _ = workspace_default_members.get(0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at cargo_metadata/src/lib.rs:387:14:
WorkspaceDefaultMembers should only be dereferenced on Cargo versions &gt;= 1.71
backtrace:
  &lt;cargo_metadata::WorkspaceDefaultMembers as core::ops::deref::Deref&gt;::deref (cargo_metadata/src/lib.rs:387:14)
  repro_cargo_metadata_src_lib_rs_387_14_44396f::main (src/main.rs:5:13)</pre></td></tr>
</table>
