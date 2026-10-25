use zlib_rs::DeflateConfig;
use zlib_rs::deflate::DeflateStream;

fn main() {
    let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
    let _ = zlib_rs::deflate::prime(&mut deflate_stream, 10000i32, 0i32);
}
