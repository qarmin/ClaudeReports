use gpu_descriptor_types::DescriptorTotalCount;

fn main() {
    let descriptor_total_count = DescriptorTotalCount { sampler: 0u32, combined_image_sampler: 0u32, sampled_image: 0u32, storage_image: 0u32, uniform_texel_buffer: 0u32, storage_texel_buffer: 0u32, uniform_buffer: 0u32, storage_buffer: 0u32, uniform_buffer_dynamic: 0u32, storage_buffer_dynamic: 0u32, input_attachment: 0u32, acceleration_structure: 0u32, inline_uniform_block_bytes: 1u32, inline_uniform_block_bindings: u32::MAX };
    let _ = descriptor_total_count.total();
}
