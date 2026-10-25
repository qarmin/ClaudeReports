use image::flat::SampleLayout;

fn main() {
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: 0usize, width: 0u32, width_stride: usize::MAX, height: 1u32, height_stride: 2147483648usize };
    let _ = sample_layout.in_bounds_index(2u8, 4294967294u32, 1212462995u32);
}
