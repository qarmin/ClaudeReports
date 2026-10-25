use zune_inflate::DeflateEncoder;

fn main() {
    let vec = vec![66u8];
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    deflate_encoder.encode_deflate();
    let _ = deflate_encoder.encode_zlib();
}
