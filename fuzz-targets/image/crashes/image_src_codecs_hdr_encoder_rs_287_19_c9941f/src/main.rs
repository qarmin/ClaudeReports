fn main() {
    let hdr_encoder = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(vec![]);
    let _ = hdr_encoder.encode(&[<image::Rgb<f32> as ::core::convert::From<[f32; 3]>>::from([0.0f32, 0.0f32, f32::INFINITY])], 0usize, 0usize);
}
