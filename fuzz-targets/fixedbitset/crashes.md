# fixedbitset - crashes

Source: https://github.com/petgraph/fixedbitset @ 111fb2552130f6f4c06a4a1a2e0de7ea6e28b922

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = FixedBitSet::with_capacity(usize::MAX);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_with_capacity_cb6bf6::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::with_capacity(usize::MAX);
let _ = fixed_bit_set.zeroes();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_zeroes_b01524::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::default();
let fixed_bit_set2 = FixedBitSet::with_capacity(10000000000000000000usize);
let _ = fixed_bit_set.difference(&amp;fixed_bit_set2);</pre></td><td valign="top"><pre>out of memory: memory allocation of 1250000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_difference_2cd944::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    1000000000000000000usize, vec![]);</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_with_capacity_and_b_162b32::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    0usize, vec![]);
fixed_bit_set.symmetric_difference_with(&amp;FixedBitSet::with_capacity(
    usize::MAX));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_symmetric_differenc_1ee2f7::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::default();
fixed_bit_set.grow(usize::MAX);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::grow_inner (src/lib.rs:169)
  fixedbitset::{impl#2}::grow::do_grow (src/lib.rs:149)
  fixedbitset::FixedBitSet::grow (fixedbitset/src/lib.rs:153)
  repro_out_of_memory_in_fixedbitset_fixedbitset_grow_5ff590::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::new();
let fixed_bit_set2 = FixedBitSet::with_capacity(1000000000000000000usize);
let _ = fixed_bit_set.intersection(&amp;fixed_bit_set2);</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_intersection_c3c300::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = IntoOnes::last_positive_bit_and_unset(&amp;mut 1usize);
let fixed_bit_set = FixedBitSet::new();
let _ = fixed_bit_set.is_superset(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    10000000000000000000usize, vec![65535usize, 14542859564429068131usize,
    13991429659670356655usize, usize_v, usize_v]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1250000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_is_superset_1220a6::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::default();
fixed_bit_set.union_with(&amp;FixedBitSet::with_capacity(1000000000000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_union_with_d300ad::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::default();
fixed_bit_set.difference_with(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    1000000000000000000usize, vec![]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_difference_with_706330::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    usize::MAX, vec![1usize, 256usize, 2147483648usize, 255usize, usize::MAX,
    usize::MAX]);
let fixed_bit_set2 = FixedBitSet::with_capacity(0usize);
let _ = fixed_bit_set.symmetric_difference(&amp;fixed_bit_set2);</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_symmetric_differenc_24dc61::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::with_capacity(usize::MAX);
let _ = fixed_bit_set.ones();</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_ones_492c85::main (src/main.rs:4)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::default();
let _ = fixed_bit_set.is_disjoint(&amp;FixedBitSet::with_capacity(usize::MAX));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_is_disjoint_b848d0::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(0usize,
    vec![]);
let _ = fixed_bit_set.symmetric_difference_count(&amp;FixedBitSet::with_capacity(
    usize::MAX));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_symmetric_differenc_4500f0::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::default();
fixed_bit_set.grow_and_insert(1000000000000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000016 bytes failed
backtrace:
  fixedbitset::FixedBitSet::grow_inner (src/lib.rs:169)
  fixedbitset::{impl#2}::grow::do_grow (src/lib.rs:149)
  fixedbitset::FixedBitSet::grow (fixedbitset/src/lib.rs:153)
  fixedbitset::FixedBitSet::grow_and_insert (fixedbitset/src/lib.rs:240)
  repro_out_of_memory_in_fixedbitset_fixedbitset_grow_and_insert_f18a73::main (src/main.rs:5)
  (20 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::new();
let _ = fixed_bit_set.intersection_count(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(usize::MAX,
    vec![11usize, 2usize]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_intersection_count_36e059::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let usize_v = IntoOnes::last_positive_bit_and_unset(&amp;mut 1usize);
let fixed_bit_set = FixedBitSet::new();
let _ = fixed_bit_set.is_subset(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    10000000000000000000usize, vec![2147483648usize, 4294967296usize, 13usize,
    usize::MAX, 2usize, usize_v]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1250000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_is_subset_e6457a::main (src/main.rs:7)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(0usize,
    vec![]);
let _ = fixed_bit_set.union_count(&amp;FixedBitSet::with_capacity(
    1000000000000000000usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 125000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  repro_out_of_memory_in_fixedbitset_fixedbitset_union_count_2332f4::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let mut fixed_bit_set = FixedBitSet::with_capacity(0usize);
fixed_bit_set.intersect_with(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(usize::MAX, vec![]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 2305843009213693952 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_intersect_with_206606::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_bit_set = FixedBitSet::new();
let _ = fixed_bit_set.difference_count(
    &amp;FixedBitSet::with_capacity_and_blocks::&lt;Vec&lt;usize&gt;&gt;(
    10000000000000000000usize, vec![]));</pre></td><td valign="top"><pre>out of memory: memory allocation of 1250000000000000000 bytes failed
backtrace:
  fixedbitset::FixedBitSet::with_capacity (src/lib.rs:107)
  fixedbitset::FixedBitSet::with_capacity_and_blocks&lt;alloc::vec::Vec&lt;usize, alloc::alloc::Global&gt;&gt; (fixedbitset/src/lib.rs:134)
  repro_out_of_memory_in_fixedbitset_fixedbitset_difference_count_1d5c2a::main (src/main.rs:5)
  (19 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = IntoOnes::last_positive_bit_and_unset(&amp;mut 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at fixedbitset/src/lib.rs:1537:15:
attempt to subtract with overflow
backtrace:
  &lt;fixedbitset::IntoOnes&gt;::last_positive_bit_and_unset (fixedbitset/src/lib.rs:1537:15)
  repro_fixedbitset_src_lib_rs_1537_15_436344::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Ones::last_positive_bit_and_unset(&amp;mut 0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at fixedbitset/src/lib.rs:1315:15:
attempt to subtract with overflow
backtrace:
  &lt;fixedbitset::Ones&gt;::last_positive_bit_and_unset (fixedbitset/src/lib.rs:1315:15)
  repro_fixedbitset_src_lib_rs_1315_15_166f7a::main (src/main.rs:4:13)</pre></td></tr>
</table>
