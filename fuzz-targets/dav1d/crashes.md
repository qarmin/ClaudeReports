# dav1d - crashes

Source: https://github.com/rust-av/dav1d-rs @ 03477a36c3de4f2aacbab81a8951150ffdb9c4c3

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Error::from(0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at dav1d-rs/src/lib.rs:42:9:
assertion failed: err &lt; 0
backtrace:
  &lt;dav1d::Error as core::convert::From&lt;i32&gt;&gt;::from (dav1d-rs/src/lib.rs:42:9)
  repro_dav1d_rs_src_lib_rs_42_9_20f61e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = PlanarImageComponent::from(1000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at dav1d-rs/src/lib.rs:854:18:
Invalid YUV index: 1000000000
backtrace:
  &lt;dav1d::PlanarImageComponent as core::convert::From&lt;usize&gt;&gt;::from (dav1d-rs/src/lib.rs:854:18)
  repro_dav1d_rs_src_lib_rs_854_18_ceee8d::main (src/main.rs:4:13)</pre></td></tr>
</table>
