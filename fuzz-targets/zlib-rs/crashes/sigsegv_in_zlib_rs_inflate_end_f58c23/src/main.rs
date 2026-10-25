use zlib_rs::InflateConfig;
use zlib_rs::inflate::InflateStream;

fn main() {
    let inflate_config = InflateConfig::default();
    let mut inflate_stream = InflateStream::new(inflate_config);
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
}
