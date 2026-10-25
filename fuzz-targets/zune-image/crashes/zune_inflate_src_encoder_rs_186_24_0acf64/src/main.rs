use zune_inflate::DeflateEncoder;

fn main() {
    let vec = vec![83u8];
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    let _ = deflate_encoder.encode_zlib();
    deflate_encoder.encode_deflate();
}
