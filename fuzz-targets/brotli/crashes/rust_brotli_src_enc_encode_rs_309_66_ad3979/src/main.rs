use brotli::enc::BrotliEncoderInitParams;
use brotli::enc::metablock::BrotliInitDistanceParams;

fn main() {
    let mut brotli_encoder_params = BrotliEncoderInitParams();
    BrotliInitDistanceParams(&mut brotli_encoder_params, u32::MAX, 0u32);
}
