# ndarray - crashes

Source: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt;&gt;::zeros(
    10000000000000000000usize);</pre></td><td valign="top"><pre>thread 'main' panicked at library/alloc/src/raw_vec/mod.rs:28:5 (called from alloc/src/vec/spec_from_elem.rs:24:31)
capacity overflow
backtrace:
  &lt;usize as alloc::vec::spec_from_elem::SpecFromElem&gt;::from_elem::&lt;alloc::alloc::Global&gt; (alloc/src/vec/spec_from_elem.rs:24:31)
  &lt;ndarray::dimension::dim::Dim&lt;ndarray::dimension::dynindeximpl::IxDynImpl&gt;&gt;::zeros (ndarray/src/dimension/dynindeximpl.rs:271:17)
  repro_ndarray_src_dimension_dynindeximpl_rs_271_17_ee8003::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ndarray::RemoveAxis&gt;::remove_axis(
    &amp;&lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ::core::default::Default&gt;::default(),
    Axis(1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/remove_axis.rs:26:9:
assertion failed: axis.index() &lt; self.ndim()
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as ndarray::dimension::remove_axis::RemoveAxis&gt;::remove_axis (ndarray/src/dimension/remove_axis.rs:26:9)
  repro_ndarray_src_dimension_remove_axis_rs_26_9_55afd4::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 5]&gt; as ::core::ops::Mul&lt;ndarray::
    prelude::Dim&lt;[usize; 5]&gt;&gt;&gt;::mul(Ix5(0usize, 0usize, 100usize, 0usize,
    0usize), Ix5(0usize, 0usize, 10000000000000000000usize, 2usize,
    4294967296usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:190:1:
attempt to multiply with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::MulAssign&lt;&amp;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt;&gt;&gt;::mul_assign (ndarray/src/dimension/dim.rs:183:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::Mul&gt;::mul (ndarray/src/dimension/dim.rs:183:9)
  repro_ndarray_src_dimension_dim_rs_190_1_d303f0::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis(0usize);
let usize_v = axis.index();
let slice = Slice::from(usize_v..0usize);
let _ = slice.step_by(0isize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/slice.rs:76:9:
assertion `left != right` failed: Slice::step_by: step must be nonzero
  left: 0
 right: 0
backtrace:
  &lt;ndarray::slice::Slice&gt;::step_by (ndarray/src/slice.rs:76:9)
  repro_ndarray_src_slice_rs_76_9_0d03ba::main (src/main.rs:8:19)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis(0usize);
let _ = &lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt; as ndarray::RemoveAxis&gt;::
    remove_axis(&amp;IxDyn(&amp;[]), axis);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dynindeximpl.rs:256:9:
assertion failed: axis.index() &lt; self.ndim()
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;ndarray::dimension::dynindeximpl::IxDynImpl&gt; as ndarray::dimension::remove_axis::RemoveAxis&gt;::remove_axis (ndarray/src/dimension/dynindeximpl.rs:256:9)
  repro_ndarray_src_dimension_dynindeximpl_rs_256_9_0dc636::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ::core::ops::Add&lt;usize&gt;&gt;::add(Ix1(
    usize::MAX), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:187:1:
attempt to add with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as core::ops::arith::AddAssign&lt;usize&gt;&gt;::add_assign (ndarray/src/dimension/dim.rs:173:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as core::ops::arith::Add&lt;usize&gt;&gt;::add (ndarray/src/dimension/dim.rs:173:9)
  repro_ndarray_src_dimension_dim_rs_187_1_8bc5cf::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ::core::ops::Sub&lt;usize&gt;&gt;::sub(
    &lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ::core::default::Default&gt;::default(),
    1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:189:1:
attempt to subtract with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as core::ops::arith::SubAssign&lt;usize&gt;&gt;::sub_assign (ndarray/src/dimension/dim.rs:178:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as core::ops::arith::Sub&lt;usize&gt;&gt;::sub (ndarray/src/dimension/dim.rs:178:9)
  repro_ndarray_src_dimension_dim_rs_189_1_ed3613::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis(1usize);
let usize_v = axis.index();
let _ = &lt;ndarray::prelude::Dim&lt;[usize; 5]&gt; as ::core::ops::Add&lt;ndarray::
    prelude::Dim&lt;[usize; 5]&gt;&gt;&gt;::add(Ix5(0usize, 0usize, usize_v, 0usize,
    usize::MAX), Ix5(0usize, 0usize, 0usize, usize_v, 1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:186:1:
attempt to add with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::AddAssign&lt;&amp;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt;&gt;&gt;::add_assign (ndarray/src/dimension/dim.rs:173:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::Add&gt;::add (ndarray/src/dimension/dim.rs:173:9)
  repro_ndarray_src_dimension_dim_rs_186_1_d64d6b::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let axis = Axis(1000000000usize);
let _ = &lt;ndarray::prelude::Dim&lt;[usize; 2]&gt; as ndarray::RemoveAxis&gt;::remove_axis(
    &amp;Ix2(0usize, 0usize), axis);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/remove_axis.rs:37:9:
assertion failed: axis &lt; self.ndim()
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 2]&gt; as ndarray::dimension::remove_axis::RemoveAxis&gt;::remove_axis (ndarray/src/dimension/remove_axis.rs:37:9)
  repro_ndarray_src_dimension_remove_axis_rs_37_9_56ad0d::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt; as ndarray::RemoveAxis&gt;::
    remove_axis(&amp;&lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt;&gt;::zeros(
    1000000000usize), Axis(0usize));</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  ndarray::dimension::dim::Dim&lt;ndarray::dimension::dynindeximpl::IxDynImpl&gt;::zeros (ndarray/src/dimension/dynindeximpl.rs:271)
  repro_out_of_memory_in_ndarray_prelude_dim_ndarray_ixdynimpl_remov_1e2496::main (src/main.rs:4)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt;&gt;::zeros(1000000000usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  ndarray::dimension::dim::Dim&lt;ndarray::dimension::dynindeximpl::IxDynImpl&gt;::zeros (ndarray/src/dimension/dynindeximpl.rs:271)
  repro_out_of_memory_in_ndarray_prelude_dim_ndarray_ixdynimpl_zeros_07eb4b::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt; as ::core::ops::Mul&lt;usize&gt;&gt;::
    mul(&lt;ndarray::prelude::Dim&lt;ndarray::IxDynImpl&gt;&gt;::zeros(1000000000usize),
    0usize);</pre></td><td valign="top"><pre>out of memory: memory allocation of 8000000000 bytes failed
backtrace:
  ndarray::dimension::dim::Dim&lt;ndarray::dimension::dynindeximpl::IxDynImpl&gt;::zeros (ndarray/src/dimension/dynindeximpl.rs:271)
  repro_out_of_memory_in_ndarray_prelude_dim_ndarray_ixdynimpl_mul_u_0575b7::main (src/main.rs:2)
  (18 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let slice = Slice::new(0isize, None, -1000000000000000000isize);
let _ = slice.step_by(10isize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/slice.rs:78:19:
attempt to multiply with overflow
backtrace:
  &lt;ndarray::slice::Slice&gt;::step_by (ndarray/src/slice.rs:78:19)
  repro_ndarray_src_slice_rs_78_19_4251af::main (src/main.rs:5:19)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 6]&gt; as ndarray::Dimension&gt;::size(&amp;Ix6(
    1usize, 1usize, 1usize, usize::MAX, 256usize, 4294967296usize));</pre></td><td valign="top"><pre>thread 'main' panicked at core/src/iter/traits/accum.rs:206:1:
attempt to multiply with overflow
backtrace:
  &lt;usize as core::ops::arith::Mul&gt;::mul (core/src/ops/arith.rs:350:45)
  &lt;usize as core::ops::arith::Mul&lt;&amp;usize&gt;&gt;::mul (core/src/internal_macros.rs:39:17)
  &lt;usize as core::iter::traits::accum::Product&lt;&amp;usize&gt;&gt;::product::&lt;core::slice::iter::Iter&lt;usize&gt;&gt;::{closure#0} (core/src/iter/traits/accum.rs:88:28)
  &lt;usize as core::iter::traits::accum::Product&lt;&amp;usize&gt;&gt;::product::&lt;core::slice::iter::Iter&lt;usize&gt;&gt; (core/src/iter/traits/accum.rs:85:22)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 6]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::size (ndarray/src/dimension/dimension_trait.rs:88:29)
  repro_ndarray_src_dimension_dimension_trait_rs_88_29_fec256::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 5]&gt; as ::core::ops::Sub&lt;ndarray::
    prelude::Dim&lt;[usize; 5]&gt;&gt;&gt;::sub(Ix5(0usize, 0usize, 0usize, 0usize, 0usize),
    Ix5(0usize, 0usize, 0usize, 0usize, 1usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:188:1:
attempt to subtract with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::SubAssign&lt;&amp;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt;&gt;&gt;::sub_assign (ndarray/src/dimension/dim.rs:178:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as core::ops::arith::Sub&gt;::sub (ndarray/src/dimension/dim.rs:178:9)
  repro_ndarray_src_dimension_dim_rs_188_1_7a5378::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let dim = Ix5(0usize, 0usize, 0usize, 1usize, 1usize);
let usize_v = &lt;ndarray::prelude::Dim&lt;[usize; 5]&gt; as ndarray::Dimension&gt;::ndim(
    &amp;dim);
let _ = &lt;ndarray::prelude::Dim&lt;[usize; 3]&gt; as ndarray::RemoveAxis&gt;::remove_axis(
    &amp;Ix3(0usize, 0usize, 0usize), Axis(usize_v));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/remove_axis.rs:68:1:
assertion failed: axis.index() &lt; self.ndim()
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 3]&gt; as ndarray::dimension::remove_axis::RemoveAxis&gt;::remove_axis (ndarray/src/dimension/remove_axis.rs:53:17)
  repro_ndarray_src_dimension_remove_axis_rs_68_1_de4e0f::main (src/main.rs:8:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 4]&gt; as ::core::ops::Mul&lt;usize&gt;&gt;::mul(Ix4(
    0usize, 0usize, 0usize, 10000000000000000000usize), 6usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dim.rs:191:1:
attempt to multiply with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 4]&gt; as core::ops::arith::MulAssign&lt;usize&gt;&gt;::mul_assign (ndarray/src/dimension/dim.rs:183:9)
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 4]&gt; as core::ops::arith::Mul&lt;usize&gt;&gt;::mul (ndarray/src/dimension/dim.rs:183:9)
  repro_ndarray_src_dimension_dim_rs_191_1_b16b86::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = Slice::new(0isize, None, 0isize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/slice.rs:64:9:
assertion `left != right` failed: Slice::new: step must be nonzero
  left: 0
 right: 0
backtrace:
  &lt;ndarray::slice::Slice&gt;::new (ndarray/src/slice.rs:64:9)
  repro_ndarray_src_slice_rs_64_9_8b5190::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 2]&gt; as ndarray::Dimension&gt;::zeros(
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:628:9:
assertion `left == right` failed
  left: 0
 right: 2
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 2]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:628:9)
  repro_ndarray_src_dimension_dimension_trait_rs_628_9_a2d58a::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 4]&gt; as ndarray::Dimension&gt;::zeros(
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:937:1:
assertion `left == right` failed
  left: 0
 right: 4
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 4]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:924:17)
  repro_ndarray_src_dimension_dimension_trait_rs_937_1_b3a42d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 1]&gt; as ndarray::Dimension&gt;::zeros(
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:495:9:
assertion `left == right` failed
  left: 0
 right: 1
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 1]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:495:9)
  repro_ndarray_src_dimension_dimension_trait_rs_495_9_8c88b2::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 5]&gt; as ndarray::Dimension&gt;::zeros(
    1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:940:1:
assertion `left == right` failed
  left: 1
 right: 5
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 5]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:924:17)
  repro_ndarray_src_dimension_dimension_trait_rs_940_1_239774::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 2]&gt; as ndarray::Dimension&gt;::size(&amp;Ix2(
    10000000000000000000usize, 6usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:658:9:
attempt to multiply with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 2]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::size (ndarray/src/dimension/dimension_trait.rs:658:9)
  repro_ndarray_src_dimension_dimension_trait_rs_658_9_b34049::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 6]&gt; as ndarray::Dimension&gt;::zeros(
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:943:1:
assertion `left == right` failed
  left: 0
 right: 6
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 6]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:924:17)
  repro_ndarray_src_dimension_dimension_trait_rs_943_1_39ec6d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 0]&gt; as ndarray::Dimension&gt;::zeros(
    1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:448:9:
assertion `left == right` failed
  left: 1
 right: 0
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 0]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:448:9)
  repro_ndarray_src_dimension_dimension_trait_rs_448_9_565a12::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 3]&gt; as ndarray::Dimension&gt;::zeros(
    0usize);</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:813:9:
assertion `left == right` failed
  left: 0
 right: 3
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 3]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::zeros (ndarray/src/dimension/dimension_trait.rs:813:9)
  repro_ndarray_src_dimension_dimension_trait_rs_813_9_8ab25d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;ndarray::prelude::Dim&lt;[usize; 3]&gt; as ndarray::Dimension&gt;::size(&amp;Ix3(
    10000000000000000000usize, 255usize, 256usize));</pre></td><td valign="top"><pre>thread 'main' panicked at ndarray/src/dimension/dimension_trait.rs:807:9:
attempt to multiply with overflow
backtrace:
  &lt;ndarray::dimension::dim::Dim&lt;[usize; 3]&gt; as ndarray::dimension::dimension_trait::Dimension&gt;::size (ndarray/src/dimension/dimension_trait.rs:807:9)
  repro_ndarray_src_dimension_dimension_trait_rs_807_9_1059d6::main (src/main.rs:4:13)</pre></td></tr>
</table>
