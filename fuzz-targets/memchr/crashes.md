# memchr - crashes

Source: https://github.com/BurntSushi/memchr @ bd6068c30e9074a90c285e47912fa0b047d07597

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Finder::with_pair(&amp;vec![97u8], Pair::new(&amp;vec![89u8, 32u8]).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/generic/packedpair.rs:59:27:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;memchr::arch::generic::packedpair::Finder&lt;core::core_arch::x86::__m128i&gt;&gt;::new (memchr/src/arch/generic/packedpair.rs:59:27)
  &lt;memchr::arch::x86_64::avx2::packedpair::Finder&gt;::with_pair_impl (memchr/src/arch/x86_64/avx2/packedpair.rs:69:20)
  &lt;memchr::arch::x86_64::avx2::packedpair::Finder&gt;::with_pair (memchr/src/arch/x86_64/avx2/packedpair.rs:54:27)
  repro_memchr_src_arch_generic_packedpair_rs_59_27_a9a8ff::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let finder = Finder::new(&amp;vec![164u8, 164u8, 0u8]);
let _ = finder.find(&amp;vec![0u8], &amp;vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/twoway.rs:260:24:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;memchr::arch::all::twoway::Finder&gt;::find_large_imp (memchr/src/arch/all/twoway.rs:260:24)
  &lt;memchr::arch::all::twoway::Finder&gt;::find_with_prefilter (memchr/src/arch/all/twoway.rs:162:22)
  &lt;memchr::arch::all::twoway::Finder&gt;::find (memchr/src/arch/all/twoway.rs:138:14)
  repro_memchr_src_arch_all_twoway_rs_260_24_dcad97::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Finder::with_pair(&amp;vec![0u8], Pair::new(&amp;vec![0u8, 0u8]).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/packedpair/mod.rs:58:21:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;memchr::arch::all::packedpair::Finder&gt;::with_pair (memchr/src/arch/all/packedpair/mod.rs:58:21)
  repro_memchr_src_arch_all_packedpair_mod_rs_58_21_0867b9::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Finder::with_pair(&amp;vec![], Pair::new(&amp;vec![110u8, 79u8]).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/generic/packedpair.rs:58:27:
index out of bounds: the len is 0 but the index is 1
backtrace:
  &lt;memchr::arch::generic::packedpair::Finder&lt;core::core_arch::x86::__m128i&gt;&gt;::new (memchr/src/arch/generic/packedpair.rs:58:27)
  &lt;memchr::arch::x86_64::sse2::packedpair::Finder&gt;::with_pair_impl (memchr/src/arch/x86_64/sse2/packedpair.rs:66:22)
  &lt;memchr::arch::x86_64::sse2::packedpair::Finder&gt;::with_pair (memchr/src/arch/x86_64/sse2/packedpair.rs:51:27)
  repro_memchr_src_arch_generic_packedpair_rs_58_27_127b21::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let finder_rev = FinderRev::new(&amp;vec![86u8, 86u8, 93u8]);
let _ = finder_rev.rfind(&amp;vec![93u8], &amp;vec![0u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/twoway.rs:377:28:
index out of bounds: the len is 1 but the index is 1
backtrace:
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind_large_imp (memchr/src/arch/all/twoway.rs:377:28)
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind (memchr/src/arch/all/twoway.rs:311:22)
  repro_memchr_src_arch_all_twoway_rs_377_28_0905fa::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Finder::with_pair(&amp;vec![], Pair::with_indices(&amp;vec![80u8, 78u8], 0u8,
    1u8).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/packedpair/mod.rs:57:21:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;memchr::arch::all::packedpair::Finder&gt;::with_pair (memchr/src/arch/all/packedpair/mod.rs:57:21)
  repro_memchr_src_arch_all_packedpair_mod_rs_57_21_9604e4::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let finder_rev = FinderRev::new(&amp;vec![0u8, 0u8]);
let _ = finder_rev.rfind(&amp;vec![0u8], &amp;vec![109u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/twoway.rs:341:17:
attempt to subtract with overflow
backtrace:
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind_small_imp (memchr/src/arch/all/twoway.rs:341:17)
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind (memchr/src/arch/all/twoway.rs:308:22)
  repro_memchr_src_arch_all_twoway_rs_341_17_749707::main (src/main.rs:5:24)</pre></td></tr>
<tr><td valign="top"><pre>let finder_rev = FinderRev::new(&amp;include_bytes!("data_0.bin").to_vec());
let _ = finder_rev.rfind(&amp;include_bytes!("data_1.bin").to_vec(), &amp;vec![101u8,
    115u8, 93u8, 10u8, 97u8, 108u8, 108u8, 111u8, 99u8, 32u8, 61u8, 32u8, 91u8,
    93u8, 10u8, 100u8, 101u8, 102u8, 97u8, 117u8, 108u8, 116u8, 32u8, 61u8,
    32u8, 91u8, 34u8, 45u8]);</pre></td><td valign="top"><pre>thread 'main' panicked at memchr/src/arch/all/twoway.rs:390:17:
attempt to subtract with overflow
backtrace:
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind_large_imp (memchr/src/arch/all/twoway.rs:390:17)
  &lt;memchr::arch::all::twoway::FinderRev&gt;::rfind (memchr/src/arch/all/twoway.rs:311:22)
  repro_memchr_src_arch_all_twoway_rs_390_17_67ae76::main (src/main.rs:5:24)</pre></td></tr>
</table>
