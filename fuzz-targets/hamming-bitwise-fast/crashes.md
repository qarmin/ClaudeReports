# hamming-bitwise-fast - crashes

Source: https://github.com/emschwartz/hamming-bitwise-fast @ 1ac2bc123d91ddad9dccb7e8a13f0f107046cd87

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = hamming_bitwise_fast::hamming_bitwise_fast(&amp;vec![], &amp;vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at hamming-bitwise-fast/src/slice.rs:42:5:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  hamming_bitwise_fast::slice::distance::distance_avx_avx2_avx512f_avx512vl_avx512vpopcntdq_f16c_fma_popcnt_sse_sse2_sse3_sse41_ssse3_version::__safe_inner (hamming-bitwise-fast/src/slice.rs:42:5)
  hamming_bitwise_fast::slice::distance::distance_avx_avx2_avx512f_avx512vl_avx512vpopcntdq_f16c_fma_popcnt_sse_sse2_sse3_sse41_ssse3_version (hamming-bitwise-fast/src/slice.rs:31:5)
  hamming_bitwise_fast::slice::distance::__resolver_fn (hamming-bitwise-fast/src/slice.rs:31:5)
  hamming_bitwise_fast::slice::distance (hamming-bitwise-fast/src/slice.rs:31:5)
  hamming_bitwise_fast (hamming-bitwise-fast/src/lib.rs:123:5)
  repro_hamming_bitwise_fast_src_slice_rs_42_5_86aa0f::main (src/main.rs:2:13)</pre></td></tr>
</table>
