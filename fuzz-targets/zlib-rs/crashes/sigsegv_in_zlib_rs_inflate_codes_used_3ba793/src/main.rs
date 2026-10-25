use zlib_rs::InflateConfig;
use zlib_rs::inflate::InflateStream;

fn main() {
    let mut inflate_stream = InflateStream::new(InflateConfig::default());
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
    let _ = zlib_rs::inflate::codes_used(&inflate_stream);
}
