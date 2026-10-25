# vte - crashes

Source: https://github.com/alacritty/vte @ abeae765dd546dfff60b278f0757dcc71beb8ab1

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut std_sync_handler = StdSyncHandler::default();
&lt;vte::ansi::StdSyncHandler as vte::ansi::Timeout&gt;::set_timeout(
    &amp;mut std_sync_handler, std::time::Duration::new(10000000000000000000u64,
    33u32 % 1_000_000_000));</pre></td><td valign="top"><pre>thread 'main' panicked at vte/src/ansi.rs:460:29:
overflow when adding duration to instant
backtrace:
  &lt;vte::ansi::StdSyncHandler as vte::ansi::Timeout&gt;::set_timeout (vte/src/ansi.rs:460:29)
  repro_vte_src_ansi_rs_460_29_3534db::main (src/main.rs:5:5)</pre></td></tr>
</table>
