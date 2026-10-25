# bitreader - crashes

Source: https://github.com/irauta/bitreader @ 22b586b1114b589ba3fdc7b854a3ccf836a80d29

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let vec = vec![0u8, 0u8, 74u8, 70u8, 73u8];
let bit_reader = BitReader::new(&amp;vec);
let mut bit_reader2 = bit_reader.relative_reader_atmost(u64::MAX);
let _ = bit_reader2.read_i64(38u8);
let _ = bit_reader2.skip(18446744073709551614u64);</pre></td><td valign="top"><pre>thread 'main' panicked at bitreader/src/lib.rs:285:28:
attempt to add with overflow
backtrace:
  &lt;bitreader::BitReader&gt;::skip (bitreader/src/lib.rs:285:28)
  repro_bitreader_src_lib_rs_285_28_60cdfd::main (src/main.rs:8:25)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let bit_reader = BitReader::new(&amp;vec);
let _ = bit_reader.is_aligned(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at bitreader/src/lib.rs:321:9:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;bitreader::BitReader&gt;::is_aligned (bitreader/src/lib.rs:321:9)
  repro_bitreader_src_lib_rs_321_9_1918c7::main (src/main.rs:6:24)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let mut bit_reader = BitReader::new(&amp;vec);
let _ = bit_reader.align(0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at bitreader/src/lib.rs:331:29:
attempt to calculate the remainder with a divisor of zero
backtrace:
  &lt;bitreader::BitReader&gt;::align (bitreader/src/lib.rs:331:29)
  repro_bitreader_src_lib_rs_331_29_f867fe::main (src/main.rs:6:24)</pre></td></tr>
<tr><td valign="top"><pre>let vec = vec![];
let bit_reader = BitReader::new(&amp;vec);
let u64_v = bit_reader.position();
let bit_reader_error = BitReaderError::NotEnoughData { position: 1u64,
    length: u64_v, requested: 0u64 };
let _ = bit_reader_error.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at bitreader/src/lib.rs:445:173:
attempt to subtract with overflow
backtrace:
  &lt;bitreader::BitReaderError as core::fmt::Display&gt;::fmt (bitreader/src/lib.rs:445:173)
  &lt;bitreader::BitReaderError as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;bitreader::BitReaderError as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_bitreader_src_lib_rs_445_173_a97263::main (src/main.rs:9:30)</pre></td></tr>
</table>
