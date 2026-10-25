# libc - crashes

Source: https://github.com/rust-lang/libc @ d490042d3390bb5f023a2cfaf85fa8c5606c04dc

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = libc::_IO(1000000000u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at libc/src/unix/linux_like/mod.rs:1781:13:
assertion failed: ty &lt;= _IOC_TYPEMASK
backtrace:
  libc::unix::linux_like::_IOC (libc/src/unix/linux_like/mod.rs:1781:13)
  libc::unix::linux_like::_IO (libc/src/unix/linux_like/mod.rs:1793:13)
  repro_libc_src_unix_linux_like_mod_rs_1781_13_4879f2::main (src/main.rs:2:13)</pre></td></tr>
</table>
