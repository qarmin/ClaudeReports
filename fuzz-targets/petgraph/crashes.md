# petgraph - crashes

Source: https://github.com/petgraph/petgraph @ b3eaf3a73f3ac7347b17bec3ba009f01eb8d2495

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = petgraph::graph6::from_graph6_representation::&lt;usize&gt;("(".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at petgraph/src/graph6/graph6_decoder.rs:50:18:
attempt to subtract with overflow
backtrace:
  petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0} (petgraph/src/graph6/graph6_decoder.rs:50:18)
  &lt;&amp;mut petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes::{closure#0} as core::ops::function::FnOnce&lt;(char,)&gt;&gt;::call_once (core/src/ops/function.rs:310:21)
  petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes (petgraph/src/graph6/graph6_decoder.rs:51:10)
  petgraph::graph6::graph6_decoder::from_graph6_representation::&lt;usize&gt; (petgraph/src/graph6/graph6_decoder.rs:33:9)
  repro_petgraph_src_graph6_graph6_decoder_rs_50_18_69ed29::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = petgraph::graph6::from_graph6_representation::&lt;usize&gt;("".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at petgraph/src/graph6/graph6_decoder.rs:56:37:
called `Option::unwrap()` on a `None` value
backtrace:
  petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes (petgraph/src/graph6/graph6_decoder.rs:56:37)
  petgraph::graph6::graph6_decoder::from_graph6_representation::&lt;usize&gt; (petgraph/src/graph6/graph6_decoder.rs:33:9)
  repro_petgraph_src_graph6_graph6_decoder_rs_56_37_1b8b80::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = petgraph::graph6::from_graph6_representation::&lt;u32&gt;("a".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at petgraph/src/graph6/graph6_decoder.rs:106:46:
index out of bounds: the len is 0 but the index is 0
backtrace:
  &lt;usize as core::slice::index::SliceIndex&lt;[u8]&gt;&gt;::index (core/src/slice/index.rs:238:10)
  &lt;[u8] as core::ops::index::Index&lt;usize&gt;&gt;::index (core/src/slice/index.rs:19:15)
  petgraph::graph6::graph6_decoder::get_edges::&lt;u32&gt; (petgraph/src/graph6/graph6_decoder.rs:106:46)
  petgraph::graph6::graph6_decoder::from_graph6_representation::&lt;u32&gt; (petgraph/src/graph6/graph6_decoder.rs:39:17)
  repro_petgraph_src_graph6_graph6_decoder_rs_106_46_00bc3b::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = petgraph::graph6::from_graph6_representation::&lt;u16&gt;("~".to_string());</pre></td><td valign="top"><pre>thread 'main' panicked at petgraph/src/graph6/graph6_decoder.rs:58:45:
range end index 3 out of range for slice of length 1
backtrace:
  &lt;[usize] as core::ops::index::Index&lt;core::ops::range::RangeInclusive&lt;usize&gt;&gt;&gt;::index (core/src/slice/index.rs:19:15)
  petgraph::graph6::graph6_decoder::get_order_bytes_and_adj_matrix_bytes (petgraph/src/graph6/graph6_decoder.rs:58:45)
  petgraph::graph6::graph6_decoder::from_graph6_representation::&lt;u16&gt; (petgraph/src/graph6/graph6_decoder.rs:33:9)
  repro_petgraph_src_graph6_graph6_decoder_rs_58_45_f6863f::main (src/main.rs:2:13)</pre></td></tr>
</table>
