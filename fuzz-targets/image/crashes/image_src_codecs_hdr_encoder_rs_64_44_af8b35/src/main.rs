fn main() {
    let hdr_encoder = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(vec![]);
    let _ = hdr_encoder.encode(&[], 100usize, 1000000000000000000usize);
}
