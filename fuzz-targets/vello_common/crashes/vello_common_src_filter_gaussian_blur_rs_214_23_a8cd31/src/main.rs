use vello_common::filter::gaussian_blur::DecimationSizer;

fn main() {
    let mut decimation_sizer = DecimationSizer::new(1u16, u16::MAX);
    let _ = decimation_sizer.downscale();
    let _ = decimation_sizer.upscale();
}
