# snap - crashes

Source: https://github.com/BurntSushi/rust-snappy @ 29fcab53647bcd8cd1550c06bde0cd42777e5565

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut decoder = Decoder::new();
let _ = decoder.decompress_vec(&amp;vec![144u8, 216u8, 179u8, 191u8, 11u8]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 3085757456 bytes failed
backtrace:
  snap::decompress::Decoder::decompress_vec (src/decompress.rs:106)
  repro_out_of_memory_in_snap_raw_decoder_decompress_vec_94bbe5::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
</table>
