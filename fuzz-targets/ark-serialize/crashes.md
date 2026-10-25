# ark-serialize - crashes

Source: https://github.com/arkworks-rs/algebra @ 4a19a1c50606ae4a0e96ccce107f6281aa0be38e

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = ark_serialize::buffer_bit_byte_size(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at serialize/src/lib.rs:289:6:
attempt to multiply with overflow
backtrace:
  ark_serialize::buffer_bit_byte_size (serialize/src/lib.rs:289:6)
  repro_serialize_src_lib_rs_289_6_0cf33e::main (src/main.rs:2:13)</pre></td></tr>
</table>
