# subtle - crashes

Source: https://github.com/dalek-cryptography/subtle @ 369e7463e85921377a5f2df80aabcbbc6d57a930

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Choice::from(100u8);</pre></td><td valign="top"><pre>thread 'main' panicked at subtle/src/lib.rs:238:9:
assertion failed: (input == 0u8) | (input == 1u8)
backtrace:
  &lt;subtle::Choice as core::convert::From&lt;u8&gt;&gt;::from (subtle/src/lib.rs:238:9)
  repro_subtle_src_lib_rs_238_9_9b5aec::main (src/main.rs:4:13)</pre></td></tr>
</table>
