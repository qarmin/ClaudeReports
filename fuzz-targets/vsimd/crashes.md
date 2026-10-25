# vsimd - crashes

Source: https://github.com/Nugine/simd @ dff9585c080896a9ebdb8d6dcabce17bd69a0874

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = vsimd::hex::decode_ascii32::&lt;vsimd::isa::SSE2&gt;(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/table.rs:20:5:
internal error: entered unreachable code
backtrace:
  vsimd::table::u8x16xn_lookup::&lt;vsimd::isa::SSE2, vsimd::vector::V256&gt; (vsimd/src/table.rs:20:5)
  vsimd::alsw::decode_ascii_xn::&lt;vsimd::isa::SSE2, vsimd::vector::V256&gt; (vsimd/src/alsw.rs:65:32)
  vsimd (vsimd/src/hex.rs:140:20)
  vsimd (vsimd/src/hex.rs:169:27)
  repro_vsimd_src_table_rs_20_5_c89f69::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::u16x8_blend::&lt;0&gt;(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::u32x4_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0u32),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD128&gt;::i64x2_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0i64));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:1143:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u16x8_blend::&lt;0&gt; (vsimd/src/simd128.rs:1143:13)
  repro_vsimd_src_simd128_rs_1143_13_680fed::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u64x4_bswap(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(),
    &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u8x32_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(), 0u8));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:696:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSSE3 as vsimd::simd128::SIMD128&gt;::u64x2_bswap (vsimd/src/simd128.rs:696:13)
  &lt;vsimd::isa::SSSE3 as vsimd::simd256::SIMD256&gt;::u64x4_bswap (vsimd/src/simd256.rs:35:18)
  repro_vsimd_src_simd128_rs_696_13_1b3417::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_e2 = vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap();
let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD256&gt;::u8x32_reduce_max(s_s_e2,
    ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:773:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u8x16_reduce_max (vsimd/src/simd128.rs:773:13)
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::u8x32_reduce_max (vsimd/src/simd256.rs:558:14)
  repro_vsimd_src_simd128_rs_773_13_58db8d::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD256&gt;::i32x8_mul_lo(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), V256::from_bytes([0u8,
    0u8 /* x15 */, 1u8, 0u8, 0u8, 1u8, 0u8 /* x11 */, 0u8]), ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:387:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::i32x4_mul_lo (vsimd/src/simd128.rs:387:13)
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::i32x8_mul_lo (vsimd/src/simd256.rs:43:18)
  repro_vsimd_src_simd128_rs_387_13_e15d37::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_e41 = vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap();
let s_s_e2 = vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap();
let s_s_s_e3 = vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap();
let _ = vsimd::hex::encode_bytes32::&lt;vsimd::isa::SSE2&gt;(s_s_e2,
    &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::v256_create_zero(s_s_s_e3),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::i32x8_splat(s_s_e41, 0i32));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:642:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u8x16_swizzle (vsimd/src/simd128.rs:642:13)
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::u8x16x2_swizzle (vsimd/src/simd256.rs:43:18)
  vsimd (vsimd/src/hex.rs:72:16)
  repro_vsimd_src_simd128_rs_642_13_c8ff48::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_e2 = vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap();
let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD256&gt;::u8x32_unzip_odd(s_s_e2,
    &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u8x32_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0u8),
    &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u32x8_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(), 0u32));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd256.rs:767:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::u8x32_unzip_odd (vsimd/src/simd256.rs:767:13)
  repro_vsimd_src_simd256_rs_767_13_ddff60::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_s_e3 = vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap();
let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u16x16_max(s_s_s_e3,
    ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/unified.rs:776:9:
internal error: entered unreachable code
backtrace:
  vsimd::unified::max::&lt;vsimd::isa::SSSE3, u16, vsimd::vector::V128&gt; (vsimd/src/unified.rs:776:9)
  vsimd::unified::max::&lt;vsimd::isa::SSSE3, u16, vsimd::vector::V256&gt; (vsimd/src/unified.rs:683:22)
  &lt;vsimd::isa::SSSE3 as vsimd::simd256::SIMD256&gt;::u16x16_max (vsimd/src/simd256.rs:410:9)
  repro_vsimd_src_unified_rs_776_9_2c60c3::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD128&gt;::u8x16_unzip_odd(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD128&gt;::i16x8_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0i16),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::u8x16_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0u8));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:1061:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd128::SIMD128&gt;::u8x16_unzip_odd (vsimd/src/simd128.rs:1061:13)
  repro_vsimd_src_simd128_rs_1061_13_3a5c7c::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::v128_all_zero(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i32x4_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i32));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:225:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::v128_all_zero (vsimd/src/simd128.rs:225:13)
  repro_vsimd_src_simd128_rs_225_13_99feee::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD128&gt;::u8x16_reduce_min(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i16x8_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i16));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:793:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSSE3 as vsimd::simd128::SIMD128&gt;::u8x16_reduce_min (vsimd/src/simd128.rs:793:13)
  repro_vsimd_src_simd128_rs_793_13_ab6721::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD256&gt;::u16x16_bswap(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u64x4_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0u64));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:660:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u16x8_bswap (vsimd/src/simd128.rs:660:13)
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::u16x16_bswap (vsimd/src/simd256.rs:35:18)
  repro_vsimd_src_simd128_rs_660_13_b4f364::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u8x32_reduce_min(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::i16x16_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0i16));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:781:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd128::SIMD128&gt;::u8x16_reduce_min (vsimd/src/simd128.rs:781:13)
  &lt;vsimd::isa::SSE41 as vsimd::simd256::SIMD256&gt;::u8x32_reduce_min (vsimd/src/simd256.rs:564:14)
  repro_vsimd_src_simd128_rs_781_13_c63992::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u32x8_blend::&lt;0&gt;(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), ENCODE_LOWER_LUT,
    ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd256.rs:851:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd256::SIMD256&gt;::u32x8_blend::&lt;0&gt; (vsimd/src/simd256.rs:851:13)
  repro_vsimd_src_simd256_rs_851_13_e42c96::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u8x32_reduce_max(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(),
    &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u64x4_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(), 0u64));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:761:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd128::SIMD128&gt;::u8x16_reduce_max (vsimd/src/simd128.rs:761:13)
  &lt;vsimd::isa::SSE41 as vsimd::simd256::SIMD256&gt;::u8x32_reduce_max (vsimd/src/simd256.rs:558:14)
  repro_vsimd_src_simd128_rs_761_13_1804db::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD128&gt;::u16x8_min(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i64x2_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i64),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i16x8_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i16));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/unified.rs:907:9:
internal error: entered unreachable code
backtrace:
  vsimd::unified::min::&lt;vsimd::isa::SSSE3, u16, vsimd::vector::V128&gt; (vsimd/src/unified.rs:907:9)
  &lt;vsimd::isa::SSSE3 as vsimd::simd128::SIMD128&gt;::u16x8_min (vsimd/src/simd128.rs:586:9)
  repro_vsimd_src_unified_rs_907_9_8c0dc2::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u64x4_permute::&lt;0&gt;(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD256&gt;::u64x4_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0u64));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd256.rs:743:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd256::SIMD256&gt;::u64x4_permute::&lt;0&gt; (vsimd/src/simd256.rs:743:13)
  repro_vsimd_src_simd256_rs_743_13_4e25e3::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE41 as vsimd::SIMD64&gt;::u8x8_unzip_even(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(),
    vsimd::hex::sse2::merge_bits(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(),
    &lt;vsimd::isa::SSE41 as vsimd::SIMD128&gt;::u64x2_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE41&gt;().unwrap(), 0u64)),
    V64::from_bytes([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd64.rs:32:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE41 as vsimd::simd64::SIMD64&gt;::u8x8_unzip_even (vsimd/src/simd64.rs:32:13)
  repro_vsimd_src_simd64_rs_32_13_40d022::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_s_e3 = vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap();
let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u8x32_swizzle(s_s_s_e3,
    ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd256.rs:508:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSSE3 as vsimd::simd256::SIMD256&gt;::u8x32_swizzle (vsimd/src/simd256.rs:508:13)
  repro_vsimd_src_simd256_rs_508_13_448696::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let s_s_s_e3 = vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap();
let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u32x8_bswap(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(),
    &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::i64x4_splat(s_s_s_e3, 0i64));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:678:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSSE3 as vsimd::simd128::SIMD128&gt;::u32x4_bswap (vsimd/src/simd128.rs:678:13)
  &lt;vsimd::isa::SSSE3 as vsimd::simd256::SIMD256&gt;::u32x8_bswap (vsimd/src/simd256.rs:35:18)
  repro_vsimd_src_simd128_rs_678_13_3b184e::main (src/main.rs:3:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSSE3 as vsimd::SIMD256&gt;::u8x32_unzip_even(
    vsimd::isa::detect::&lt;vsimd::isa::SSSE3&gt;().unwrap(), ENCODE_LOWER_LUT,
    ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd256.rs:754:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSSE3 as vsimd::simd256::SIMD256&gt;::u8x32_unzip_even (vsimd/src/simd256.rs:754:13)
  repro_vsimd_src_simd256_rs_754_13_3912fb::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD256&gt;::u8x32_blendv(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), ENCODE_LOWER_LUT,
    ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:1161:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u8x16_blendv (vsimd/src/simd128.rs:1161:13)
  &lt;vsimd::isa::SSE2 as vsimd::simd256::SIMD256&gt;::u8x32_blendv (vsimd/src/simd256.rs:52:18)
  repro_vsimd_src_simd128_rs_1161_13_898c9b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::u8x16_unzip_even(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), V128::from_bytes([0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8 /* x8 */, 0u8]),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::u16x8_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0u16));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:1034:13:
not implemented
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::u8x16_unzip_even (vsimd/src/simd128.rs:1034:13)
  repro_vsimd_src_simd128_rs_1034_13_a17bd8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i16x8_maddubs(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i64x2_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i64),
    &lt;vsimd::isa::SSE2 as vsimd::SIMD128&gt;::i8x16_splat(
    vsimd::isa::detect::&lt;vsimd::isa::SSE2&gt;().unwrap(), 0i8));</pre></td><td valign="top"><pre>thread 'main' panicked at vsimd/src/simd128.rs:1127:13:
internal error: entered unreachable code
backtrace:
  &lt;vsimd::isa::SSE2 as vsimd::simd128::SIMD128&gt;::i16x8_maddubs (vsimd/src/simd128.rs:1127:13)
  repro_vsimd_src_simd128_rs_1127_13_ac4620::main (src/main.rs:2:13)</pre></td></tr>
</table>
