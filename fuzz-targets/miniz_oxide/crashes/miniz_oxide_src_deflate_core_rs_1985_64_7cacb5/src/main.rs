use miniz_oxide::DataFormat;
use miniz_oxide::MZFlush;
use miniz_oxide::deflate::CompressionLevel;
use miniz_oxide::deflate::core::CompressionStrategy;
use miniz_oxide::deflate::core::CompressorOxide;
use miniz_oxide::deflate::core::TDEFLFlush;

fn main() {
    let mut compressor_oxide = CompressorOxide::with_params(CompressorOxide::with_params(DataFormat::Raw, 0u8, CompressionStrategy::Filtered, 0u8).data_format(), 1u8, CompressionStrategy::Filtered, 0u8);
    let _ = miniz_oxide::deflate::stream::deflate(&mut compressor_oxide, &include_bytes!("data_0.bin").to_vec(), &mut vec![0u8], MZFlush::Block);
    compressor_oxide.set_compression_level(CompressionLevel::UberCompression);
    let _ = miniz_oxide::deflate::core::compress(&mut compressor_oxide, &vec![66u8], &mut vec![], TDEFLFlush::None);
}
