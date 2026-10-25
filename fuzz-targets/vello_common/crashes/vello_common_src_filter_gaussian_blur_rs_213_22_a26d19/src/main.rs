use vello_common::filter::gaussian_blur::DecimationSizer;

fn main() {
    let mut decimation_sizer = DecimationSizer::new(u16::MAX, 0u16);
    let _ = decimation_sizer.downscale();
    let _ = decimation_sizer.upscale();
}
