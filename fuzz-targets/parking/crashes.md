# parking - crashes

Source: https://github.com/smol-rs/parking @ cedb1884e0502ad21d4863a21eec9e3f13c3cf5f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let parker = Parker::new();
parker.park();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  parking::Inner::park (src/lib.rs:377)
  parking::Parker::park (src/lib.rs:132)
  repro_timeout_in_parking_parker_park_1300fb::main (src/main.rs:5)
  (5 std/libc frames omitted)</pre></td></tr>
</table>
