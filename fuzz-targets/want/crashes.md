# want - crashes

Source: https://github.com/seanmonstar/want @ 322291ce5a87c72ecfd83d383d0d7f5245cf72c6

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut taker = { want::new().1 };
taker.cancel();
taker.want();</pre></td><td valign="top"><pre>thread 'main' panicked at want/src/lib.rs:333:9:
want called after cancel
backtrace:
  &lt;want::Taker&gt;::want (want/src/lib.rs:333:9)
  repro_want_src_lib_rs_333_9_cda1e7::main (src/main.rs:4:11)</pre></td></tr>
</table>
