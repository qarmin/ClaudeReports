# x11-dl - crashes

Source: https://github.com/AltF02/x11-rs @ ff57a942e5a4d35d8660362a8ca2ac0c02fa4852

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let client_message_data = ClientMessageData::from([1u64, 0u64, 0u64, 0u64,
    1u64]);
let _ = client_message_data.get_byte(1000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at x11-dl/src/xlib.rs:2675:9:
index out of bounds: the len is 20 but the index is 1000000000
backtrace:
  &lt;x11_dl::xlib::ClientMessageData&gt;::get_byte (x11-dl/src/xlib.rs:2675:9)
  repro_x11_dl_src_xlib_rs_2675_9_a7194f::main (src/main.rs:5:33)</pre></td></tr>
<tr><td valign="top"><pre>let client_message_data = ClientMessageData::from([0i64, 0i64, 0i64, 0i64,
    0i64]);
let _ = client_message_data.get_long(1000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at x11-dl/src/xlib.rs:2679:9:
index out of bounds: the len is 5 but the index is 1000000000
backtrace:
  &lt;x11_dl::xlib::ClientMessageData&gt;::get_long (x11-dl/src/xlib.rs:2679:9)
  repro_x11_dl_src_xlib_rs_2679_9_c91009::main (src/main.rs:5:33)</pre></td></tr>
<tr><td valign="top"><pre>let client_message_data = ClientMessageData::default();
let _ = client_message_data.get_short(100usize);</pre></td><td valign="top"><pre>thread 'main' panicked at x11-dl/src/xlib.rs:2683:9:
index out of bounds: the len is 10 but the index is 100
backtrace:
  &lt;x11_dl::xlib::ClientMessageData&gt;::get_short (x11-dl/src/xlib.rs:2683:9)
  repro_x11_dl_src_xlib_rs_2683_9_a9e63c::main (src/main.rs:5:33)</pre></td></tr>
</table>
