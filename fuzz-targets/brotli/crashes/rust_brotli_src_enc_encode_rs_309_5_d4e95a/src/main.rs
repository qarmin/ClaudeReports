use brotli::enc::BrotliEncoderParams;
use brotli::enc::metablock::BrotliInitDistanceParams;

fn main() {
    let mut brotli_encoder_params = BrotliEncoderParams::default();
    BrotliInitDistanceParams(&mut brotli_encoder_params, 0u32, u32::MAX);
}
