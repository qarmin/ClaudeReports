# nalgebra - crashes

Source: https://github.com/dimforge/nalgebra @ b2466e6c4070b06240d929895473c9141fe24720

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;nalgebra::base::dimension::Dyn as ::core::ops::Add&lt;usize&gt;&gt;::add(Dyn(
    1usize), usize::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at nalgebra/src/base/dimension.rs:115:14:
attempt to add with overflow
backtrace:
  &lt;nalgebra::base::dimension::Dyn as core::ops::arith::Add&lt;usize&gt;&gt;::add (nalgebra/src/base/dimension.rs:115:14)
  repro_nalgebra_src_base_dimension_rs_115_14_41e789::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;nalgebra::base::dimension::Dyn as ::core::ops::Sub&lt;usize&gt;&gt;::sub(
    &lt;nalgebra::base::dimension::Dyn as nalgebra::base::dimension::Dim&gt;::
    from_usize(0usize), 1usize);</pre></td><td valign="top"><pre>thread 'main' panicked at nalgebra/src/base/dimension.rs:124:14:
attempt to subtract with overflow
backtrace:
  &lt;nalgebra::base::dimension::Dyn as core::ops::arith::Sub&lt;usize&gt;&gt;::sub (nalgebra/src/base/dimension.rs:124:14)
  repro_nalgebra_src_base_dimension_rs_124_14_bdb74c::main (src/main.rs:2:13)</pre></td></tr>
</table>
