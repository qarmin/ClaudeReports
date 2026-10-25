use y4m::Colorspace;
use y4m::EncoderBuilder;
use y4m::Ratio;
use y4m::VendorExtensionString;

fn main() {
    let encoder_builder = EncoderBuilder::new(0usize, usize::MAX, Ratio { num: 0usize, den: 1usize });
    let encoder_builder2 = encoder_builder.with_colorspace(Colorspace::C422p10);
    let encoder_builder3 = encoder_builder2.append_vendor_extension(VendorExtensionString::new(vec![]).unwrap());
    let _ = encoder_builder3.write_header::<Vec<u8>>(vec![]);
}
