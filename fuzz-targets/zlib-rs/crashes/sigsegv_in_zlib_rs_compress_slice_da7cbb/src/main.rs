use zlib_rs::DeflateConfig;
use zlib_rs::InflateConfig;
use zlib_rs::inflate::InflateStream;

fn main() {
    let mut inflate_stream = InflateStream::new(InflateConfig::default());
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
    let _ = zlib_rs::compress_slice(&mut vec![0u8], &vec![], DeflateConfig::default());
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
}
