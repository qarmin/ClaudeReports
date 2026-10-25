# tiff - crashes

Source: https://github.com/image-rs/image-tiff @ 3302936b08e2260d4c66f40f51170efbb1fded4c

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::I8(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:129:86:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[i8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:129:86)
  repro_image_tiff_src_decoder_buffer_rs_129_86_50219d::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::F32(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:127:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[f32] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:127:88)
  repro_image_tiff_src_decoder_buffer_rs_127_88_9cb223::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::F64(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:128:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[f64] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:128:88)
  repro_image_tiff_src_decoder_buffer_rs_128_88_10f9d9::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::I32(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:131:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[i32] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:131:88)
  repro_image_tiff_src_decoder_buffer_rs_131_88_803fe0::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::I64(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:132:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[i64] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:132:88)
  repro_image_tiff_src_decoder_buffer_rs_132_88_729306::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::U32(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:124:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u32] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:124:88)
  repro_image_tiff_src_decoder_buffer_rs_124_88_44f636::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::I16(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:130:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[i16] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:130:88)
  repro_image_tiff_src_decoder_buffer_rs_130_88_68d185::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::U8(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:122:86:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:122:86)
  repro_image_tiff_src_decoder_buffer_rs_122_86_9d5513::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::U64(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:125:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u64] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:125:88)
  repro_image_tiff_src_decoder_buffer_rs_125_88_0bc27b::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let mut decoding_sample_buffer = DecodingSampleBuffer::U16(vec![]);
let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at image-tiff/src/decoder/buffer.rs:123:88:
range start index 1 out of range for slice of length 0
backtrace:
  &lt;[u16] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  &lt;tiff::decoder::buffer::DecodingSampleBuffer&gt;::as_buffer (image-tiff/src/decoder/buffer.rs:123:88)
  repro_image_tiff_src_decoder_buffer_rs_123_88_fabca3::main (src/main.rs:5:56)</pre></td></tr>
<tr><td valign="top"><pre>let tiff_header = TiffHeader { byte_order: ByteOrder::BigEndian,
    variant: TiffVariant::BigTiff, first_ifd: IfdPointer(0u64) };
let decoder = tiff_header.open::&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;(
    std::io::Cursor::new(vec![]));
let decoder2 = decoder.with_limits(Limits::default());
let mut decoder3 = decoder2.with_limits(Limits::unlimited());
let _ = decoder3.read_string(100000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 100000000000000 bytes failed
backtrace:
  tiff::decoder::Decoder&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt;::read_string&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8, alloc::alloc::Global&gt;&gt;&gt; (image-tiff/src/decoder/mod.rs:1397)
  repro_out_of_memory_in_tiff_decoder_decoder_std_io_cursor_vec_u8_r_709f04::main (src/main.rs:12)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let tiff_header = TiffHeader { byte_order: ValueBuffer::empty(
    Type::BYTE).byte_order(), variant: TiffVariant::BigTiff,
    first_ifd: IfdPointer(0u64) };
let mut decoder = tiff_header.open::&lt;::std::io::Cursor&lt;Vec&lt;u8&gt;&gt;&gt;(
    std::io::Cursor::new(vec![]));
let _ = decoder.read_string(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:51:31)
capacity overflow
backtrace:
  &lt;u8 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:51:31)
  &lt;tiff::decoder::Decoder&lt;core::io::cursor::Cursor&lt;alloc::vec::Vec&lt;u8&gt;&gt;&gt;&gt;::read_string (image-tiff/src/decoder/mod.rs:1397:23)
  repro_image_tiff_src_decoder_mod_rs_1397_23_2cb5da::main (src/main.rs:10:21)</pre></td></tr>
</table>
