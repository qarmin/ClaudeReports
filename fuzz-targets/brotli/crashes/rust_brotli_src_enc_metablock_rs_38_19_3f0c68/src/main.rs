use brotli::enc::BrotliEncoderInitParams;
use brotli::enc::metablock::BrotliInitDistanceParams;

fn main() {
    let mut brotli_encoder_params = BrotliEncoderInitParams();
    BrotliInitDistanceParams(&mut brotli_encoder_params, 10u32, 0u32);
}
