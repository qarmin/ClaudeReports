# taffy - crashes

Source: https://github.com/DioxusLabs/taffy @ 8bb96a26f91353e951b373b4db2c716e37e70c20

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = AxisStaticPosition::from_alignment(AlignItems::SAFE_SELF_START,
    &lt;taffy::geometry::Line&lt;f32&gt; as taffy::prelude::FromLength&gt;::
    from_length::&lt;f64&gt;(1.0f64), false);</pre></td><td valign="top"><pre>thread 'main' panicked at taffy/src/tree/layout.rs:303:9:
self-start/self-end must be resolved before computing the static position
backtrace:
  &lt;taffy::tree::layout::AxisStaticPosition&gt;::from_alignment (taffy/src/tree/layout.rs:303:9)
  repro_taffy_src_tree_layout_rs_303_9_f5e44a::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = taffy::compute_block_align_content_offset(AlignContent::default(),
    0.0f32);</pre></td><td valign="top"><pre>thread 'main' panicked at taffy/src/compute/common/alignment.rs:49:5:
normal must be resolved before computing content alignment
backtrace:
  taffy::compute::common::alignment::apply_alignment_fallback (taffy/src/compute/common/alignment.rs:49:5)
  taffy::compute::block::compute_block_align_content_offset (taffy/src/compute/block.rs:36:19)
  repro_taffy_src_compute_common_alignment_rs_49_5_38f2fe::main (src/main.rs:4:13)</pre></td></tr>
</table>
