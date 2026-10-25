use image::flat::SampleLayout;

fn main() {
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX, width: 3632196899u32, width_stride: 1usize, height: 13u32, height_stride: 2usize };
    let _ = sample_layout.in_bounds_index(232u8, 16u32, 8u32);
}
