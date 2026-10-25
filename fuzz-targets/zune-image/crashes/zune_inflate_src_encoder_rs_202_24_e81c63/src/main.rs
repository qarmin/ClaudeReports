use zune_inflate::DeflateEncoder;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    deflate_encoder.encode_deflate();
    deflate_encoder.encode_deflate();
}
