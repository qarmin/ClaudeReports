use miniz_oxide::DataFormat;
use miniz_oxide::MZFlush;
use miniz_oxide::deflate::CompressionLevel;
use miniz_oxide::deflate::core::CompressorOxide;
use miniz_oxide::deflate::core::TDEFLFlush;

fn main() {
    let mut compressor_oxide = CompressorOxide::default();
    let _ = miniz_oxide::deflate::stream::deflate(&mut compressor_oxide, &vec![0u8, 0u8, 0u8, 0u8], &mut vec![0u8], MZFlush::Block);
    compressor_oxide.set_format_and_level(CompressorOxide::with_format_and_level(DataFormat::Zlib, CompressionLevel::BestSpeed).data_format(), 1u8);
    let _ = miniz_oxide::deflate::core::compress(&mut compressor_oxide, &vec![], &mut vec![], TDEFLFlush::NoSync);
}
