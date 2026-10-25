# xkeysym - crashes

Source: https://github.com/notgull/xkeysym @ 77596eb8319f8d7a97db6477f307a94a0905c439

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let key_code = KeyCode::default();
let _ = xkeysym::keysym(key_code, 0u8, KeyCode::from(1u32), 0u8, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at xkeysym/src/lib.rs:304:17:
attempt to subtract with overflow
backtrace:
  xkeysym::keysym (xkeysym/src/lib.rs:304:17)
  repro_xkeysym_src_lib_rs_304_17_50481c::main (src/main.rs:5:13)</pre></td></tr>
</table>
