# fax - crashes

Source: https://github.com/pdf-rs/fax @ 8f26f4f63a54fb93529ffdedd9b5a584c3ed01af

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = VecWriter::with_capacity(usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at fax/src/lib.rs:100:38:
attempt to add with overflow
backtrace:
  &lt;fax::VecWriter&gt;::with_capacity (fax/src/lib.rs:100:38)
  repro_fax_src_lib_rs_100_38_0f67ed::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let bits = Bits { data: 0u16, len: 26u8 };
let _ = bits.to_string();</pre></td><td valign="top"><pre>thread 'main' panicked at fax/src/lib.rs:524:26:
attempt to shift left with overflow
backtrace:
  &lt;fax::Bits as core::fmt::Display&gt;::fmt (fax/src/lib.rs:524:26)
  &lt;fax::Bits as alloc::string::SpecToString&gt;::spec_to_string (alloc/src/string.rs:2952:9)
  &lt;fax::Bits as alloc::string::ToString&gt;::to_string (alloc/src/string.rs:2931:9)
  repro_fax_src_lib_rs_524_26_b5128e::main (src/main.rs:5:18)</pre></td></tr>
<tr><td valign="top"><pre>let _ = VecWriter::with_capacity(10000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1250000000000000000 bytes failed
backtrace:
  fax::VecWriter::with_capacity (src/lib.rs:100)
  repro_out_of_memory_in_fax_vecwriter_with_capacity_ed4937::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let bits = Bits { data: 0u16, len: 100u8 };
let mut vec_writer = VecWriter::new();
let _ = &lt;fax::VecWriter as fax::BitWriter&gt;::write(&amp;mut vec_writer, bits);</pre></td><td valign="top"><pre>thread 'main' panicked at fax/src/lib.rs:79:47:
attempt to subtract with overflow
backtrace:
  &lt;fax::VecWriter as fax::BitWriter&gt;::write (fax/src/lib.rs:79:47)
  repro_fax_src_lib_rs_79_47_34d345::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut vec_writer = VecWriter::new();
let _ = &lt;fax::VecWriter as fax::BitWriter&gt;::write(&amp;mut vec_writer,
    Bits { data: 0u16, len: 0u8 });</pre></td><td valign="top"><pre>thread 'main' panicked at fax/src/lib.rs:79:25:
attempt to shift left with overflow
backtrace:
  &lt;fax::VecWriter as fax::BitWriter&gt;::write (fax/src/lib.rs:79:25)
  repro_fax_src_lib_rs_79_25_48c924::main (src/main.rs:6:13)</pre></td></tr>
</table>
