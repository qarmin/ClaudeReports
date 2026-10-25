use zune_inflate::DeflateEncoder;

fn main() {
    let vec = vec![];
    let mut deflate_encoder = DeflateEncoder::new(&vec);
    let _ = deflate_encoder.encode_zlib();
    deflate_encoder.encode_deflate();
}
