use vello_common::filter::gaussian_blur::DecimationSizer;

fn main() {
    let mut decimation_sizer = DecimationSizer::default();
    let _ = decimation_sizer.upscale();
}
