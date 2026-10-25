use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let _ = <zune_image::image::Image as ::core::ops::Sub<zune_image::image::Image>>::sub(Image::from_u8(&vec![], 1usize, usize::MAX, ColorSpace::YCCK), <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>> as zune_image::traits::DecoderTrait>::decode(&mut <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(include_bytes!("png.json").to_vec())).unwrap()).unwrap());
}
