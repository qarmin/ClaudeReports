use image::flat::SampleLayout;

fn main() {
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX, width: 0u32, width_stride: 0usize, height: 0u32, height_stride: 1usize };
    let _ = sample_layout.in_bounds_index(1u8, 0u32, 1u32);
}
