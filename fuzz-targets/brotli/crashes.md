# brotli - crashes

Source: https://github.com/dropbox/rust-brotli @ 6518a4eff2f9b93a5e81555bdb10c2979b3057ef

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>BrotliConvertBitDepthsToSymbols(&amp;vec![], 1usize, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:555:34:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::entropy_encode::BrotliConvertBitDepthsToSymbols (rust-brotli/src/enc/entropy_encode.rs:555:34)
  repro_rust_brotli_src_enc_entropy_encode_rs_555_34_bffa66::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliWriteMetadataMetaBlock(&amp;BrotliEncoderParams::default(), &amp;mut 0usize,
    &amp;mut vec![84u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/brotli_bit_stream.rs:748:5:
index out of bounds: the len is 1 but the index is 7
backtrace:
  brotli::enc::brotli_bit_stream::BrotliWriteBits (rust-brotli/src/enc/brotli_bit_stream.rs:748:5)
  brotli::enc::brotli_bit_stream::BrotliWriteMetadataMetaBlock (rust-brotli/src/enc/brotli_bit_stream.rs:2874:5)
  repro_rust_brotli_src_enc_brotli_bit_stream_rs_748_5_ac9b61::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BroCatli::try_new_with_window_size(100u8).unwrap().stream(&amp;vec![],
    &amp;mut 1usize, &amp;mut vec![82u8], &amp;mut 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/concat/mod.rs:554:62:
attempt to subtract with overflow
backtrace:
  &lt;brotli::concat::BroCatli&gt;::stream (rust-brotli/src/concat/mod.rs:554:62)
  repro_rust_brotli_src_concat_mod_rs_554_62_15c9db::main (src/main.rs:4:64)</pre></td></tr>
<tr><td valign="top"><pre>let command = Command { insert_len_: 0u32, copy_len_: 0u32, dist_extra_: 0u32,
    cmd_prefix_: 0u16, dist_prefix_: 10000u16 };
let _ = command.distance_index_and_offset(
    &amp;BrotliDistanceParams { distance_postfix_bits: 1000000000u32,
    num_direct_distance_codes: 0u32, alphabet_size: 0u32,
    max_distance: 0usize });</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/command.rs:236:28:
attempt to shift left with overflow
backtrace:
  &lt;brotli::enc::command::Command&gt;::distance_index_and_offset (rust-brotli/src/enc/command.rs:236:28)
  repro_rust_brotli_src_enc_command_rs_236_28_5831a9::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::backward_references::H6Sub as brotli::enc::
    backward_references::AdvHashSpecialization&gt;::load_and_mix_word(
    &amp;H6Sub { hash_mask: 0u64, hash_shift_: 0i32, bucket_size_: 0u32,
    block_mask_: 0u32, block_bits_: 0i32 }, &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/static_dict.rs:46:31:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at (core/src/slice/mod.rs:1964:21)
  brotli::enc::static_dict::BROTLI_UNALIGNED_LOAD64 (rust-brotli/src/enc/static_dict.rs:46:31)
  &lt;brotli::enc::backward_references::H6Sub as brotli::enc::backward_references::AdvHashSpecialization&gt;::load_and_mix_word (rust-brotli/src/enc/backward_references/mod.rs:1139:10)
  repro_rust_brotli_src_enc_static_dict_rs_46_31_894491::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>BrotliZopfliCreateCommands(0usize, 0usize, 0usize, &amp;[ZopfliNode::default()],
    &amp;mut vec![], &amp;mut 0usize,
    &amp;BrotliEncoderParams { dist: BrotliDistanceParams { distance_postfix_bits: 0u32,
    num_direct_distance_codes: 0u32, alphabet_size: 0u32,
    max_distance: 1usize }, mode: BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR,
    quality: 0i32, q9_5: false, lgwin: 0i32, lgblock: 0i32, size_hint: 0usize,
    disable_literal_context_modeling: 0i32,
    hasher: BrotliHasherParams { type_: 0i32, bucket_bits: 0i32,
    block_bits: 0i32, hash_len: 0i32, num_last_distances_to_check: 0i32,
    literal_byte_score: 0i32 }, log_meta_block: false,
    stride_detection_quality: 0u8, high_entropy_detection_quality: 0u8,
    cdf_adaptation_detection: 1u8, prior_bitmask_detection: 0u8,
    literal_adaptation: [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (1u16,
    0u16)], large_window: false, avoid_distance_prefix_search: false,
    byte_align: false, bare_stream: false, catable: false,
    use_dictionary: false, appendable: false, magic_number: false,
    favor_cpu_efficiency: false }, &amp;mut vec![], &amp;mut 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/hq.rs:136:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::backward_references (rust-brotli/src/enc/backward_references/hq.rs:136:17)
  repro_rust_brotli_src_enc_backward_references_hq_rs_136_17_7d23c0::main (src/main.rs:9:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::prior_eval::SlowCMPrior as brotli::enc::prior_eval::
    Prior&gt;::lookup_lin(0u8, 0u8, 6023330749496444399usize, None);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/prior_eval.rs:298:13:
attempt to multiply with overflow
backtrace:
  &lt;brotli::enc::prior_eval::SlowCMPrior as brotli::enc::prior_eval::Prior&gt;::lookup_lin (rust-brotli/src/enc/prior_eval.rs:298:13)
  repro_rust_brotli_src_enc_prior_eval_rs_298_13_610a76::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut brotli_encoder_params = BrotliEncoderParams::default();
BrotliInitDistanceParams(&amp;mut brotli_encoder_params, 0u32, u32::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/encode.rs:309:5:
attempt to add with overflow
backtrace:
  brotli::enc::encode::BROTLI_DISTANCE_ALPHABET_SIZE (rust-brotli/src/enc/encode.rs:309:5)
  brotli::enc::metablock::BrotliInitDistanceParams (rust-brotli/src/enc/metablock.rs:36:21)
  repro_rust_brotli_src_enc_encode_rs_309_5_d4e95a::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliOptimizeHuffmanCountsForRle(usize::MAX, &amp;mut vec![2u32, 0u32, 65535u32],
    &amp;mut include_bytes!("image_0.tiff").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:222:12:
index out of bounds: the len is 3 but the index is 3
backtrace:
  brotli::enc::entropy_encode::BrotliOptimizeHuffmanCountsForRle (rust-brotli/src/enc/entropy_encode.rs:222:12)
  repro_rust_brotli_src_enc_entropy_encode_rs_222_12_fbbfe7::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior&gt;::
    lookup_lin(0u8, 0u8, usize::MAX, Some(15u8));</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/prior_eval.rs:253:37:
attempt to multiply with overflow
backtrace:
  &lt;brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior&gt;::lookup_lin (rust-brotli/src/enc/prior_eval.rs:253:37)
  repro_rust_brotli_src_enc_prior_eval_rs_253_37_b2b4b1::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let slice_offset = SliceOffset(0usize, 1u32);
let vec = vec![];
let _ = &lt;brotli::SliceOffset as brotli::interface::Unfreezable&gt;::thaw(
    &amp;slice_offset, &amp;vec);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/interface.rs:584:43:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at (core/src/slice/mod.rs:1964:21)
  &lt;brotli::enc::interface::SliceOffset as brotli::enc::interface::Unfreezable&gt;::thaw (rust-brotli/src/enc/interface.rs:584:43)
  repro_rust_brotli_src_enc_interface_rs_584_43_f96e81::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::UnionHasher&lt;brotli::enc::StandardAlloc&gt; as brotli::enc::
    backward_references::AnyHasher&gt;::HashTypeLength(
    &amp;&lt;brotli::enc::UnionHasher&lt;brotli::enc::StandardAlloc&gt; as ::core::default::
    Default&gt;::default());</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/mod.rs:2231:9:
UNINTIALIZED
backtrace:
  &lt;brotli::enc::backward_references::UnionHasher&lt;alloc_stdlib::std_alloc::StandardAlloc&gt; as brotli::enc::backward_references::AnyHasher&gt;::HashTypeLength (rust-brotli/src/enc/backward_references/mod.rs:2210:30)
  repro_rust_brotli_src_enc_backward_references_mod_rs_2231_9_c3fbbc::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut brotli_encoder_params = BrotliEncoderInitParams();
BrotliInitDistanceParams(&amp;mut brotli_encoder_params, 10u32, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/metablock.rs:38:19:
attempt to shift left with overflow
backtrace:
  brotli::enc::metablock::BrotliInitDistanceParams (rust-brotli/src/enc/metablock.rs:38:19)
  repro_rust_brotli_src_enc_metablock_rs_38_19_3f0c68::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>RecomputeDistancePrefixes(&amp;mut vec![], 1usize, 0u32, 1u32,
    &amp;BrotliDistanceParams { distance_postfix_bits: 1u32,
    num_direct_distance_codes: 0u32, alphabet_size: 0u32,
    max_distance: 1usize });</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/command.rs:259:38:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::command::RecomputeDistancePrefixes (rust-brotli/src/enc/command.rs:259:38)
  repro_rust_brotli_src_enc_command_rs_259_38_9c4fc3::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior&gt;::
    lookup_lin(0u8, 0u8, usize::MAX, None);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/prior_eval.rs:255:13:
attempt to multiply with overflow
backtrace:
  &lt;brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior&gt;::lookup_lin (rust-brotli/src/enc/prior_eval.rs:255:13)
  repro_rust_brotli_src_enc_prior_eval_rs_255_13_ffda31::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>BrotliEstimateBitCostsForLiterals(0usize, 1usize, 0usize, &amp;vec![35u8],
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/literal_cost.rs:171:17:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::literal_cost::EstimateBitCostsForLiteralsUTF8 (rust-brotli/src/enc/literal_cost.rs:171:17)
  brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals (rust-brotli/src/enc/literal_cost.rs:186:9)
  repro_rust_brotli_src_enc_literal_cost_rs_171_17_3df385::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FindMatchLengthWithLimitMin4(&amp;vec![0u8, 0u8, 0u8, 0u8, 0u8], &amp;vec![],
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/static_dict.rs:136:34:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at (core/src/slice/mod.rs:1964:21)
  brotli::enc::static_dict::FindMatchLengthWithLimitMin4 (rust-brotli/src/enc/static_dict.rs:136:34)
  repro_rust_brotli_src_enc_static_dict_rs_136_34_603f17::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>BROTLI_UNALIGNED_STORE64(&amp;mut vec![], 0u64);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/static_dict.rs:68:10:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at_mut (core/src/slice/mod.rs:1998:21)
  brotli::enc::static_dict::BROTLI_UNALIGNED_STORE64 (rust-brotli/src/enc/static_dict.rs:68:10)
  repro_rust_brotli_src_enc_static_dict_rs_68_10_ec8aeb::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let input_reference_mut = &lt;brotli::SliceOffset as brotli::interface::
    Unfreezable&gt;::thaw_mut(&amp;SliceOffset(1usize, 0u32), &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/interface.rs:590:24:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at_mut (core/src/slice/mod.rs:1998:21)
  &lt;brotli::enc::interface::SliceOffset as brotli::enc::interface::Unfreezable&gt;::thaw_mut (rust-brotli/src/enc/interface.rs:590:24)
  repro_rust_brotli_src_enc_interface_rs_590_24_172dfc::main (src/main.rs:4:31)</pre></td></tr>
<tr><td valign="top"><pre>BrotliConvertBitDepthsToSymbols(&amp;vec![35u8], 1usize, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:555:25:
index out of bounds: the len is 16 but the index is 35
backtrace:
  brotli::enc::entropy_encode::BrotliConvertBitDepthsToSymbols (rust-brotli/src/enc/entropy_encode.rs:555:25)
  repro_rust_brotli_src_enc_entropy_encode_rs_555_25_bf7d10::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut brotli_encoder_params = BrotliEncoderInitParams();
BrotliInitDistanceParams(&amp;mut brotli_encoder_params, u32::MAX, 0u32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/encode.rs:309:66:
attempt to add with overflow
backtrace:
  brotli::enc::encode::BROTLI_DISTANCE_ALPHABET_SIZE (rust-brotli/src/enc/encode.rs:309:66)
  brotli::enc::metablock::BrotliInitDistanceParams (rust-brotli/src/enc/metablock.rs:36:21)
  repro_rust_brotli_src_enc_encode_rs_309_66_ad3979::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut union_hasher = &lt;brotli::enc::UnionHasher&lt;brotli::ffi::alloc_util::
    BrotliSubclassableAllocator&gt; as ::core::default::Default&gt;::default();
&lt;brotli::enc::UnionHasher&lt;brotli::ffi::alloc_util::
    BrotliSubclassableAllocator&gt; as brotli::enc::backward_references::
    AnyHasher&gt;::StitchToPreviousBlock(&amp;mut union_hasher, 0usize, 0usize,
    &amp;vec![], 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/mod.rs:2246:9:
UNINTIALIZED
backtrace:
  &lt;brotli::enc::backward_references::UnionHasher&lt;brotli::ffi::alloc_util::BrotliSubclassableAllocator&gt; as brotli::enc::backward_references::AnyHasher&gt;::StitchToPreviousBlock (rust-brotli/src/enc/backward_references/mod.rs:2193:34)
  repro_rust_brotli_src_enc_backward_references_mod_rs_2246_9_029314::main (src/main.rs:3:5)</pre></td></tr>
<tr><td valign="top"><pre>RecomputeDistancePrefixes(&amp;mut vec![Command { insert_len_: 0u32,
    copy_len_: 1u32, dist_extra_: 0u32, cmd_prefix_: 1000u16,
    dist_prefix_: u16::MAX }], 1usize, 0u32, 1u32,
    &amp;BrotliDistanceParams { distance_postfix_bits: 0u32,
    num_direct_distance_codes: 0u32, alphabet_size: 0u32,
    max_distance: 0usize });</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/command.rs:194:26:
attempt to shift left with overflow
backtrace:
  &lt;brotli::enc::command::Command&gt;::restore_distance_code (rust-brotli/src/enc/command.rs:194:26)
  brotli::enc::command::RecomputeDistancePrefixes (rust-brotli/src/enc/command.rs:262:21)
  repro_rust_brotli_src_enc_command_rs_194_26_80f6f8::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliStoreHuffmanTree(&amp;vec![], 0usize, &amp;mut vec![HuffmanTree::default(),
    HuffmanTree::new(0u32, 0i16, 0i16)], &amp;mut 0usize, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:194:25:
index out of bounds: the len is 2 but the index is 2
backtrace:
  brotli::enc::entropy_encode::BrotliCreateHuffmanTree (rust-brotli/src/enc/entropy_encode.rs:194:25)
  brotli::enc::brotli_bit_stream::BrotliStoreHuffmanTree (rust-brotli/src/enc/brotli_bit_stream.rs:881:5)
  repro_rust_brotli_src_enc_entropy_encode_rs_194_25_1abe5b::main (src/main.rs:5:5)</pre></td></tr>
<tr><td valign="top"><pre>let slice_offset = SliceOffset(1usize, 0u32);
let vec = vec![];
let _ = &lt;brotli::SliceOffset as brotli::interface::Unfreezable&gt;::thaw(
    &amp;slice_offset, &amp;vec);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/interface.rs:584:24:
mid &gt; len
backtrace:
  &lt;[u8]&gt;::split_at (core/src/slice/mod.rs:1964:21)
  &lt;brotli::enc::interface::SliceOffset as brotli::enc::interface::Unfreezable&gt;::thaw (rust-brotli/src/enc/interface.rs:584:24)
  repro_rust_brotli_src_enc_interface_rs_584_24_37d6d8::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>BrotliWriteHuffmanTree(&amp;vec![35u8], 1usize, &amp;mut 0usize, &amp;mut vec![],
    &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:399:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::entropy_encode::BrotliWriteHuffmanTreeRepetitions (rust-brotli/src/enc/entropy_encode.rs:399:9)
  brotli::enc::entropy_encode::BrotliWriteHuffmanTree (rust-brotli/src/enc/entropy_encode.rs:513:13)
  repro_rust_brotli_src_enc_entropy_encode_rs_399_9_3eb0a8::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliCreateHuffmanTree(&amp;[0u32, 1u32], 2usize, 0i32, &amp;mut vec![], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:153:21:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::entropy_encode::BrotliCreateHuffmanTree (rust-brotli/src/enc/entropy_encode.rs:153:21)
  repro_rust_brotli_src_enc_entropy_encode_rs_153_21_2679f3::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliWriteBits(0usize, 0u64, &amp;mut 1000000000000000000usize, &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/compress_fragment_two_pass.rs:409:23:
range start index 125000000000000000 out of range for slice of length 0
backtrace:
  &lt;[u8] as core::ops::index::IndexMut&lt;core::ops::range::RangeFrom&lt;usize&gt;&gt;&gt;::index_mut (core/src/slice/index.rs:32:15)
  brotli::enc::compress_fragment_two_pass::BrotliWriteBits (rust-brotli/src/enc/compress_fragment_two_pass.rs:409:23)
  repro_rust_brotli_src_enc_compress_fragment_two_pass_rs_409_23_7e25a6::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let _ = BrotliSetDepth(0i32, &amp;mut vec![HuffmanTree::default()], &amp;mut vec![],
    1000000000i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/entropy_encode.rs:38:13:
index out of bounds: the len is 16 but the index is 16
backtrace:
  brotli::enc::entropy_encode::BrotliSetDepth (rust-brotli/src/enc/entropy_encode.rs:38:13)
  repro_rust_brotli_src_enc_entropy_encode_rs_38_13_674d30::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Command::new(&amp;BrotliDistanceParams { distance_postfix_bits: 0u32,
    num_direct_distance_codes: 0u32, alphabet_size: 0u32,
    max_distance: 0usize }, 0usize, 0usize, 1usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/command.rs:118:53:
attempt to shift right with overflow
backtrace:
  brotli::enc::command::combine_length_codes (rust-brotli/src/enc/command.rs:118:53)
  brotli::enc::command::get_length_code (rust-brotli/src/enc/command.rs:132:13)
  &lt;brotli::enc::command::Command&gt;::init (rust-brotli/src/enc/command.rs:291:9)
  &lt;brotli::enc::command::Command&gt;::new (rust-brotli/src/enc/command.rs:313:13)
  repro_rust_brotli_src_enc_command_rs_118_53_9a49b8::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut union_hasher = &lt;brotli::enc::UnionHasher&lt;brotli::enc::
    StandardAlloc&gt; as ::core::default::Default&gt;::default();
let _ = &lt;brotli::enc::UnionHasher&lt;brotli::enc::StandardAlloc&gt; as brotli::enc::
    backward_references::AnyHasher&gt;::GetHasherCommon(&amp;mut union_hasher);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/mod.rs:2219:9:
UNINTIALIZED
backtrace:
  &lt;brotli::enc::backward_references::UnionHasher&lt;alloc_stdlib::std_alloc::StandardAlloc&gt; as brotli::enc::backward_references::AnyHasher&gt;::GetHasherCommon (rust-brotli/src/enc/backward_references/mod.rs:2193:34)
  repro_rust_brotli_src_enc_backward_references_mod_rs_2219_9_239a3c::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = BROTLI_NUM_LITERAL_SYMBOLS;
BrotliInitZopfliNodes(&amp;mut vec![], usize_v);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/hq.rs:63:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::backward_references (rust-brotli/src/enc/backward_references/hq.rs:63:9)
  repro_rust_brotli_src_enc_backward_references_hq_rs_63_9_b95556::main (src/main.rs:6:5)</pre></td></tr>
<tr><td valign="top"><pre>BrotliEstimateBitCostsForLiterals(0usize, 1usize, 1usize, &amp;vec![], &amp;mut vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/utf8_util.rs:4:9:
index out of bounds: the len is 0 but the index is 0
backtrace:
  brotli::enc::utf8_util::parse_as_utf8 (rust-brotli/src/enc/utf8_util.rs:4:9)
  brotli::enc::utf8_util::is_mostly_utf8 (rust-brotli/src/enc/utf8_util.rs:55:36)
  brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals (rust-brotli/src/enc/literal_cost.rs:185:8)
  repro_rust_brotli_src_enc_utf8_util_rs_4_9_9dac27::main (src/main.rs:4:5)</pre></td></tr>
<tr><td valign="top"><pre>let mut bro_catli = BroCatli::try_new_with_window_size(100u8).unwrap();
let _ = bro_catli.finish(&amp;mut vec![], &amp;mut 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/concat/mod.rs:618:13:
index out of bounds: the len is 0 but the index is 1
backtrace:
  &lt;brotli::concat::BroCatli&gt;::finish (rust-brotli/src/concat/mod.rs:618:13)
  repro_rust_brotli_src_concat_mod_rs_618_13_1bf5b8::main (src/main.rs:5:23)</pre></td></tr>
<tr><td valign="top"><pre>let command = Command { insert_len_: 0u32, copy_len_: 0u32, dist_extra_: 0u32,
    cmd_prefix_: 0u16, dist_prefix_: 100u16 };
let _ = command.distance_index_and_offset(
    &amp;BrotliDistanceParams { distance_postfix_bits: 0u32,
    num_direct_distance_codes: 1u32, alphabet_size: 0u32,
    max_distance: 0usize });</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/command.rs:240:22:
attempt to subtract with overflow
backtrace:
  &lt;brotli::enc::command::Command&gt;::distance_index_and_offset (rust-brotli/src/enc/command.rs:240:22)
  repro_rust_brotli_src_enc_command_rs_240_22_79c227::main (src/main.rs:6:21)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FastLog2u32(-1000000000i32);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/util.rs:71:26:
index out of bounds: the len is 256 but the index is 18446744073709551556
backtrace:
  brotli::enc::util::FastLog2u32 (rust-brotli/src/enc/util.rs:71:26)
  repro_rust_brotli_src_enc_util_rs_71_26_dabdb3::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;brotli::enc::prior_eval::FastCMPrior as brotli::enc::prior_eval::
    Prior&gt;::lookup_lin(0u8, 0u8, 10000000000000000000usize, Some(255u8));</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/prior_eval.rs:274:13:
attempt to multiply with overflow
backtrace:
  &lt;brotli::enc::prior_eval::FastCMPrior as brotli::enc::prior_eval::Prior&gt;::lookup_lin (rust-brotli/src/enc/prior_eval.rs:274:13)
  repro_rust_brotli_src_enc_prior_eval_rs_274_13_43ae20::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let mut union_hasher = &lt;brotli::enc::UnionHasher&lt;brotli::ffi::alloc_util::
    BrotliSubclassableAllocator&gt; as ::core::default::Default&gt;::default();
&lt;brotli::enc::UnionHasher&lt;brotli::ffi::alloc_util::
    BrotliSubclassableAllocator&gt; as brotli::enc::backward_references::
    AnyHasher&gt;::StoreEvenVec4(&amp;mut union_hasher, &amp;vec![], 0usize, 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at rust-brotli/src/enc/backward_references/mod.rs:2288:9:
UNINTIALIZED
backtrace:
  &lt;brotli::enc::backward_references::UnionHasher&lt;brotli::ffi::alloc_util::BrotliSubclassableAllocator&gt; as brotli::enc::backward_references::AnyHasher&gt;::Store (rust-brotli/src/enc/backward_references/mod.rs:2193:34)
  &lt;brotli::enc::backward_references::UnionHasher&lt;brotli::ffi::alloc_util::BrotliSubclassableAllocator&gt; as brotli::enc::backward_references::AnyHasher&gt;::StoreEvenVec4 (rust-brotli/src/enc/backward_references/mod.rs:195:18)
  repro_rust_brotli_src_enc_backward_references_mod_rs_2288_9_4ab7f8::main (src/main.rs:3:5)</pre></td></tr>
</table>
