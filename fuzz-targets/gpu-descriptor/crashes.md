# gpu-descriptor - crashes

Source: https://github.com/zakarumych/gpu-descriptor @ 21e6b7526642ebf9abd48548434ba4599bf2190f

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let descriptor_total_count = DescriptorTotalCount { sampler: 0u32,
    combined_image_sampler: 0u32, sampled_image: 0u32, storage_image: 0u32,
    uniform_texel_buffer: 0u32, storage_texel_buffer: 0u32,
    uniform_buffer: 0u32, storage_buffer: 0u32, uniform_buffer_dynamic: 0u32,
    storage_buffer_dynamic: 0u32, input_attachment: 0u32,
    acceleration_structure: 0u32, inline_uniform_block_bytes: 1u32,
    inline_uniform_block_bindings: u32::MAX };
let _ = descriptor_total_count.total();</pre></td><td valign="top"><pre>thread 'main' panicked at types/src/types.rs:38:9:
attempt to add with overflow
backtrace:
  &lt;gpu_descriptor_types::types::DescriptorTotalCount&gt;::total (types/src/types.rs:38:9)
  repro_types_src_types_rs_38_9_91a66d::main (src/main.rs:5:36)</pre></td></tr>
</table>
