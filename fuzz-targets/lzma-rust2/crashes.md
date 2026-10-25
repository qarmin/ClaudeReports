# lzma-rust2 - crashes

Source: https://github.com/hasenbanck/lzma-rust2 @ d6268a287ceb288114a02a5f29da28acaaaf9e52

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let encode_mode = EncodeMode::Fast;
let lzma_options = LzmaOptions { dict_size: 0u32, lc: 0u32, lp: 0u32,
    pb: 1000000000u32, mode: encode_mode, nice_len: 0u32, mf: MfType::Hc4,
    depth_limit: 0i32, preset_dict: None };
let _ = lzma_options.get_props();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/enc/lzma2_writer.rs:162:11:
attempt to multiply with overflow
backtrace:
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_props (lzma-rust2/src/enc/lzma2_writer.rs:162:11)
  repro_lzma_rust2_src_enc_lzma2_writer_rs_162_11_7d89fd::main (src/main.rs:8:26)</pre></td></tr>
<tr><td valign="top"><pre>let lzma_options = LzmaOptions { dict_size: 4294967294u32, lc: 0u32, lp: 0u32,
    pb: 0u32, mode: EncodeMode::Fast, nice_len: 1u32, mf: MfType::default(),
    depth_limit: 0i32, preset_dict: None };
let _ = lzma_options.get_memory_usage();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/lz/lz_encoder.rs:470:5:
attempt to add with overflow
backtrace:
  lzma_rust2::lz::lz_encoder::get_buf_size (lzma-rust2/src/lz/lz_encoder.rs:470:5)
  &lt;lzma_rust2::lz::lz_encoder::LzEncoder&gt;::get_memory_usage (lzma-rust2/src/lz/lz_encoder.rs:102:9)
  &lt;lzma_rust2::enc::encoder_fast::FastEncoderMode&gt;::get_memory_usage (lzma-rust2/src/enc/encoder_fast.rs:15:9)
  &lt;lzma_rust2::enc::encoder::LzmaEncoder&gt;::get_mem_usage (lzma-rust2/src/enc/encoder.rs:126:33)
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_memory_usage (lzma-rust2/src/enc/lzma2_writer.rs:148:9)
  repro_lzma_rust2_src_lz_lz_encoder_rs_470_5_f17026::main (src/main.rs:7:26)</pre></td></tr>
<tr><td valign="top"><pre>let u32_v = lzma_rust2::get_extra_size_before(1000000000u32);
let u32_v2 = Bcj2Options::RELATIVE_LIMIT_DEFAULT;
let lzma_options = LzmaOptions::new(u32_v, 0u32, 0u32, u32_v2,
    EncodeMode::Normal, 0u32, MfType::Hc4, 0i32);
let _ = lzma_options.get_memory_usage();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/lz/hash234.rs:23:21:
attempt to subtract with overflow
backtrace:
  &lt;lzma_rust2::lz (lzma-rust2/src/lz/hash234.rs:23:21)
  &lt;lzma_rust2::lz (lzma-rust2/src/lz/hash234.rs:37:36)
  &lt;lzma_rust2::lz (lzma-rust2/src/lz/hc4.rs:21:9)
  &lt;lzma_rust2::lz::lz_encoder::MfType&gt;::get_memory_usage (lzma-rust2/src/lz/lz_encoder.rs:51:28)
  &lt;lzma_rust2::lz::lz_encoder::LzEncoder&gt;::get_memory_usage (lzma-rust2/src/lz/lz_encoder.rs:109:18)
  &lt;lzma_rust2::enc::encoder_normal::NormalEncoderMode&gt;::get_memory_usage (lzma-rust2/src/enc/encoder_normal.rs:24:9)
  &lt;lzma_rust2::enc::encoder::LzmaEncoder&gt;::get_mem_usage (lzma-rust2/src/enc/encoder.rs:128:17)
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_memory_usage (lzma-rust2/src/enc/lzma2_writer.rs:148:9)
  repro_lzma_rust2_src_lz_hash234_rs_23_21_31a246::main (src/main.rs:10:26)</pre></td></tr>
<tr><td valign="top"><pre>let lzma_options = LzmaOptions { dict_size: u32::MAX, lc: 0u32, lp: 0u32,
    pb: 0u32, mode: EncodeMode::Fast, nice_len: 0u32, mf: MfType::Hc4,
    depth_limit: 0i32, preset_dict: None };
let _ = lzma_options.get_memory_usage();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/lz/lz_encoder.rs:467:28:
attempt to add with overflow
backtrace:
  lzma_rust2::lz::lz_encoder::get_buf_size (lzma-rust2/src/lz/lz_encoder.rs:467:28)
  &lt;lzma_rust2::lz::lz_encoder::LzEncoder&gt;::get_memory_usage (lzma-rust2/src/lz/lz_encoder.rs:102:9)
  &lt;lzma_rust2::enc::encoder_fast::FastEncoderMode&gt;::get_memory_usage (lzma-rust2/src/enc/encoder_fast.rs:15:9)
  &lt;lzma_rust2::enc::encoder::LzmaEncoder&gt;::get_mem_usage (lzma-rust2/src/enc/encoder.rs:126:33)
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_memory_usage (lzma-rust2/src/enc/lzma2_writer.rs:148:9)
  repro_lzma_rust2_src_lz_lz_encoder_rs_467_28_99396a::main (src/main.rs:7:26)</pre></td></tr>
<tr><td valign="top"><pre>let lzma_options = LzmaOptions { dict_size: 0u32, lc: u32::MAX, lp: 0u32,
    pb: 1u32, mode: EncodeMode::Normal, nice_len: 0u32, mf: MfType::Hc4,
    depth_limit: 0i32, preset_dict: None };
let _ = lzma_options.get_props();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/enc/lzma2_writer.rs:162:9:
attempt to add with overflow
backtrace:
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_props (lzma-rust2/src/enc/lzma2_writer.rs:162:9)
  repro_lzma_rust2_src_enc_lzma2_writer_rs_162_9_d5dcb6::main (src/main.rs:7:26)</pre></td></tr>
<tr><td valign="top"><pre>let mf_type = MfType::Bt4;
let lzma_options = LzmaOptions::new(1u32, 0u32, 1000000000u32, 0u32,
    EncodeMode::Fast, 0u32, mf_type, 0i32);
let _ = lzma_options.get_props();</pre></td><td valign="top"><pre>thread 'main' panicked at lzma-rust2/src/enc/lzma2_writer.rs:162:10:
attempt to multiply with overflow
backtrace:
  &lt;lzma_rust2::enc::lzma2_writer::LzmaOptions&gt;::get_props (lzma-rust2/src/enc/lzma2_writer.rs:162:10)
  repro_lzma_rust2_src_enc_lzma2_writer_rs_162_10_d2e95a::main (src/main.rs:8:26)</pre></td></tr>
</table>
