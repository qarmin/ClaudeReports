# miniz_oxide - crashes

Source: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;BlockBoundaryState { num_bits: 100u8, bit_buf: 0u8, z_header0: 0u32,
    z_header1: 0u32, check_adler32: 0u32 });
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide, &amp;vec![],
    &amp;mut vec![], 0usize, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:2080:18:
attempt to shift left with overflow
backtrace:
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:2080:18)
  miniz_oxide::inflate::core::decompress (miniz_oxide/src/inflate/core.rs:1420:5)
  repro_miniz_oxide_src_inflate_core_rs_2080_18_dd52bd::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let block_boundary_state = BlockBoundaryState { num_bits: 100u8, bit_buf: 99u8,
    z_header0: 0u32, z_header1: 0u32, check_adler32: 0u32 };
let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;block_boundary_state);
let _ = miniz_oxide::inflate::core::decompress_with_limit(
    &amp;mut decompressor_oxide, &amp;vec![], &amp;mut vec![73u8], 0usize, 1usize, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1989:34:
attempt to shift left with overflow
backtrace:
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1989:34)
  repro_miniz_oxide_src_inflate_core_rs_1989_34_cbffcb::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;BlockBoundaryState { num_bits: 100u8, bit_buf: 0u8, z_header0: 1u32,
    z_header1: 0u32, check_adler32: 1u32 });
let _ = decompressor_oxide.block_boundary_state();</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:370:13:
assertion failed: self.num_bits &lt; 8
backtrace:
  &lt;miniz_oxide::inflate::core::DecompressorOxide&gt;::block_boundary_state (miniz_oxide/src/inflate/core.rs:370:13)
  repro_miniz_oxide_src_inflate_core_rs_370_13_a7ad77::main (src/main.rs:6:32)</pre></td></tr>
<tr><td valign="top"><pre>let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;BlockBoundaryState { num_bits: 64u8, bit_buf: 243u8, z_header0: 0u32,
    z_header1: 0u32, check_adler32: 0u32 });
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide, &amp;vec![],
    &amp;mut vec![35u8], 0usize, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1990:21:
assertion `left == right` failed
  left: 40
 right: 0
backtrace:
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1990:21)
  miniz_oxide::inflate::core::decompress (miniz_oxide/src/inflate/core.rs:1420:5)
  repro_miniz_oxide_src_inflate_core_rs_1990_21_ec19b1::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut compressor_oxide = CompressorOxide::with_params(
    CompressorOxide::with_params(DataFormat::Raw, 0u8,
    CompressionStrategy::Filtered, 0u8).data_format(), 1u8,
    CompressionStrategy::Filtered, 0u8);
let _ = miniz_oxide::deflate::stream::deflate(&amp;mut compressor_oxide,
    &amp;include_bytes!("data_0.bin").to_vec(), &amp;mut vec![0u8], MZFlush::Block);
compressor_oxide.set_compression_level(CompressionLevel::UberCompression);
let _ = miniz_oxide::deflate::core::compress(&amp;mut compressor_oxide, &amp;vec![66u8],
    &amp;mut vec![], TDEFLFlush::None);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/deflate/core.rs:1985:64:
attempt to subtract with overflow
backtrace:
  miniz_oxide::deflate::core::compress_normal (miniz_oxide/src/deflate/core.rs:1985:64)
  miniz_oxide::deflate::core::compress_inner (miniz_oxide/src/deflate/core.rs:2483:9)
  miniz_oxide::deflate::core::compress (miniz_oxide/src/deflate/core.rs:2414:5)
  repro_miniz_oxide_src_deflate_core_rs_1985_64_7cacb5::main (src/main.rs:12:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;BlockBoundaryState::default());
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide,
    &amp;vec![35u8, 32u8, 84u8], &amp;mut vec![0u8, 68u8, 78u8, 0u8, 66u8, 0u8, 126u8,
    0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 0u8, 0u8, 75u8, 0u8, 0u8,
    0u8, 104u8, 76u8, 0u8, 113u8, 0u8, 60u8, 99u8, 82u8, 42u8, 0u8], 3usize,
    1u32);
let _ = miniz_oxide::inflate::core::decompress_with_limit(
    &amp;mut decompressor_oxide, &amp;vec![], &amp;mut vec![82u8, 73u8], 0usize,
    1000000000usize, 6u32);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1151:13:
assertion failed: (source_pos + 1) &amp; out_buf_size_mask &lt; out_slice.len()
backtrace:
  miniz_oxide::inflate::core::transfer (miniz_oxide/src/inflate/core.rs:1151:13)
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1966:21)
  repro_miniz_oxide_src_inflate_core_rs_1151_13_22cc58::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut compressor_oxide = CompressorOxide::default();
let _ = miniz_oxide::deflate::stream::deflate(&amp;mut compressor_oxide, &amp;vec![0u8,
    0u8, 0u8, 0u8], &amp;mut vec![0u8], MZFlush::Block);
compressor_oxide.set_format_and_level(CompressorOxide::with_format_and_level(
    DataFormat::Zlib, CompressionLevel::BestSpeed).data_format(), 1u8);
let _ = miniz_oxide::deflate::core::compress(&amp;mut compressor_oxide, &amp;vec![],
    &amp;mut vec![], TDEFLFlush::NoSync);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/deflate/core.rs:2220:38:
attempt to subtract with overflow
backtrace:
  miniz_oxide::deflate::core::compress_fast (miniz_oxide/src/deflate/core.rs:2220:38)
  miniz_oxide::deflate::core::compress_inner (miniz_oxide/src/deflate/core.rs:2481:9)
  miniz_oxide::deflate::core::compress (miniz_oxide/src/deflate/core.rs:2414:5)
  repro_miniz_oxide_src_deflate_core_rs_2220_38_6502b4::main (src/main.rs:11:13)</pre></td></tr>
<tr><td valign="top"><pre>let u32_v = TINFL_FLAG_PARSE_ZLIB_HEADER;
let block_boundary_state = BlockBoundaryState::default();
let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;block_boundary_state);
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide,
    &amp;vec![35u8, 32u8, 84u8], &amp;mut vec![0u8, 0u8, 0u8, 0u8, 91u8, 76u8, 0u8, 0u8,
    42u8, 39u8, 0u8, 0u8, 98u8, 51u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 49u8, 0u8,
    0u8, 0u8, 61u8, 39u8, 33u8, 0u8, 0u8, 111u8, 0u8, 69u8], 2usize, u32_v);
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide, &amp;vec![],
    &amp;mut vec![0u8], 0usize, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1148:35:
index out of bounds: the len is 1 but the index is 18446744073709551597
backtrace:
  miniz_oxide::inflate::core::transfer (miniz_oxide/src/inflate/core.rs:1148:35)
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1966:21)
  miniz_oxide::inflate::core::decompress (miniz_oxide/src/inflate/core.rs:1420:5)
  repro_miniz_oxide_src_inflate_core_rs_1148_35_bc206a::main (src/main.rs:10:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut decompressor_oxide = DecompressorOxide::new();
let _ = miniz_oxide::inflate::core::decompress_with_limit(
    &amp;mut decompressor_oxide, &amp;vec![82u8, 73u8, 70u8, 70u8], &amp;mut vec![0u8, 0u8,
    0u8, 0u8, 60u8, 0u8, 86u8, 123u8, 71u8, 66u8, 0u8, 0u8, 0u8, 33u8], 3usize,
    10000000000000000000usize, 2388820830u32);
let _ = miniz_oxide::inflate::core::decompress(&amp;mut decompressor_oxide,
    &amp;include_bytes!("image_0.bmp").to_vec(),
    &amp;mut include_bytes!("image_1.jpg").to_vec(), 2usize, 968484430u32);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1157:21:
attempt to add with overflow
backtrace:
  miniz_oxide::inflate::core::transfer (miniz_oxide/src/inflate/core.rs:1157:21)
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1966:21)
  miniz_oxide::inflate::core::decompress (miniz_oxide/src/inflate/core.rs:1420:5)
  repro_miniz_oxide_src_inflate_core_rs_1157_21_764974::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(
    &amp;BlockBoundaryState::default());
let _ = { let r = &amp;mut decompressor_oxide; let in_buf = &amp;vec![82u8, 73u8, 70u8,
    70u8]; let out = &amp;mut vec![0u8, 0u8, 0u8,
    0u8]; let out_pos = 0usize; let flags = 0u32; miniz_oxide::inflate::core::
    decompress(r, in_buf, out, out_pos, flags).0 };
let _ = miniz_oxide::inflate::core::decompress_with_limit(
    &amp;mut decompressor_oxide, &amp;vec![], &amp;mut vec![35u8, 32u8, 84u8, 72u8, 73u8],
    1usize, 5usize, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at miniz_oxide/src/inflate/core.rs:1135:21:
attempt to add with overflow
backtrace:
  miniz_oxide::inflate::core::transfer (miniz_oxide/src/inflate/core.rs:1135:21)
  miniz_oxide::inflate::core::decompress_with_limit (miniz_oxide/src/inflate/core.rs:1966:21)
  repro_miniz_oxide_src_inflate_core_rs_1135_21_4d5252::main (src/main.rs:7:13)</pre></td></tr>
</table>
