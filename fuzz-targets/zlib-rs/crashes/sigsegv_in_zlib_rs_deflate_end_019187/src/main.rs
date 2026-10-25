use zlib_rs::DeflateConfig;
use zlib_rs::deflate::DeflateStream;

fn main() {
    let mut deflate_stream = DeflateStream::new(DeflateConfig::default());
    let _ = zlib_rs::deflate::end(&mut deflate_stream);
    let _ = zlib_rs::deflate::end(&mut deflate_stream);
}
