use zlib_rs::DeflateConfig;
use zlib_rs::deflate::DeflateStream;

fn main() {
    let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
    let _ = zlib_rs::deflate::end(&mut deflate_stream);
    let _ = deflate_stream.pending();
}
