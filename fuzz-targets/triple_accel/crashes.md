# triple_accel - crashes

Source: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = triple_accel::alloc_str(10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:24:31)
capacity overflow
backtrace:
  &lt;u128 as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:24:31)
  triple_accel::alloc_str (triple_accel/src/lib.rs:199:17)
  repro_triple_accel_src_lib_rs_199_17_a6659f::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::hamming::hamming_search_naive(&amp;vec![], &amp;vec![]);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/hamming.rs:136:46:
attempt to divide by zero
backtrace:
  triple_accel (triple_accel/src/hamming.rs:136:46)
  triple_accel (triple_accel/src/hamming.rs:71:5)
  repro_triple_accel_src_hamming_rs_136_46_abc6f7::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&amp;vec![],
    &amp;include_bytes!("data_0.bin").to_vec(), 2428045950u32, false,
    EditCosts::new(55u8, 52u8, 134u8, None));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::_mm256_adds_epu32 (src/jewel.rs:849)
  triple_accel::jewel::{impl#2}::adds_mut (src/jewel.rs:90)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:1059)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:790)
  repro_timeout_in_triple_accel_levenshtein_levenshtein_simd_k_with__850faa::main (src/main.rs:4)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::alloc_str(2147483648usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2147483648 bytes failed
backtrace:
  triple_accel::alloc_str (triple_accel/src/lib.rs:199)
  repro_out_of_memory_in_triple_accel_alloc_str_3d25a5::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec = triple_accel::alloc_str(65536usize);
let _ = triple_accel::levenshtein::levenshtein_simd_k(&amp;vec, &amp;vec![],
    1000000000u32);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::_mm256_adds_epu32 (src/jewel.rs:844)
  triple_accel::jewel::{impl#2}::adds_mut (src/jewel.rs:90)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:1108)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:790)
  triple_accel::levenshtein::levenshtein_simd_k (src/levenshtein.rs:678)
  repro_timeout_in_triple_accel_levenshtein_levenshtein_simd_k_267b39::main (src/main.rs:3)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein_search(&amp;include_bytes!("data_0.bin").to_vec(),
    &amp;include_bytes!("data_1.bin").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::triple_min_length (src/jewel.rs:1131)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32::{closure#0} (src/levenshtein.rs:1754)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32 (src/levenshtein.rs:1798)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (src/levenshtein.rs:1568)
  triple_accel::levenshtein::levenshtein_search_simd (src/levenshtein.rs:1478)
  triple_accel::levenshtein::levenshtein_search (src/levenshtein.rs:1877)
  repro_timeout_in_triple_accel_levenshtein_search_1e5fc8::main (src/main.rs:2)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::rdamerau_exp(&amp;vec![],
    &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::adds_mut (src/jewel.rs:90)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:920)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:644)
  triple_accel::levenshtein::rdamerau_exp (src/levenshtein.rs:1194)
  repro_timeout_in_triple_accel_rdamerau_exp_8618c3::main (src/main.rs:2)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein(&amp;vec![],
    &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::min_mut (src/jewel.rs:90)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:872)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:644)
  triple_accel::levenshtein::levenshtein_simd_k (src/levenshtein.rs:574)
  triple_accel::levenshtein::levenshtein (src/levenshtein.rs:1108)
  repro_timeout_in_triple_accel_levenshtein_4c0b3d::main (src/main.rs:2)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let vec = triple_accel::alloc_str(65536usize);
let _ = triple_accel::rdamerau(&amp;vec, &amp;vec![]);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::shift_right_1_mut (src/jewel.rs:1030)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:851)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:644)
  triple_accel::levenshtein::rdamerau (src/levenshtein.rs:1130)
  repro_timeout_in_triple_accel_rdamerau_2107c3::main (src/main.rs:3)
  (2 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein_exp(&amp;vec![],
    &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::shift_right_1_mut (src/jewel.rs:1030)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx8x32 (src/levenshtein.rs:851)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (src/levenshtein.rs:644)
  triple_accel::levenshtein::levenshtein_simd_k (src/levenshtein.rs:574)
  triple_accel::levenshtein::levenshtein_exp (src/levenshtein.rs:1160)
  repro_timeout_in_triple_accel_levenshtein_exp_379a7f::main (src/main.rs:2)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(
    &amp;include_bytes!("data_0.bin").to_vec(),
    &amp;include_bytes!("data_1.bin").to_vec(), 255u32, SearchType::Best,
    RDAMERAU_COSTS, true);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#0}::cmpeq (src/jewel.rs:77)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx16x16::{closure#0} (src/levenshtein.rs:1699)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx16x16 (src/levenshtein.rs:1798)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (src/levenshtein.rs:1566)
  repro_timeout_in_triple_accel_levenshtein_levenshtein_search_simd__b4d2b1::main (src/main.rs:5)
  (3 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_naive_with_opts(
    &amp;include_bytes!("data_0.bin").to_vec(),
    &amp;include_bytes!("image_1.webp").to_vec(), 4294967294u32, SearchType::Best,
    RDAMERAU_COSTS, false);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::levenshtein::levenshtein_search_naive_with_opts::{closure#1} (src/levenshtein.rs:1384)
  triple_accel::levenshtein::levenshtein_search_naive_with_opts (src/levenshtein.rs:1427)
  repro_timeout_in_triple_accel_levenshtein_levenshtein_search_naive_116cdc::main (src/main.rs:5)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_simd(&amp;include_bytes!(
    "data_0.bin").to_vec(), &amp;include_bytes!("data_1.bin").to_vec());</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  triple_accel::jewel::{impl#2}::double_min_length (src/jewel.rs:1154)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32::{closure#0} (src/levenshtein.rs:1724)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32 (src/levenshtein.rs:1798)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (src/levenshtein.rs:1568)
  triple_accel::levenshtein::levenshtein_search_simd (src/levenshtein.rs:1478)
  repro_timeout_in_triple_accel_levenshtein_levenshtein_search_simd_4d9a77::main (src/main.rs:2)
  (1 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EditCosts::new(1u8, 1u8, 0u8, Some(0u8));</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:48:13:
assertion failed: cost &gt; 0
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::new (triple_accel/src/levenshtein.rs:48:13)
  repro_triple_accel_src_levenshtein_rs_48_13_3b85a4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EditCosts::new(0u8, 0u8, 0u8, None);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:44:9:
assertion failed: mismatch_cost &gt; 0
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::new (triple_accel/src/levenshtein.rs:44:9)
  repro_triple_accel_src_levenshtein_rs_44_9_b50dc8::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EditCosts::new(100u8, 1u8, 0u8, Some(10u8));</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:51:13:
assertion failed: (cost &gt;&gt; 1) &lt; gap_cost
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::new (triple_accel/src/levenshtein.rs:51:13)
  repro_triple_accel_src_levenshtein_rs_51_13_2f47fc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EditCosts::new(1u8, 1u8, 0u8, Some(100u8));</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:50:13:
assertion failed: (cost &gt;&gt; 1) &lt; mismatch_cost
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::new (triple_accel/src/levenshtein.rs:50:13)
  repro_triple_accel_src_levenshtein_rs_50_13_b1645e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = EditCosts::new(1u8, 0u8, 0u8, None);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:45:9:
assertion failed: gap_cost &gt; 0
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::new (triple_accel/src/levenshtein.rs:45:9)
  repro_triple_accel_src_levenshtein_rs_45_9_8ed66f::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(
    &amp;vec![82u8], &amp;vec![], 0u32, SearchType::Best, EditCosts::new(1u8, 1u8,
    255u8, Some(1u8)), false);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:69:29:
attempt to add with overflow
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::check_search (triple_accel/src/levenshtein.rs:69:29)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (triple_accel/src/levenshtein.rs:1965:11)
  repro_triple_accel_src_levenshtein_rs_69_29_dfcf6a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&amp;vec![0u8,
    0u8], &amp;vec![], u32::MAX, SearchType::Best, RDAMERAU_COSTS, true);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:2465:1:
attempt to add with overflow
backtrace:
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32::{closure#0} (triple_accel/src/levenshtein.rs:2337:29)
  triple_accel::levenshtein::levenshtein_search_simd_core_avx_nx8x32 (triple_accel/src/levenshtein.rs:2426:26)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (triple_accel/src/levenshtein.rs:2047:21)
  repro_triple_accel_src_levenshtein_rs_2465_1_eb8e76::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let edit_costs = RDAMERAU_COSTS;
let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&amp;include_bytes!(
    "data_0.bin").to_vec(), &amp;vec![255u8], 1000000000u32, true, edit_costs);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:1315:1:
attempt to subtract with overflow
backtrace:
  triple_accel::levenshtein::traceback_avx_nx16x16 (triple_accel/src/levenshtein.rs:1265:25)
  triple_accel::levenshtein::levenshtein_simd_core_avx_nx16x16 (triple_accel/src/levenshtein.rs:1185:22)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (triple_accel/src/levenshtein.rs:788:33)
  repro_triple_accel_src_levenshtein_rs_1315_1_97e1a8::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&amp;vec![128u8],
    &amp;include_bytes!("data_0.bin").to_vec(), 1000000000u32, true,
    RDAMERAU_COSTS);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:1308:1:
attempt to subtract with overflow
backtrace:
  triple_accel::levenshtein::traceback_avx_8x32x8 (triple_accel/src/levenshtein.rs:1265:25)
  triple_accel::levenshtein::levenshtein_simd_core_avx_8x32x8 (triple_accel/src/levenshtein.rs:1185:22)
  triple_accel::levenshtein::levenshtein_simd_k_with_opts (triple_accel/src/levenshtein.rs:786:33)
  repro_triple_accel_src_levenshtein_rs_1308_1_02ee03::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(
    &amp;vec![66u8], &amp;vec![], 0u32, SearchType::Best, EditCosts::new(100u8, 63u8,
    1u8, Some(100u8)), false);</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:69:13:
assertion failed: cost &lt;= self.start_gap_cost + self.gap_cost
backtrace:
  &lt;triple_accel::levenshtein::EditCosts&gt;::check_search (triple_accel/src/levenshtein.rs:69:13)
  triple_accel::levenshtein::levenshtein_search_simd_with_opts (triple_accel/src/levenshtein.rs:1965:11)
  repro_triple_accel_src_levenshtein_rs_69_13_25f6f7::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = triple_accel::rdamerau(&amp;vec![], &amp;include_bytes!("data_0.bin").to_vec());</pre></td><td valign="top"><pre>thread 'main' panicked at triple_accel/src/levenshtein.rs:1421:10:
called `Option::unwrap()` on a `None` value
backtrace:
  triple_accel::levenshtein::rdamerau (triple_accel/src/levenshtein.rs:1421:10)
  repro_triple_accel_src_levenshtein_rs_1421_10_f2133d::main (src/main.rs:2:13)</pre></td></tr>
</table>
