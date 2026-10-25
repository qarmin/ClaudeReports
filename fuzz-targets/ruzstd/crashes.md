# ruzstd - crashes

Source: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(1u8);
let _ = f_s_e_table.build_from_probabilities(1u8, &amp;[1000i32]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/fse/fse_decoder.rs:327:5:
assertion failed: x &gt; 0
backtrace:
  ruzstd::fse::fse_decoder (ruzstd/src/fse/fse_decoder.rs:327:5)
  ruzstd::fse::fse_decoder::calc_baseline_and_numbits (ruzstd/src/fse/fse_decoder.rs:357:20)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:209:28)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_327_5_d1a813::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let _ = HuffmanTable::build_from_weights(&amp;[1usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/huff0/huff0_encoder.rs:244:36:
attempt to subtract with overflow
backtrace:
  &lt;ruzstd (ruzstd/src/huff0/huff0_encoder.rs:244:36)
  repro_ruzstd_src_huff0_huff0_encoder_rs_244_36_49f1ff::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = HuffmanTable::build_from_weights(&amp;[1000000000000000000usize]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/huff0/huff0_encoder.rs:228:48:
attempt to shift left with overflow
backtrace:
  &lt;ruzstd (ruzstd/src/huff0/huff0_encoder.rs:228:48)
  &lt;usize as core::iter::traits::accum::Sum&gt;::sum::&lt;core::iter::adapters::map::Map&lt;core::slice::iter::Iter&lt;&lt;ruzstd (core/src/iter/traits/accum.rs:52:22)
  &lt;ruzstd (ruzstd/src/huff0/huff0_encoder.rs:228:69)
  repro_ruzstd_src_huff0_huff0_encoder_rs_228_48_900b59::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = HuffmanTable::build_from_weights(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/huff0/huff0_encoder.rs:230:13:
This is an internal error
backtrace:
  &lt;ruzstd (ruzstd/src/huff0/huff0_encoder.rs:230:13)
  repro_ruzstd_src_huff0_huff0_encoder_rs_230_13_642d3c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = HuffmanTable::build_from_counts(&amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/huff0/huff0_encoder.rs:296:5:
assertion failed: amount &gt;= 2
backtrace:
  ruzstd (ruzstd/src/huff0/huff0_encoder.rs:296:5)
  &lt;ruzstd (ruzstd/src/huff0/huff0_encoder.rs:176:27)
  repro_ruzstd_src_huff0_huff0_encoder_rs_296_5_b7f1a5::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let huffman_table = HuffmanTable::new();
let mut huffman_decoder = HuffmanDecoder::new(&amp;huffman_table);
let _ = huffman_decoder.decode_symbol();</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/huff0/huff0_decoder.rs:26:26:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[ruzstd (core/src/slice/index.rs:238:10)
  &lt;[ruzstd (core/src/slice/index.rs:19:15)
  &lt;ruzstd (ruzstd/src/huff0/huff0_decoder.rs:26:26)
  repro_ruzstd_src_huff0_huff0_decoder_rs_26_26_1630f8::main (src/main.rs:7:29)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(1u8, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/fse/fse_decoder.rs:206:49:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[i32]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[i32] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:206:49)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_206_49_7bc41b::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(100u8, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/fse/fse_decoder.rs:150:26:
attempt to shift left with overflow
backtrace:
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:150:26)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_150_26_207a8d::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(30u8, &amp;[]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8589934592 bytes failed
backtrace:
  ruzstd::fse::fse_decoder::FSETable::build_decoding_table (src/fse/fse_decoder.rs:152)
  ruzstd::fse::fse_decoder::FSETable::build_from_probabilities (src/fse/fse_decoder.rs:136)
  repro_out_of_memory_in_ruzstd_fse_fsetable_build_from_probabilitie_4cf8d9::main (src/main.rs:5)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(63u8, &amp;[]);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from ruzstd/src/fse/fse_decoder.rs:152:25)
capacity overflow
backtrace:
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:152:25)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_152_25_6809bd::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(21u8, &amp;[1i32]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/fse/fse_decoder.rs:364:10:
attempt to multiply with overflow
backtrace:
  ruzstd::fse::fse_decoder::calc_baseline_and_numbits (ruzstd/src/fse/fse_decoder.rs:364:10)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:209:28)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_364_10_963853::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(0u8);
let _ = f_s_e_table.build_from_probabilities(1u8, &amp;[-1000000000i32]);</pre></td><td valign="top"><pre>thread 'main' panicked at ruzstd/src/fse/fse_decoder.rs:351:9:
attempt to shift left with overflow
backtrace:
  ruzstd::fse::fse_decoder::calc_baseline_and_numbits (ruzstd/src/fse/fse_decoder.rs:351:9)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_decoding_table (ruzstd/src/fse/fse_decoder.rs:209:28)
  &lt;ruzstd::fse::fse_decoder::FSETable&gt;::build_from_probabilities (ruzstd/src/fse/fse_decoder.rs:136:14)
  repro_ruzstd_src_fse_fse_decoder_rs_351_9_e4cb31::main (src/main.rs:5:25)</pre></td></tr>
<tr><td valign="top"><pre>let mut f_s_e_table = FSETable::new(64u8);
let _ = f_s_e_table.build_from_probabilities(26u8, &amp;[7i32, 2041339626i32, 45i32,
    i32::MAX, 1i32, 1i32]);
let _ = HuffmanTable::build_from_weights(&amp;[3usize, 6180167714942147986usize,
    2usize]);
let _ = BlockSizeError::BlockSizeTooLarge { size: 0u32 };
let _ = DEFAULT_MAX_WINDOW_SIZE;
let _ = CompressionLevel::Default;
let _ = FSEDecoderError::TableIsUninitialized;</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  ruzstd::fse::fse_decoder::FSETable::build_decoding_table (src/fse/fse_decoder.rs:187)
  ruzstd::fse::fse_decoder::FSETable::build_from_probabilities (src/fse/fse_decoder.rs:136)
  repro_timeout_in_ruzstd_fse_fsetable_build_from_probabilities_751cb8::main (src/main.rs:10)
  (2 std/libc frames omitted)</pre></td></tr>
</table>
