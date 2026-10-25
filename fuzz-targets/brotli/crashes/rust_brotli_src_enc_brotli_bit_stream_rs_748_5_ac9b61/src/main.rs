use brotli::enc::BrotliEncoderParams;
use brotli::enc::brotli_bit_stream::BrotliWriteMetadataMetaBlock;

fn main() {
    BrotliWriteMetadataMetaBlock(&BrotliEncoderParams::default(), &mut 0usize, &mut vec![84u8]);
}
