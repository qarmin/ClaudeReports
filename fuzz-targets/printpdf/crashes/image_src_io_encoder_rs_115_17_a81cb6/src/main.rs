use printpdf::image_types::OutputImageFormat;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageData;
use printpdf::image_types::RawImageFormat;

fn main() {
    let raw_image = RawImage { pixels: RawImageData::U8(vec![119u8]), width: 0usize, height: 0usize, data_format: RawImageFormat::RG8, tag: vec![] };
    let _ = raw_image.encode_to_bytes(&[OutputImageFormat::Pnm]);
}
