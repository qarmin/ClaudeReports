# notify-rust - crashes

Source: https://github.com/hoodie/notify-rust @ 7841a687bea2212396a0c269ef1092b78ef3a270

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Image::from_rgb(-1000000000i32, -1000000000i32, vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at notify-rust/src/image.rs:45:27:
attempt to multiply with overflow
backtrace:
  &lt;notify_rust::image::Image&gt;::from_raw_data (notify-rust/src/image.rs:45:27)
  &lt;notify_rust::image::Image&gt;::from_rgb (notify-rust/src/image.rs:64:9)
  repro_notify_rust_src_image_rs_45_27_fd7f69::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::from_rgba(-1000000000i32, 1i32, vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at notify-rust/src/image.rs:45:26:
attempt to multiply with overflow
backtrace:
  &lt;notify_rust::image::Image&gt;::from_raw_data (notify-rust/src/image.rs:45:26)
  &lt;notify_rust::image::Image&gt;::from_rgba (notify-rust/src/image.rs:71:9)
  repro_notify_rust_src_image_rs_45_26_622f23::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Image::from_rgb(-1000000000i32, 0i32, vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at notify-rust/src/image.rs:54:28:
attempt to multiply with overflow
backtrace:
  &lt;notify_rust::image::Image&gt;::from_raw_data (notify-rust/src/image.rs:54:28)
  &lt;notify_rust::image::Image&gt;::from_rgb (notify-rust/src/image.rs:64:9)
  repro_notify_rust_src_image_rs_54_28_482d3b::main (src/main.rs:4:13)</pre></td></tr>
</table>
